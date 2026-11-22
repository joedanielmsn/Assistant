use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    os::unix::{
        fs::{MetadataExt, PermissionsExt},
        net::UnixStream,
    },
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

use nix::{
    sys::signal::{Signal, kill},
    unistd::{Pid, Uid},
};
use serde_json::Value;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

struct Daemon {
    child: Child,
    runtime_root: PathBuf,
    socket_path: PathBuf,
}

impl Daemon {
    fn start() -> Self {
        let runtime_root = std::env::temp_dir().join(format!(
            "assistantd-ipc-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));

        fs::create_dir(&runtime_root).unwrap();
        fs::set_permissions(&runtime_root, fs::Permissions::from_mode(0o700)).unwrap();

        let socket_path = runtime_root.join("assistant-v2/core.sock");

        let child = Command::new(env!("CARGO_BIN_EXE_assistantd"))
            .arg("serve")
            .env("XDG_RUNTIME_DIR", &runtime_root)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let mut daemon = Self {
            child,
            runtime_root,
            socket_path,
        };

        let deadline = Instant::now() + Duration::from_secs(3);

        loop {
            if daemon.socket_path.exists() {
                break;
            }

            if let Some(status) = daemon.child.try_wait().unwrap() {
                panic!("assistantd exited before creating its socket: {status}");
            }

            assert!(
                Instant::now() < deadline,
                "assistantd did not create its socket in time"
            );

            thread::sleep(Duration::from_millis(10));
        }

        daemon
    }

    fn request(&self, bytes: &[u8]) -> Value {
        let mut stream = UnixStream::connect(&self.socket_path).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();

        let mut frame = bytes.to_vec();
        if !frame.ends_with(b"\n") {
            frame.push(b'\n');
        }
        stream.write_all(&frame).unwrap();

        let mut response = String::new();
        BufReader::new(stream).read_line(&mut response).unwrap();

        serde_json::from_str(&response).unwrap()
    }

    fn stop(&mut self) {
        if self.child.try_wait().unwrap().is_none() {
            kill(Pid::from_raw(self.child.id() as i32), Signal::SIGINT).unwrap();

            let status = self.child.wait().unwrap();
            assert!(status.success(), "assistantd exited unsuccessfully");
        }

        assert!(
            !self.socket_path.exists(),
            "daemon socket was not cleaned up"
        );
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = kill(Pid::from_raw(self.child.id() as i32), Signal::SIGINT);
            let _ = self.child.wait();
        }

        let _ = fs::remove_dir_all(&self.runtime_root);
    }
}

#[test]
fn daemon_ipc_security_and_lifecycle() {
    let mut daemon = Daemon::start();

    // Runtime directory and socket permissions.
    let runtime_dir = daemon.socket_path.parent().unwrap();
    let dir_metadata = fs::symlink_metadata(runtime_dir).unwrap();
    let socket_metadata = fs::symlink_metadata(&daemon.socket_path).unwrap();

    assert!(dir_metadata.is_dir());
    assert_eq!(dir_metadata.permissions().mode() & 0o777, 0o700);
    assert_eq!(socket_metadata.permissions().mode() & 0o777, 0o600);
    assert_eq!(socket_metadata.uid(), Uid::current().as_raw());

    // Health endpoint.
    let health = daemon.request(br#"{"version":1,"id":"health-1","operation":"core.health.read"}"#);

    assert_eq!(health["ok"], true);
    assert_eq!(health["id"], "health-1");
    assert_eq!(health["result"]["status"], "ok");
    assert_eq!(health["result"]["protocol_version"], 1);

    // Status endpoint.
    let status = daemon.request(br#"{"version":1,"id":"status-1","operation":"core.status.read"}"#);

    assert_eq!(status["ok"], true);
    assert_eq!(status["result"]["status"], "running");

    // Unknown operations must be denied.
    let denied = daemon.request(br#"{"version":1,"id":"deny-1","operation":"system.execute"}"#);

    assert_eq!(denied["ok"], false);
    assert_eq!(denied["error"]["code"], "forbidden");

    // Unsupported protocol versions must be rejected.
    let unsupported =
        daemon.request(br#"{"version":99,"id":"version-1","operation":"core.health.read"}"#);

    assert_eq!(unsupported["ok"], false);
    assert_eq!(unsupported["error"]["code"], "unsupported_version");

    // Malformed JSON must receive a structured error.
    let malformed = daemon.request(b"{invalid-json}\n");

    assert_eq!(malformed["ok"], false);
    assert_eq!(malformed["error"]["code"], "invalid_request");

    // An active socket must never be replaced by another daemon.
    let second = Command::new(env!("CARGO_BIN_EXE_assistantd"))
        .arg("serve")
        .env("XDG_RUNTIME_DIR", &daemon.runtime_root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();

    assert!(!second.success());
    assert!(daemon.socket_path.exists());

    // Oversized frames must be rejected without a response.
    let mut oversized = UnixStream::connect(&daemon.socket_path).unwrap();
    oversized
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    oversized.write_all(&vec![b'x'; 4097]).unwrap();

    let mut response = Vec::new();
    oversized.read_to_end(&mut response).unwrap();
    assert!(response.is_empty());

    // Graceful shutdown must remove the socket.
    daemon.stop();
    assert!(!daemon.socket_path.exists());
}
