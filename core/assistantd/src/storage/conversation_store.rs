use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};

#[derive(Debug, Clone)]
pub struct ConversationSummary {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct StoredMessage {
    pub id: i64,
    pub role: String,
    pub content: String,
    pub created_at: i64,
}

pub struct ConversationStore {
    connection: Connection,
}

impl ConversationStore {
    pub fn open() -> Result<Self, String> {
        let directory = data_directory()?;

        fs::create_dir_all(&directory)
            .map_err(|e| format!("Could not create data directory: {e}"))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))
                .map_err(|e| format!("Could not secure data directory: {e}"))?;
        }

        let database_path = directory.join("assistant.db");

        let connection = Connection::open(&database_path)
            .map_err(|e| format!("Could not open conversation database: {e}"))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&database_path, fs::Permissions::from_mode(0o600))
                .map_err(|e| format!("Could not secure database file: {e}"))?;
        }

        connection
            .busy_timeout(std::time::Duration::from_secs(5))
            .map_err(|e| format!("Could not configure database timeout: {e}"))?;

        connection
            .execute_batch(
                "
                PRAGMA foreign_keys = ON;
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;

                CREATE TABLE IF NOT EXISTS conversations (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL DEFAULT 'New conversation',
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );

                CREATE TABLE IF NOT EXISTS messages (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    conversation_id TEXT NOT NULL,
                    role TEXT NOT NULL CHECK(role IN ('user', 'assistant')),
                    content TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    FOREIGN KEY(conversation_id)
                        REFERENCES conversations(id)
                        ON DELETE CASCADE
                );

                CREATE INDEX IF NOT EXISTS idx_conversations_updated
                    ON conversations(updated_at DESC);

                CREATE INDEX IF NOT EXISTS idx_messages_conversation
                    ON messages(conversation_id, id);

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS conversation_task_context (
                    conversation_id TEXT PRIMARY KEY,
                    context_json TEXT NOT NULL,
                    updated_at INTEGER NOT NULL,
                    FOREIGN KEY(conversation_id)
                        REFERENCES conversations(id)
                        ON DELETE CASCADE
                );
                ",
            )
            .map_err(|e| format!("Could not initialize database schema: {e}"))?;

        Ok(Self { connection })
    }

    pub fn context_length(&self, default: usize) -> Result<usize, String> {
        let stored: Option<String> = self
            .connection
            .query_row(
                "SELECT value FROM settings WHERE key = 'context_length_tokens'",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| format!("Could not load context setting: {e}"))?;

        match stored {
            Some(value) => value
                .parse::<usize>()
                .map_err(|e| format!("Stored context setting is invalid: {e}")),
            None => Ok(default),
        }
    }

    pub fn set_context_length(&mut self, value: usize) -> Result<(), String> {
        self.connection
            .execute(
                "INSERT INTO settings (key, value)
                 VALUES ('context_length_tokens', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![value.to_string()],
            )
            .map_err(|e| format!("Could not save context setting: {e}"))?;

        Ok(())
    }

    pub fn load_task_context(
        &self,
        conversation_id: &str,
    ) -> Result<Option<serde_json::Value>, String> {
        let stored: Option<String> = self
            .connection
            .query_row(
                "SELECT context_json FROM conversation_task_context
                 WHERE conversation_id = ?1",
                params![conversation_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| format!("Could not load task context: {e}"))?;

        stored
            .map(|value| {
                serde_json::from_str(&value)
                    .map_err(|e| format!("Stored task context is invalid: {e}"))
            })
            .transpose()
    }

    pub fn save_task_context(
        &mut self,
        conversation_id: &str,
        context: &serde_json::Value,
    ) -> Result<(), String> {
        if !context.is_object() {
            return Err("Task context must be a JSON object".to_string());
        }

        let serialized = serde_json::to_string(context)
            .map_err(|e| format!("Could not serialize task context: {e}"))?;

        self.connection
            .execute(
                "INSERT INTO conversation_task_context
                    (conversation_id, context_json, updated_at)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(conversation_id) DO UPDATE SET
                    context_json = excluded.context_json,
                    updated_at = excluded.updated_at",
                params![conversation_id, serialized, current_timestamp()],
            )
            .map_err(|e| format!("Could not save task context: {e}"))?;

        Ok(())
    }

    pub fn clear_task_context(&mut self, conversation_id: &str) -> Result<(), String> {
        self.connection
            .execute(
                "DELETE FROM conversation_task_context WHERE conversation_id = ?1",
                params![conversation_id],
            )
            .map_err(|e| format!("Could not clear task context: {e}"))?;

        Ok(())
    }

    pub fn create(&mut self, id: &str) -> Result<ConversationSummary, String> {
        let now = current_timestamp();

        self.connection
            .execute(
                "
                INSERT INTO conversations (id, title, created_at, updated_at)
                VALUES (?1, 'New conversation', ?2, ?2)
                ",
                params![id, now],
            )
            .map_err(|e| format!("Could not create conversation: {e}"))?;

        Ok(ConversationSummary {
            id: id.to_string(),
            title: "New conversation".to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn list(&self) -> Result<Vec<ConversationSummary>, String> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT id, title, created_at, updated_at
                FROM conversations
                ORDER BY updated_at DESC, id DESC
                ",
            )
            .map_err(|e| format!("Could not prepare conversation list: {e}"))?;

        let rows = statement
            .query_map([], |row| {
                Ok(ConversationSummary {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            })
            .map_err(|e| format!("Could not query conversations: {e}"))?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Could not read conversations: {e}"))
    }

    pub fn load(
        &self,
        conversation_id: &str,
    ) -> Result<Option<(ConversationSummary, Vec<StoredMessage>)>, String> {
        let conversation = self
            .connection
            .query_row(
                "
                SELECT id, title, created_at, updated_at
                FROM conversations
                WHERE id = ?1
                ",
                params![conversation_id],
                |row| {
                    Ok(ConversationSummary {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        created_at: row.get(2)?,
                        updated_at: row.get(3)?,
                    })
                },
            )
            .optional()
            .map_err(|e| format!("Could not load conversation: {e}"))?;

        let Some(conversation) = conversation else {
            return Ok(None);
        };

        let mut statement = self
            .connection
            .prepare(
                "
                SELECT id, role, content, created_at
                FROM messages
                WHERE conversation_id = ?1
                ORDER BY id ASC
                ",
            )
            .map_err(|e| format!("Could not prepare message query: {e}"))?;

        let rows = statement
            .query_map(params![conversation_id], |row| {
                Ok(StoredMessage {
                    id: row.get(0)?,
                    role: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .map_err(|e| format!("Could not query messages: {e}"))?;

        let messages = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Could not read messages: {e}"))?;

        Ok(Some((conversation, messages)))
    }

    pub fn append_exchange(
        &mut self,
        conversation_id: &str,
        user_text: &str,
        assistant_text: &str,
    ) -> Result<(), String> {
        let now = current_timestamp();
        let title = make_title(user_text);

        let transaction = self
            .connection
            .transaction()
            .map_err(|e| format!("Could not begin message transaction: {e}"))?;

        let exists: bool = transaction
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM conversations WHERE id = ?1)",
                params![conversation_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("Could not check conversation: {e}"))?;

        if !exists {
            return Err("Conversation not found".to_string());
        }

        transaction
            .execute(
                "
                INSERT INTO messages (conversation_id, role, content, created_at)
                VALUES (?1, 'user', ?2, ?3)
                ",
                params![conversation_id, user_text, now],
            )
            .map_err(|e| format!("Could not save user message: {e}"))?;

        transaction
            .execute(
                "
                INSERT INTO messages (conversation_id, role, content, created_at)
                VALUES (?1, 'assistant', ?2, ?3)
                ",
                params![conversation_id, assistant_text, now],
            )
            .map_err(|e| format!("Could not save assistant message: {e}"))?;

        transaction
            .execute(
                "
                UPDATE conversations
                SET title = CASE
                    WHEN title = 'New conversation' THEN ?1
                    ELSE title
                END,
                updated_at = ?2
                WHERE id = ?3
                ",
                params![title, now, conversation_id],
            )
            .map_err(|e| format!("Could not update conversation: {e}"))?;

        transaction
            .commit()
            .map_err(|e| format!("Could not commit messages: {e}"))?;

        Ok(())
    }

    pub fn rename(&mut self, conversation_id: &str, title: &str) -> Result<bool, String> {
        let title = title.trim();

        if title.is_empty() {
            return Err("Conversation title cannot be empty".to_string());
        }

        if title.chars().count() > 100 {
            return Err("Conversation title cannot exceed 100 characters".to_string());
        }

        let changed = self
            .connection
            .execute(
                "
                UPDATE conversations
                SET title = ?1, updated_at = ?2
                WHERE id = ?3
                ",
                params![title, current_timestamp(), conversation_id],
            )
            .map_err(|e| format!("Could not rename conversation: {e}"))?;

        Ok(changed > 0)
    }

    pub fn delete(&mut self, conversation_id: &str) -> Result<bool, String> {
        let changed = self
            .connection
            .execute(
                "DELETE FROM conversations WHERE id = ?1",
                params![conversation_id],
            )
            .map_err(|e| format!("Could not delete conversation: {e}"))?;

        Ok(changed > 0)
    }
}

fn data_directory() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(path).join("local-assistant"));
    }

    let home =
        env::var_os("HOME").ok_or_else(|| "Neither XDG_DATA_HOME nor HOME is set".to_string())?;

    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("local-assistant"))
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn make_title(text: &str) -> String {
    let first_line = text.lines().next().unwrap_or("").trim();

    let title: String = first_line.chars().take(72).collect();

    if title.is_empty() {
        "New conversation".to_string()
    } else {
        title
    }
}

#[cfg(test)]
mod task_context_tests {
    use super::*;

    fn test_store() -> ConversationStore {
        let connection = Connection::open_in_memory().unwrap();

        connection
            .execute_batch(
                "
                PRAGMA foreign_keys = ON;

                CREATE TABLE conversations (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL DEFAULT 'New conversation',
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );

                CREATE TABLE conversation_task_context (
                    conversation_id TEXT PRIMARY KEY,
                    context_json TEXT NOT NULL,
                    updated_at INTEGER NOT NULL,
                    FOREIGN KEY(conversation_id)
                        REFERENCES conversations(id)
                        ON DELETE CASCADE
                );
                ",
            )
            .unwrap();

        ConversationStore { connection }
    }

    #[test]
    fn task_context_survives_storage_round_trip() {
        let mut store = test_store();
        store.create("conversation-a").unwrap();

        let context = serde_json::json!({
            "objective": "Build Local Assistant",
            "constraints": ["Use Rust", "Keep inference local"],
            "additionalContext": "Linux desktop implementation"
        });

        store.save_task_context("conversation-a", &context).unwrap();

        let loaded = store.load_task_context("conversation-a").unwrap();

        assert_eq!(loaded, Some(context));
    }

    #[test]
    fn task_context_is_deleted_with_its_conversation() {
        let mut store = test_store();
        store.create("conversation-a").unwrap();

        let context = serde_json::json!({
            "objective": "Test conversation isolation"
        });

        store.save_task_context("conversation-a", &context).unwrap();

        store.delete("conversation-a").unwrap();

        assert_eq!(store.load_task_context("conversation-a").unwrap(), None);
    }
}
