use crate::identity::ClientIdentity;

/// The authenticated principal responsible for an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal {
    pub pid: i32,
    pub uid: u32,
    pub gid: u32,
}

impl From<&ClientIdentity> for Principal {
    fn from(identity: &ClientIdentity) -> Self {
        Self {
            pid: identity.pid,
            uid: identity.uid,
            gid: identity.gid,
        }
    }
}

/// Identifies the execution context of one operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestContext {
    pub principal: Principal,
    pub workspace: Option<String>,
    pub task_id: Option<String>,
    pub request_id: String,
}

impl RequestContext {
    pub fn new(
        principal: Principal,
        workspace: Option<String>,
        task_id: Option<String>,
        request_id: String,
    ) -> Self {
        Self {
            principal,
            workspace,
            task_id,
            request_id,
        }
    }

    pub fn for_client(identity: &ClientIdentity, request_id: String) -> Self {
        Self::new(Principal::from(identity), None, None, request_id)
    }

    pub fn with_task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = Some(task_id.into());
        self
    }

    pub fn with_workspace(mut self, workspace: impl Into<String>) -> Self {
        self.workspace = Some(workspace.into());
        self
    }

    /// Writes a structured diagnostic event associated with this request.
    pub fn log_event(&self, event: &str, details: Option<&str>) {
        match details {
            Some(details) => eprintln!(
                "event={} request_id={} task_id={} pid={} uid={} gid={} details={:?}",
                event,
                self.request_id,
                self.task_id.as_deref().unwrap_or("-"),
                self.principal.pid,
                self.principal.uid,
                self.principal.gid,
                details
            ),
            None => eprintln!(
                "event={} request_id={} task_id={} pid={} uid={} gid={}",
                event,
                self.request_id,
                self.task_id.as_deref().unwrap_or("-"),
                self.principal.pid,
                self.principal.uid,
                self.principal.gid
            ),
        }
    }
}
