use crate::permissions::Capability;
use crate::request_context::{Principal, RequestContext};

use super::registry::AgentDefinition;

/// Immutable execution identity and permissions for a specialist agent.
#[derive(Debug, Clone)]
pub struct AgentContext {
    agent_id: String,
    principal: Principal,
    task_id: Option<String>,
    request_id: String,
    capabilities: Vec<Capability>,
}

impl AgentContext {
    pub(crate) fn from_definition(
        definition: &AgentDefinition,
        request: &RequestContext,
    ) -> Self {
        Self {
            agent_id: definition.id.clone(),
            principal: request.principal.clone(),
            task_id: request.task_id.clone(),
            request_id: request.request_id.clone(),
            capabilities: definition.capabilities.clone(),
        }
    }

    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn principal(&self) -> &Principal {
        &self.principal
    }

    pub fn task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }

    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    pub fn has_capability(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }
}

#[cfg(test)]
mod tests {
    use super::AgentContext;
    use crate::agent::registry::AgentRegistry;
    use crate::identity::ClientIdentity;
    use crate::permissions::Capability;
    use crate::request_context::RequestContext;

    fn request_context() -> RequestContext {
        let identity = ClientIdentity {
            pid: 123,
            uid: 1000,
            gid: 1000,
        };

        RequestContext::for_client(&identity, "request-123".to_string())
            .with_task_id("task-456")
    }

    #[test]
    fn creates_context_from_registered_agent() {
        let registry = AgentRegistry::with_builtins().unwrap();
        let definition = registry.get("general").unwrap();

        let context =
            AgentContext::from_definition(definition, &request_context());

        assert_eq!(context.agent_id(), "general");
        assert_eq!(context.task_id(), Some("task-456"));
        assert_eq!(context.request_id(), "request-123");
    }

    #[test]
    fn inherits_authenticated_principal() {
        let registry = AgentRegistry::with_builtins().unwrap();
        let definition = registry.get("general").unwrap();

        let context =
            AgentContext::from_definition(definition, &request_context());

        assert_eq!(context.principal().pid, 123);
        assert_eq!(context.principal().uid, 1000);
        assert_eq!(context.principal().gid, 1000);
    }

    #[test]
    fn receives_only_registered_agent_capabilities() {
        let registry = AgentRegistry::with_builtins().unwrap();

        let general = AgentContext::from_definition(
            registry.get("general").unwrap(),
            &request_context(),
        );

        let system = AgentContext::from_definition(
            registry.get("system").unwrap(),
            &request_context(),
        );

        assert!(general.has_capability(Capability::AgentToolExecute));
        assert!(!system.has_capability(Capability::AgentToolExecute));
    }

    #[test]
    fn preserves_context_without_optional_task_id() {
        let registry = AgentRegistry::with_builtins().unwrap();
        let identity = ClientIdentity {
            pid: 123,
            uid: 1000,
            gid: 1000,
        };

        let request =
            RequestContext::for_client(&identity, "request-789".to_string());

        let context = AgentContext::from_definition(
            registry.get("system").unwrap(),
            &request,
        );

        assert_eq!(context.task_id(), None);
        assert_eq!(context.request_id(), "request-789");
    }
}
