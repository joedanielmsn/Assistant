use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::{Value, json};

use crate::agent::tools::ToolRegistry;
use crate::permissions::Capability;

const MAX_FILE_BYTES: u64 = 64 * 1024;
const MAX_DIRECTORY_ENTRIES: usize = 100;

pub fn register(registry: &mut ToolRegistry) -> Result<(), String> {
    let workspace_root = configured_workspace();

    registry.register(
        "system_info",
        "Returns basic operating-system, architecture, hostname, and memory information. Read-only.",
        "1.0.0",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
        Capability::AgentToolExecute,
        |_| system_info(),
    )?;

    let list_root = workspace_root.clone();
    registry.register(
        "workspace_list",
        "Lists files and directories inside the configured workspace. Accepts an optional relative directory path. Read-only.",
        "1.0.0",
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "maxLength": 512}
            },
            "additionalProperties": false
        }),
        Capability::AgentToolExecute,
        move |arguments| list_workspace(&list_root, arguments),
    )?;

    registry.register(
        "workspace_read",
        "Reads a UTF-8 text file inside the configured workspace, up to 64 KiB. Requires a relative path. Read-only.",
        "1.0.0",
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "minLength": 1, "maxLength": 512}
            },
            "required": ["path"],
            "additionalProperties": false
        }),
        Capability::AgentToolExecute,
        move |arguments| read_workspace_file(&workspace_root, arguments),
    )?;

    Ok(())
}

fn configured_workspace() -> Option<PathBuf> {
    let configured = std::env::var_os("LOCAL_ASSISTANT_WORKSPACE")?;
    let root = fs::canonicalize(configured).ok()?;

    root.is_dir().then_some(root)
}

fn system_info() -> Result<Value, String> {
    let os_name = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                line.strip_prefix("PRETTY_NAME=")
                    .map(|value| value.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| std::env::consts::OS.to_string());

    let memory = fs::read_to_string("/proc/meminfo")
        .ok()
        .map(|contents| {
            let mut total_kb = None;
            let mut available_kb = None;

            for line in contents.lines() {
                if let Some(value) = line.strip_prefix("MemTotal:") {
                    total_kb = value
                        .split_whitespace()
                        .next()
                        .and_then(|number| number.parse::<u64>().ok());
                } else if let Some(value) = line.strip_prefix("MemAvailable:") {
                    available_kb = value
                        .split_whitespace()
                        .next()
                        .and_then(|number| number.parse::<u64>().ok());
                }
            }

            json!({
                "total_kib": total_kb,
                "available_kib": available_kb
            })
        })
        .unwrap_or(Value::Null);

    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());

    Ok(json!({
        "operating_system": os_name,
        "platform": std::env::consts::FAMILY,
        "architecture": std::env::consts::ARCH,
        "hostname": hostname,
        "memory": memory
    }))
}

fn resolve_workspace_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let relative_path = Path::new(relative);

    if relative_path.is_absolute()
        || relative_path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(
            "Workspace paths must be relative and cannot contain traversal components".to_string(),
        );
    }

    let candidate = fs::canonicalize(root.join(relative_path))
        .map_err(|error| format!("Could not resolve workspace path: {error}"))?;

    if !candidate.starts_with(root) {
        return Err("Access outside the configured workspace is denied".to_string());
    }

    Ok(candidate)
}

fn is_sensitive_path(path: &Path) -> bool {
    for component in path.components() {
        let name = component.as_os_str().to_string_lossy().to_ascii_lowercase();

        if matches!(name.as_str(), ".ssh" | ".gnupg" | ".aws") {
            return true;
        }

        if name == ".env"
            || name.starts_with(".env.")
            || name == "credentials"
            || name.contains("secret")
            || name == "id_rsa"
            || name == "id_ed25519"
            || name.ends_with(".pem")
            || name.ends_with(".key")
        {
            return true;
        }
    }

    false
}

fn list_workspace(root: &Option<PathBuf>, arguments: &Value) -> Result<Value, String> {
    let root = root.as_ref().ok_or_else(|| {
        "No valid workspace configured. Set LOCAL_ASSISTANT_WORKSPACE to a directory.".to_string()
    })?;

    let relative = arguments["path"].as_str().unwrap_or("");
    let directory = if relative.is_empty() {
        root.clone()
    } else {
        resolve_workspace_path(root, relative)?
    };

    if !directory.is_dir() {
        return Err("The requested workspace path is not a directory".to_string());
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(&directory)
        .map_err(|error| format!("Could not list workspace directory: {error}"))?
    {
        let entry = entry.map_err(|error| format!("Could not read directory entry: {error}"))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("Could not inspect directory entry: {error}"))?;

        // Do not follow symlinks while enumerating a workspace.
        if file_type.is_symlink() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().into_owned();
        let metadata = entry.metadata().ok();

        entries.push(json!({
            "name": name,
            "kind": if file_type.is_dir() { "directory" } else if file_type.is_file() { "file" } else { "other" },
            "size_bytes": metadata.map(|value| value.len())
        }));
    }

    entries.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));

    let truncated = entries.len() > MAX_DIRECTORY_ENTRIES;
    entries.truncate(MAX_DIRECTORY_ENTRIES);

    Ok(json!({
        "directory": relative,
        "entries": entries,
        "truncated": truncated,
        "maximum_entries": MAX_DIRECTORY_ENTRIES
    }))
}

fn read_workspace_file(root: &Option<PathBuf>, arguments: &Value) -> Result<Value, String> {
    let root = root.as_ref().ok_or_else(|| {
        "No valid workspace configured. Set LOCAL_ASSISTANT_WORKSPACE to a directory.".to_string()
    })?;

    let relative = arguments["path"]
        .as_str()
        .ok_or_else(|| "A relative file path is required".to_string())?;

    let path = resolve_workspace_path(root, relative)?;

    if is_sensitive_path(Path::new(relative)) {
        return Err("Reading sensitive files is not permitted".to_string());
    }

    let metadata = fs::metadata(&path)
        .map_err(|error| format!("Could not inspect workspace file: {error}"))?;

    if !metadata.is_file() {
        return Err("The requested path is not a regular file".to_string());
    }

    if metadata.len() > MAX_FILE_BYTES {
        return Err(format!(
            "File exceeds the {} KiB read limit",
            MAX_FILE_BYTES / 1024
        ));
    }

    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read UTF-8 text file: {error}"))?;

    Ok(json!({
        "path": relative,
        "size_bytes": metadata.len(),
        "content": content
    }))
}

#[cfg(test)]
mod tests {
    use super::{list_workspace, read_workspace_file, resolve_workspace_path};
    use serde_json::json;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

    fn test_root() -> PathBuf {
        let id = TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "local-assistant-workspace-test-{}-{id}",
            std::process::id()
        ));

        fs::create_dir_all(&root).unwrap();
        root.canonicalize().unwrap()
    }

    #[test]
    fn resolves_relative_workspace_paths() {
        let root = test_root();
        fs::write(root.join("hello.txt"), "hello").unwrap();

        assert_eq!(
            resolve_workspace_path(&root, "hello.txt").unwrap(),
            root.join("hello.txt")
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_absolute_and_traversal_paths() {
        let root = test_root();

        assert!(resolve_workspace_path(&root, "../outside").is_err());
        assert!(resolve_workspace_path(&root, "/etc/passwd").is_err());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reads_workspace_text_files() {
        let root = test_root();
        fs::write(root.join("hello.txt"), "workspace content").unwrap();

        let result =
            read_workspace_file(&Some(root.clone()), &json!({"path": "hello.txt"})).unwrap();

        assert_eq!(result["content"], "workspace content");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn refuses_sensitive_files() {
        let root = test_root();
        fs::write(root.join(".env"), "SECRET=value").unwrap();

        assert!(read_workspace_file(&Some(root.clone()), &json!({"path": ".env"})).is_err());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn lists_workspace_entries() {
        let root = test_root();
        fs::write(root.join("hello.txt"), "hello").unwrap();

        let result = list_workspace(&Some(root.clone()), &json!({})).unwrap();

        assert_eq!(result["entries"][0]["name"], "hello.txt");

        fs::remove_dir_all(root).unwrap();
    }
}
