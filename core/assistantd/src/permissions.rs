use crate::identity::ClientIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    SystemInitialize,
    SettingsRead,
    SettingsWrite,
    ConversationRead,
    ConversationWrite,
    ChatCancel,
    AgentToolExecute,
}

impl Capability {
    pub fn name(self) -> &'static str {
        match self {
            Self::SystemInitialize => "system.initialize",
            Self::SettingsRead => "settings.read",
            Self::SettingsWrite => "settings.write",
            Self::ConversationRead => "conversation.read",
            Self::ConversationWrite => "conversation.write",
            Self::ChatCancel => "chat.cancel",
            Self::AgentToolExecute => "agent.tool.execute",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationDecision {
    pub allowed: bool,
    pub capability: &'static str,
    pub reason: &'static str,
}

pub struct PermissionBroker {
    desktop_uid: u32,
}

impl PermissionBroker {
    /// Creates the initial desktop policy for the authenticated daemon user.
    /// Future principals (agents and workers) will receive separate policies.
    pub fn for_desktop_user(uid: u32) -> Self {
        Self { desktop_uid: uid }
    }

    pub fn authorize(
        &self,
        identity: &ClientIdentity,
        capability: Capability,
    ) -> AuthorizationDecision {
        if identity.uid != self.desktop_uid {
            return AuthorizationDecision {
                allowed: false,
                capability: capability.name(),
                reason: "Principal UID is not authorized",
            };
        }

        AuthorizationDecision {
            allowed: true,
            capability: capability.name(),
            reason: "Authenticated desktop user policy grants this capability",
        }
    }

    pub fn capability_for_method(method: &str) -> Option<Capability> {
        match method {
            "initialize" => Some(Capability::SystemInitialize),
            "settings.get" => Some(Capability::SettingsRead),
            "settings.update" => Some(Capability::SettingsWrite),
            "conversation.list" | "conversation.get" => Some(Capability::ConversationRead),
            "conversation.create" | "conversation.rename" | "conversation.delete" | "chat.send" => {
                Some(Capability::ConversationWrite)
            }
            "chat.cancel" => Some(Capability::ChatCancel),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Capability, PermissionBroker};
    use crate::identity::ClientIdentity;

    fn identity(uid: u32) -> ClientIdentity {
        ClientIdentity {
            pid: 123,
            uid,
            gid: uid,
        }
    }

    #[test]
    fn grants_known_capability_to_authenticated_desktop_user() {
        let broker = PermissionBroker::for_desktop_user(1000);
        let decision = broker.authorize(&identity(1000), Capability::ConversationRead);

        assert!(decision.allowed);
        assert_eq!(decision.capability, "conversation.read");
    }

    #[test]
    fn denies_different_user() {
        let broker = PermissionBroker::for_desktop_user(1000);
        let decision = broker.authorize(&identity(1001), Capability::ConversationRead);

        assert!(!decision.allowed);
    }

    #[test]
    fn maps_every_existing_ipc_method() {
        for method in [
            "initialize",
            "settings.get",
            "settings.update",
            "conversation.create",
            "conversation.list",
            "conversation.get",
            "conversation.rename",
            "conversation.delete",
            "chat.send",
            "chat.cancel",
        ] {
            assert!(
                PermissionBroker::capability_for_method(method).is_some(),
                "Missing capability mapping for {method}"
            );
        }
    }

    #[test]
    fn denies_unknown_methods_by_default() {
        assert_eq!(
            PermissionBroker::capability_for_method("system.shell"),
            None
        );
        assert_eq!(
            PermissionBroker::capability_for_method("screen.capture"),
            None
        );
    }
}
