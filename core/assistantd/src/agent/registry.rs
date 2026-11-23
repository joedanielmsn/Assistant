use std::collections::HashMap;

use crate::permissions::Capability;
use crate::request_context::RequestContext;

use super::context::AgentContext;

#[derive(Debug, Clone)]
pub struct AgentDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub capabilities: Vec<Capability>,
}

#[derive(Default)]
pub struct AgentRegistry {
    agents: HashMap<String, AgentDefinition>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        definition: AgentDefinition,
    ) -> Result<(), String> {
        if definition.id.trim().is_empty()
            || definition.id.len() > 64
        {
            return Err(
                "Agent ID must contain 1–64 characters".to_string()
            );
        }

        if !definition.id.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || character == '_'
                || character == '-'
        }) {
            return Err(
                "Agent ID contains unsupported characters".to_string()
            );
        }

        if definition.name.trim().is_empty() {
            return Err("Agent name cannot be empty".to_string());
        }

        if definition.description.trim().is_empty() {
            return Err("Agent description cannot be empty".to_string());
        }

        if definition.version.trim().is_empty() {
            return Err("Agent version cannot be empty".to_string());
        }

        if self.agents.contains_key(&definition.id) {
            return Err(format!(
                "Agent is already registered: {}",
                definition.id
            ));
        }

        self.agents.insert(definition.id.clone(), definition);

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&AgentDefinition> {
        self.agents.get(id)
    }

    pub fn create_context(
        &self,
        agent_id: &str,
        request: &RequestContext,
    ) -> Result<AgentContext, String> {
        let definition = self
            .get(agent_id)
            .ok_or_else(|| format!("Agent is not registered: {agent_id}"))?;

        Ok(AgentContext::from_definition(definition, request))
    }

    pub fn definitions(&self) -> Vec<AgentDefinition> {
        let mut definitions: Vec<AgentDefinition> =
            self.agents.values().cloned().collect();

        definitions.sort_by(|a, b| a.id.cmp(&b.id));

        definitions
    }

    pub fn with_builtins() -> Result<Self, String> {
        let mut registry = Self::new();

        registry.register(AgentDefinition {
            id: "general".to_string(),
            name: "General Assistant".to_string(),
            description: "Handles everyday questions and general assistance."
                .to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![
                Capability::AgentToolExecute,
            ],
        })?;

        registry.register(AgentDefinition {
            id: "research".to_string(),
            name: "Research Agent".to_string(),
            description: "Handles research, information gathering, and analysis."
                .to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![
                Capability::AgentToolExecute,
            ],
        })?;

        registry.register(AgentDefinition {
            id: "coding".to_string(),
            name: "Coding Agent".to_string(),
            description: "Helps with programming, debugging, and code analysis."
                .to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![
                Capability::AgentToolExecute,
            ],
        })?;

        registry.register(AgentDefinition {
            id: "system".to_string(),
            name: "System Assistant".to_string(),
            description: "Provides assistance with operating system tasks."
                .to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![],
        })?;

        Ok(registry)
    }
}

#[cfg(test)]
mod tests {
    use super::{AgentDefinition, AgentRegistry};
    use crate::permissions::Capability;

    fn definition(id: &str) -> AgentDefinition {
        AgentDefinition {
            id: id.to_string(),
            name: "Test Agent".to_string(),
            description: "A test specialist agent.".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![Capability::AgentToolExecute],
        }
    }

    #[test]
    fn registers_and_retrieves_agent() {
        let mut registry = AgentRegistry::new();

        registry.register(definition("test")).unwrap();

        assert_eq!(
            registry.get("test").unwrap().name,
            "Test Agent"
        );
    }

    #[test]
    fn rejects_duplicate_agent_ids() {
        let mut registry = AgentRegistry::new();

        registry.register(definition("test")).unwrap();

        assert!(registry.register(definition("test")).is_err());
    }

    #[test]
    fn rejects_invalid_agent_ids() {
        let mut registry = AgentRegistry::new();

        assert!(registry.register(definition("invalid id")).is_err());
    }

    #[test]
    fn rejects_empty_agent_metadata() {
        let mut registry = AgentRegistry::new();
        let mut agent = definition("test");
        agent.description.clear();

        assert!(registry.register(agent).is_err());
    }

    #[test]
    fn returns_definitions_in_stable_order() {
        let mut registry = AgentRegistry::new();

        registry.register(definition("zeta")).unwrap();
        registry.register(definition("alpha")).unwrap();

        let definitions = registry.definitions();

        assert_eq!(definitions[0].id, "alpha");
        assert_eq!(definitions[1].id, "zeta");
    }

    #[test]
    fn registers_builtin_specialists() {
        let registry = AgentRegistry::with_builtins().unwrap();

        assert!(registry.get("general").is_some());
        assert!(registry.get("research").is_some());
        assert!(registry.get("coding").is_some());
        assert!(registry.get("system").is_some());

        assert!(
            registry.get("system").unwrap().capabilities.is_empty()
        );
    }
}
