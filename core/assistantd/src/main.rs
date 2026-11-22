use std::{
    env,
    error::Error,
    fs::{self, DirBuilder},
    io,
    os::unix::{
        fs::{DirBuilderExt, FileTypeExt, MetadataExt, PermissionsExt},
        net::UnixStream as StdUnixStream,
    },
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use nix::unistd::Uid;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    sync::Semaphore,
    task::JoinSet,
    time::timeout,
};

const PROTOCOL_VERSION: u16 = 1;
const MAX_FRAME_BYTES: usize = 4 * 1024;
const MAX_CONNECTIONS: usize = 32;
const CONNECTION_DEADLINE: Duration = Duration::from_secs(5);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Request {
    version: u16,
    id: String,
    operation: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    version: u16,
    id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ApiError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiError {
    code: String,
    message: String,
}

impl Response {
    fn success(id: String, result: Value) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            id,
            ok: true,
            result: Some(result),
            error: None,
        }
    }

    fn failure(id: String, code: &str, message: &str) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            id,
            ok: false,
            result: None,
            error: Some(ApiError {
                code: code.to_owned(),
                message: message.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    HealthRead,
    StatusRead,
}

impl Operation {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "core.health.read" => Some(Self::HealthRead),
            "core.status.read" => Some(Self::StatusRead),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::HealthRead => "core.health.read",
            Self::StatusRead => "core.status.read",
        }
    }
}

#[derive(Clone, Copy)]
struct Principal {
    uid: u32,
}

struct AuthorizationRequest {
    principal: Principal,
    operation: Option<Operation>,
    resource: &'static str,
}

struct PermissionBroker {
    owner_uid: u32,
}

impl PermissionBroker {
    fn authorize(&self, request: &AuthorizationRequest) -> bool {
        if request.principal.uid != self.owner_uid || request.resource != "core" {
            return false;
        }

        matches!(
            request.operation,
            Some(Operation::HealthRead | Operation::StatusRead)
        )
    }
}

struct ServerState {
    owner_uid: u32,
    started_at: Instant,
    broker: PermissionBroker,
}

struct SocketCleanup {
    path: PathBuf,
    device: u64,
    inode: u64,
    owner_uid: u32,
}

impl Drop for SocketCleanup {
    fn drop(&mut self) {
        if let Ok(metadata) = fs::symlink_metadata(&self.path) {
            let same_socket = metadata.file_type().is_socket()
                && metadata.uid() == self.owner_uid
                && metadata.dev() == self.device
                && metadata.ino() == self.inode;

            if same_socket {
                let _ = fs::remove_file(&self.path);
            }
        }
    }
}

fn io_error(kind: io::ErrorKind, message: impl Into<String>) -> io::Error {
    io::Error::new(kind, message.into())
}

fn runtime_directory() -> io::Result<PathBuf> {
    let configured = env::var_os("XDG_RUNTIME_DIR")
        .ok_or_else(|| io_error(io::ErrorKind::NotFound, "XDG_RUNTIME_DIR is not set"))?;
    let base = PathBuf::from(configured);
    let metadata = fs::symlink_metadata(&base)?;
    let current_uid = Uid::effective().as_raw();

    if !metadata.file_type().is_dir()
        || metadata.uid() != current_uid
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(io_error(
            io::ErrorKind::PermissionDenied,
            "XDG_RUNTIME_DIR must be a real directory owned by the current user with no group/other permissions",
        ));
    }

    Ok(base.join("assistant-v2"))
}

fn ensure_runtime_directory() -> io::Result<PathBuf> {
    let directory = runtime_directory()?;

    match fs::symlink_metadata(&directory) {
        Ok(metadata) => validate_private_directory(&directory, &metadata)?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let mut builder = DirBuilder::new();
            builder.mode(0o700);
            builder.create(&directory)?;
            let metadata = fs::symlink_metadata(&directory)?;
            validate_private_directory(&directory, &metadata)?;
        }
        Err(error) => return Err(error),
    }

    Ok(directory)
}

fn validate_private_directory(path: &Path, metadata: &fs::Metadata) -> io::Result<()> {
    if !metadata.file_type().is_dir()
        || metadata.uid() != Uid::effective().as_raw()
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(io_error(
            io::ErrorKind::PermissionDenied,
            format!("runtime directory is not a private, user-owned directory: {}", path.display()),
        ));
    }
    Ok(())
}

fn prepare_socket_path(path: &Path, owner_uid: u32) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if !metadata.file_type().is_socket() || metadata.uid() != owner_uid {
                return Err(io_error(
                    io::ErrorKind::AlreadyExists,
                    "refusing to remove a non-socket or differently owned object at the socket path",
                ));
            }

            match StdUnixStream::connect(path) {
                Ok(_) => {
                    return Err(io_error(
                        io::ErrorKind::AddrInUse,
                        "assistantd already has an active socket",
                    ));
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::ConnectionRefused | io::ErrorKind::NotFound
                    ) =>
                {
                    fs::remove_file(path)?;
                }
                Err(error) => return Err(error),
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

fn validate_request_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

async fn read_frame<R>(reader: &mut R) -> io::Result<Vec<u8>>
where
    R: AsyncRead + Unpin,
{
    let mut frame = Vec::with_capacity(512);
    let mut chunk = [0_u8; 512];

    loop {
        let count = reader.read(&mut chunk).await?;
        if count == 0 {
            return Err(io_error(
                io::ErrorKind::UnexpectedEof,
                "connection closed before a complete frame",
            ));
        }

        if let Some(newline) = chunk[..count].iter().position(|byte| *byte == b'\n') {
            if frame.len() + newline > MAX_FRAME_BYTES {
                return Err(io_error(io::ErrorKind::InvalidData, "frame exceeds size limit"));
            }
            frame.extend_from_slice(&chunk[..newline]);

            if newline + 1 != count {
                return Err(io_error(
                    io::ErrorKind::InvalidData,
                    "only one request frame is accepted per connection",
                ));
            }
            return Ok(frame);
        }

        frame.extend_from_slice(&chunk[..count]);
        if frame.len() > MAX_FRAME_BYTES {
            return Err(io_error(io::ErrorKind::InvalidData, "frame exceeds size limit"));
        }
    }
}

async fn write_response<W>(writer: &mut W, response: &Response) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let mut bytes = serde_json::to_vec(response)
        .map_err(|error| io_error(io::ErrorKind::InvalidData, error.to_string()))?;
    bytes.push(b'\n');
    writer.write_all(&bytes).await
}

fn audit(uid: u32, request_id: &str, operation: &str, decision: &str) {
    eprintln!(
        "{}",
        json!({
            "event": "authorization_decision",
            "uid": uid,
            "request_id": request_id,
            "operation": operation,
            "decision": decision
        })
    );
}

async fn handle_connection(
    mut stream: UnixStream,
    state: Arc<ServerState>,
) -> io::Result<()> {
    let credentials = stream.peer_cred()?;
    let principal = Principal {
        uid: credentials.uid(),
    };

    if principal.uid != state.owner_uid {
        audit(principal.uid, "unknown", "ipc.connect", "deny");
        return Ok(());
    }

    let frame = match read_frame(&mut stream).await {
        Ok(frame) => frame,
        Err(error) => {
            audit(principal.uid, "unknown", "protocol.frame", "deny");
            return Err(error);
        }
    };

    let request: Request = match serde_json::from_slice(&frame) {
        Ok(request) => request,
        Err(_) => {
            let response = Response::failure(
                "invalid".to_owned(),
                "invalid_request",
                "Request JSON is invalid or contains unsupported fields",
            );
            return write_response(&mut stream, &response).await;
        }
    };

    if !validate_request_id(&request.id) {
        let response = Response::failure(
            "invalid".to_owned(),
            "invalid_request_id",
            "Request ID must be 1-64 ASCII letters, digits, hyphens or underscores",
        );
        return write_response(&mut stream, &response).await;
    }

    if request.version != PROTOCOL_VERSION {
        audit(principal.uid, &request.id, &request.operation, "deny");
        let response = Response::failure(
            request.id,
            "unsupported_version",
            "The requested protocol version is not supported",
        );
        return write_response(&mut stream, &response).await;
    }

    let operation = match Operation::parse(&request.operation) {
        Some(operation) => operation,
        None => {
            audit(principal.uid, &request.id, &request.operation, "deny");
            let response = Response::failure(
                request.id,
                "forbidden",
                "The operation is not available to this client",
            );
            return write_response(&mut stream, &response).await;
        }
    };
    let authorization = AuthorizationRequest {
        principal,
        operation: Some(operation),
        resource: "core",
    };

    if !state.broker.authorize(&authorization) {
        audit(principal.uid, &request.id, &request.operation, "deny");
        let response = Response::failure(
            request.id,
            "forbidden",
            "The operation is not available to this client",
        );
        return write_response(&mut stream, &response).await;
    }

    audit(principal.uid, &request.id, &request.operation, "allow");

    let result = match operation {
        Operation::HealthRead => json!({
            "status": "ok",
            "protocol_version": PROTOCOL_VERSION
        }),
        Operation::StatusRead => json!({
            "status": "running",
            "uptime_seconds": state.started_at.elapsed().as_secs(),
            "protocol_version": PROTOCOL_VERSION
        }),
    };

    write_response(&mut stream, &Response::success(request.id, result)).await
}

async fn run_server(
    listener: UnixListener,
    state: Arc<ServerState>,
) -> io::Result<()> {
    let permits = Arc::new(Semaphore::new(MAX_CONNECTIONS));
    let mut tasks = JoinSet::new();
    let mut shutdown = Box::pin(tokio::signal::ctrl_c());

    loop {
        tokio::select! {
            _ = &mut shutdown => {
                eprintln!("assistantd: shutdown requested");
                break;
            }
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                let permit = match permits.clone().try_acquire_owned() {
                    Ok(permit) => permit,
                    Err(_) => {
                        eprintln!("assistantd: connection limit reached; rejecting client");
                        drop(stream);
                        continue;
                    }
                };
                let state = state.clone();
                tasks.spawn(async move {
                    let _permit = permit;
                    match timeout(CONNECTION_DEADLINE, handle_connection(stream, state)).await {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => eprintln!("assistantd: client request rejected: {error}"),
                        Err(_) => eprintln!("assistantd: client request deadline exceeded"),
                    }
                });
            }
            Some(result) = tasks.join_next(), if !tasks.is_empty() => {
                if let Err(error) = result {
                    eprintln!("assistantd: connection task failed: {error}");
                }
            }
        }
    }

    drop(listener);
    while let Some(result) = tasks.join_next().await {
        if let Err(error) = result {
            eprintln!("assistantd: connection task failed during shutdown: {error}");
        }
    }

    Ok(())
}

async fn serve() -> Result<(), Box<dyn Error>> {
    let directory = ensure_runtime_directory()?;
    let owner_uid = Uid::effective().as_raw();
    let socket_path = directory.join("core.sock");

    prepare_socket_path(&socket_path, owner_uid)?;

    let listener = UnixListener::bind(&socket_path)?;
    fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o600))?;
    let metadata = fs::symlink_metadata(&socket_path)?;
    let cleanup = SocketCleanup {
        path: socket_path.clone(),
        device: metadata.dev(),
        inode: metadata.ino(),
        owner_uid,
    };

    eprintln!("assistantd: listening on {}", socket_path.display());
    let state = Arc::new(ServerState {
        owner_uid,
        started_at: Instant::now(),
        broker: PermissionBroker { owner_uid },
    });

    let result = run_server(listener, state).await;
    drop(cleanup);
    result?;
    Ok(())
}

async fn call(operation: Operation) -> Result<(), Box<dyn Error>> {
    let socket_path = runtime_directory()?.join("core.sock");
    let mut stream = UnixStream::connect(&socket_path).await?;
    let request = Request {
        version: PROTOCOL_VERSION,
        id: "cli-1".to_owned(),
        operation: operation.as_str().to_owned(),
    };

    let mut bytes = serde_json::to_vec(&request)?;
    bytes.push(b'\n');
    stream.write_all(&bytes).await?;

    let frame = read_frame(&mut stream).await?;
    let response: Response = serde_json::from_slice(&frame)?;

    if !response.ok {
        let error = response
            .error
            .map(|error| format!("{}: {}", error.code, error.message))
            .unwrap_or_else(|| "unknown core error".to_owned());
        return Err(io_error(io::ErrorKind::PermissionDenied, error).into());
    }

    println!("{}", serde_json::to_string_pretty(&response.result)?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    match arguments.next().as_deref() {
        Some("serve") => serve().await,
        Some("health") => call(Operation::HealthRead).await,
        Some("status") => call(Operation::StatusRead).await,
        Some("--version") => {
            println!("assistantd {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Some("help") | Some("--help") | None => {
            println!(
                "Local Assistant 2.0 core prototype\n\
                 Usage: assistantd <serve|health|status|--version>\n\
                 The daemon uses a private Unix socket under XDG_RUNTIME_DIR."
            );
            Ok(())
        }
        Some(_) => Err(io_error(
            io::ErrorKind::InvalidInput,
            "unknown command; use assistantd --help",
        )
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broker_allows_only_safe_core_reads_for_owner() {
        let broker = PermissionBroker { owner_uid: 1000 };
        let principal = Principal { uid: 1000 };

        for operation in [Operation::HealthRead, Operation::StatusRead] {
            assert!(broker.authorize(&AuthorizationRequest {
                principal,
                operation: Some(operation),
                resource: "core",
            }));
        }

        assert!(!broker.authorize(&AuthorizationRequest {
            principal: Principal { uid: 1001 },
            operation: Some(Operation::HealthRead),
            resource: "core",
        }));
        assert!(!broker.authorize(&AuthorizationRequest {
            principal,
            operation: None,
            resource: "core",
        }));
        assert!(!broker.authorize(&AuthorizationRequest {
            principal,
            operation: Some(Operation::HealthRead),
            resource: "other",
        }));
    }

    #[test]
    fn request_cannot_supply_identity_or_extra_fields() {
        let request = br#"{"version":1,"id":"test-1","operation":"core.health.read","uid":0}"#;
        assert!(serde_json::from_slice::<Request>(request).is_err());
    }

    #[test]
    fn request_ids_are_bounded_and_ascii_safe() {
        assert!(validate_request_id("req-01_A"));
        assert!(!validate_request_id(""));
        assert!(!validate_request_id(&"a".repeat(65)));
        assert!(!validate_request_id("request with spaces"));
    }

    #[tokio::test]
    async fn frame_reader_accepts_one_bounded_line() {
        let (mut writer, mut reader) = tokio::io::duplex(32);
        writer.write_all(b"{\\"version\\":1}\\n").await.unwrap();
        let frame = read_frame(&mut reader).await.unwrap();
        assert_eq!(frame, br#"{\"version\":1}"#);
    }

    #[tokio::test]
    async fn frame_reader_rejects_oversized_input() {
        let (mut writer, mut reader) = tokio::io::duplex(MAX_FRAME_BYTES + 8);
        writer
            .write_all(&vec![b'x'; MAX_FRAME_BYTES + 1])
            .await
            .unwrap();
        let error = read_frame(&mut reader).await.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[tokio::test]
    async fn frame_reader_rejects_multiple_frames_in_one_read() {
        let (mut writer, mut reader) = tokio::io::duplex(32);
        writer.write_all(b"one\\ntwo\\n").await.unwrap();
        let error = read_frame(&mut reader).await.unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }
}
