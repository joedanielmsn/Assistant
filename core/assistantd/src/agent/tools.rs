use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use serde_json::{Value, json};

use crate::permissions::Capability;

pub type ToolHandler = Arc<dyn Fn(&Value) -> Result<Value, String> + Send + Sync>;

pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub argument_schema: Value,
    pub required_capability: Capability,
}

struct RegisteredTool {
    metadata: ToolMetadata,
    handler: ToolHandler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolExecutionStatus {
    Succeeded,
    Failed,
}

impl ToolExecutionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

pub struct ToolExecutionResult {
    pub tool_name: String,
    pub status: ToolExecutionStatus,
    pub success: bool,
    pub output: Value,
    pub error: Option<String>,
    pub duration_ms: u128,
}

#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, RegisteredTool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F>(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
        argument_schema: Value,
        required_capability: Capability,
        handler: F,
    ) -> Result<(), String>
    where
        F: Fn(&Value) -> Result<Value, String> + Send + Sync + 'static,
    {
        let name = name.into();
        let description = description.into();
        let version = version.into();

        if name.trim().is_empty() || name.len() > 64 {
            return Err("Tool name must contain 1–64 characters".to_string());
        }

        if !name.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        }) {
            return Err("Tool name contains unsupported characters".to_string());
        }

        if description.trim().is_empty() {
            return Err("Tool description cannot be empty".to_string());
        }

        if version.trim().is_empty() {
            return Err("Tool version cannot be empty".to_string());
        }

        validate_schema(&argument_schema)?;

        if self.tools.contains_key(&name) {
            return Err(format!("Tool is already registered: {name}"));
        }

        let metadata = ToolMetadata {
            name: name.clone(),
            description,
            version,
            argument_schema,
            required_capability,
        };

        self.tools.insert(
            name,
            RegisteredTool {
                metadata,
                handler: Arc::new(handler),
            },
        );

        Ok(())
    }

    pub fn with_builtins() -> Result<Self, String> {
        let mut registry = Self::new();

        registry.register(
            "calculator",
            "Evaluates basic arithmetic expressions with parentheses and decimals.",
            "1.0.0",
            json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "minLength": 1,
                        "maxLength": 256
                    }
                },
                "required": ["expression"],
                "additionalProperties": false
            }),
            Capability::AgentToolExecute,
            execute_calculator,
        )?;

        crate::agent::system_tools::register(&mut registry)?;

        Ok(registry)
    }

    pub fn definitions(&self) -> Vec<ToolMetadata> {
        let mut definitions: Vec<ToolMetadata> = self
            .tools
            .values()
            .map(|tool| ToolMetadata {
                name: tool.metadata.name.clone(),
                description: tool.metadata.description.clone(),
                version: tool.metadata.version.clone(),
                argument_schema: tool.metadata.argument_schema.clone(),
                required_capability: tool.metadata.required_capability,
            })
            .collect();

        definitions.sort_by(|a, b| a.name.cmp(&b.name));
        definitions
    }

    pub fn execute(
        &self,
        name: &str,
        arguments: &Value,
        granted_capabilities: &[Capability],
    ) -> Result<ToolExecutionResult, String> {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| format!("Tool is not registered: {name}"))?;

        if !granted_capabilities.contains(&tool.metadata.required_capability) {
            eprintln!("[tools] Permission denied: {name}");
            return Err(format!("Permission denied for tool: {name}"));
        }

        validate_arguments(&tool.metadata.argument_schema, arguments)?;

        eprintln!(
            "[tools] Executing {} v{}",
            tool.metadata.name, tool.metadata.version
        );

        let started = Instant::now();
        let handler_result = (tool.handler)(arguments);
        let duration_ms = started.elapsed().as_millis();

        let result = match handler_result {
            Ok(output) => {
                eprintln!("[tools] Execution successful: {name} ({duration_ms} ms)");

                ToolExecutionResult {
                    tool_name: name.to_string(),
                    status: ToolExecutionStatus::Succeeded,
                    success: true,
                    output,
                    error: None,
                    duration_ms,
                }
            }
            Err(error) => {
                eprintln!("[tools] Execution failed: {name} ({duration_ms} ms): {error}");

                ToolExecutionResult {
                    tool_name: name.to_string(),
                    status: ToolExecutionStatus::Failed,
                    success: false,
                    output: Value::Null,
                    error: Some(error),
                    duration_ms,
                }
            }
        };

        Ok(result)
    }
}

fn validate_schema(schema: &Value) -> Result<(), String> {
    validate_schema_node(schema, "root")
}

fn validate_schema_node(schema: &Value, location: &str) -> Result<(), String> {
    let object = schema
        .as_object()
        .ok_or_else(|| format!("Schema at {location} must be an object"))?;

    let schema_type = object
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("Schema at {location} must declare a type"))?;

    match schema_type {
        "string" => {
            validate_nonnegative_integer_keyword(object, "minLength", location)?;
            validate_nonnegative_integer_keyword(object, "maxLength", location)?;
            validate_integer_range(object, "minLength", "maxLength", location)?;
        }
        "number" | "integer" => {
            validate_numeric_keyword(object, "minimum", location)?;
            validate_numeric_keyword(object, "maximum", location)?;
            validate_numeric_range(object, "minimum", "maximum", location)?;
        }
        "boolean" => {}
        "object" => {
            let properties = object
                .get("properties")
                .and_then(Value::as_object)
                .ok_or_else(|| format!("Object schema at {location} must define properties"))?;

            if let Some(additional) = object.get("additionalProperties") {
                if !additional.is_boolean() {
                    return Err(format!(
                        "additionalProperties at {location} must be a boolean"
                    ));
                }
            }

            if let Some(required) = object.get("required") {
                let required = required
                    .as_array()
                    .ok_or_else(|| format!("required at {location} must be an array"))?;

                for field in required {
                    let name = field
                        .as_str()
                        .ok_or_else(|| format!("Invalid required field in schema at {location}"))?;

                    if !properties.contains_key(name) {
                        return Err(format!(
                            "Required field '{name}' has no property definition at {location}"
                        ));
                    }
                }
            }

            for (name, property_schema) in properties {
                validate_schema_node(property_schema, &format!("{location}.{name}"))?;
            }
        }
        "array" => {
            let items = object
                .get("items")
                .ok_or_else(|| format!("Array schema at {location} must define items"))?;

            validate_schema_node(items, &format!("{location}[]"))?;

            validate_nonnegative_integer_keyword(object, "minItems", location)?;
            validate_nonnegative_integer_keyword(object, "maxItems", location)?;
            validate_integer_range(object, "minItems", "maxItems", location)?;
        }
        other => {
            return Err(format!("Unsupported schema type '{other}' at {location}"));
        }
    }

    Ok(())
}

fn validate_nonnegative_integer_keyword(
    object: &serde_json::Map<String, Value>,
    keyword: &str,
    location: &str,
) -> Result<(), String> {
    if let Some(value) = object.get(keyword) {
        if value.as_u64().is_none() {
            return Err(format!(
                "{keyword} at {location} must be a nonnegative integer"
            ));
        }
    }

    Ok(())
}

fn validate_numeric_keyword(
    object: &serde_json::Map<String, Value>,
    keyword: &str,
    location: &str,
) -> Result<(), String> {
    if let Some(value) = object.get(keyword) {
        if value.as_f64().is_none() {
            return Err(format!("{keyword} at {location} must be a number"));
        }
    }

    Ok(())
}

fn validate_integer_range(
    object: &serde_json::Map<String, Value>,
    minimum_key: &str,
    maximum_key: &str,
    location: &str,
) -> Result<(), String> {
    if let (Some(minimum), Some(maximum)) = (
        object.get(minimum_key).and_then(Value::as_u64),
        object.get(maximum_key).and_then(Value::as_u64),
    ) {
        if minimum > maximum {
            return Err(format!(
                "{minimum_key} cannot exceed {maximum_key} at {location}"
            ));
        }
    }

    Ok(())
}

fn validate_numeric_range(
    object: &serde_json::Map<String, Value>,
    minimum_key: &str,
    maximum_key: &str,
    location: &str,
) -> Result<(), String> {
    if let (Some(minimum), Some(maximum)) = (
        object.get(minimum_key).and_then(Value::as_f64),
        object.get(maximum_key).and_then(Value::as_f64),
    ) {
        if minimum > maximum {
            return Err(format!(
                "{minimum_key} cannot exceed {maximum_key} at {location}"
            ));
        }
    }

    Ok(())
}

fn validate_arguments(schema: &Value, arguments: &Value) -> Result<(), String> {
    validate_value(schema, arguments, "arguments")
}

fn validate_value(schema: &Value, value: &Value, location: &str) -> Result<(), String> {
    let schema_type = schema["type"]
        .as_str()
        .ok_or_else(|| format!("Invalid schema type at {location}"))?;

    let valid_type = match schema_type {
        "string" => value.is_string(),
        "number" => value.is_number(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "boolean" => value.is_boolean(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        _ => false,
    };

    if !valid_type {
        return Err(format!(
            "Value at '{location}' must be of type {schema_type}"
        ));
    }

    match schema_type {
        "string" => {
            let text = value.as_str().unwrap();
            let length = text.chars().count();

            if let Some(min) = schema["minLength"].as_u64() {
                if length < min as usize {
                    return Err(format!("Value at '{location}' is too short"));
                }
            }

            if let Some(max) = schema["maxLength"].as_u64() {
                if length > max as usize {
                    return Err(format!("Value at '{location}' is too long"));
                }
            }
        }
        "number" | "integer" => {
            let number = value
                .as_f64()
                .ok_or_else(|| format!("Invalid number at {location}"))?;

            if let Some(minimum) = schema["minimum"].as_f64() {
                if number < minimum {
                    return Err(format!("Value at '{location}' is below its minimum"));
                }
            }

            if let Some(maximum) = schema["maximum"].as_f64() {
                if number > maximum {
                    return Err(format!("Value at '{location}' exceeds its maximum"));
                }
            }
        }
        "object" => {
            let object = value.as_object().unwrap();
            let properties = schema["properties"].as_object().unwrap();

            if schema["additionalProperties"].as_bool() == Some(false) {
                for key in object.keys() {
                    if !properties.contains_key(key) {
                        return Err(format!("Unexpected argument: {location}.{key}"));
                    }
                }
            }

            if let Some(required) = schema["required"].as_array() {
                for field in required {
                    let name = field.as_str().unwrap();

                    if !object.contains_key(name) {
                        return Err(format!("Missing required argument: {location}.{name}"));
                    }
                }
            }

            for (key, child_value) in object {
                if let Some(child_schema) = properties.get(key) {
                    validate_value(child_schema, child_value, &format!("{location}.{key}"))?;
                } else if schema["additionalProperties"].as_bool() == Some(false) {
                    return Err(format!("Unexpected argument: {location}.{key}"));
                }
            }
        }
        "array" => {
            let array = value.as_array().unwrap();

            if let Some(min) = schema["minItems"].as_u64() {
                if array.len() < min as usize {
                    return Err(format!("Array at '{location}' has too few items"));
                }
            }

            if let Some(max) = schema["maxItems"].as_u64() {
                if array.len() > max as usize {
                    return Err(format!("Array at '{location}' has too many items"));
                }
            }

            let item_schema = &schema["items"];

            for (index, item) in array.iter().enumerate() {
                validate_value(item_schema, item, &format!("{location}[{index}]"))?;
            }
        }
        "boolean" => {}
        _ => return Err(format!("Unsupported schema type at {location}")),
    }

    Ok(())
}

fn execute_calculator(arguments: &Value) -> Result<Value, String> {
    let expression = arguments
        .get("expression")
        .and_then(Value::as_str)
        .ok_or_else(|| "Calculator requires a string expression".to_string())?;

    let result = calculate(expression)?;

    Ok(json!({
        "expression": expression,
        "result": result
    }))
}

struct Parser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
        }
    }

    fn skip_spaces(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_ascii_whitespace() {
            self.position += 1;
        }
    }

    fn expression(&mut self, depth: usize) -> Result<f64, String> {
        let mut value = self.term(depth)?;

        loop {
            self.skip_spaces();

            let operator = match self.input.get(self.position) {
                Some(b'+') => '+',
                Some(b'-') => '-',
                _ => break,
            };

            self.position += 1;
            let rhs = self.term(depth)?;

            value = if operator == '+' {
                value + rhs
            } else {
                value - rhs
            };

            self.ensure_finite(value)?;
        }

        Ok(value)
    }

    fn term(&mut self, depth: usize) -> Result<f64, String> {
        let mut value = self.factor(depth)?;

        loop {
            self.skip_spaces();

            let operator = match self.input.get(self.position) {
                Some(b'*') => '*',
                Some(b'/') => '/',
                _ => break,
            };

            self.position += 1;
            let rhs = self.factor(depth)?;

            if operator == '/' && rhs == 0.0 {
                return Err("Division by zero is not allowed".to_string());
            }

            value = if operator == '*' {
                value * rhs
            } else {
                value / rhs
            };

            self.ensure_finite(value)?;
        }

        Ok(value)
    }

    fn factor(&mut self, depth: usize) -> Result<f64, String> {
        if depth > 32 {
            return Err("Expression nesting is too deep".to_string());
        }

        self.skip_spaces();

        match self.input.get(self.position) {
            Some(b'+') => {
                self.position += 1;
                self.factor(depth + 1)
            }
            Some(b'-') => {
                self.position += 1;
                Ok(-self.factor(depth + 1)?)
            }
            Some(b'(') => {
                self.position += 1;
                let value = self.expression(depth + 1)?;
                self.skip_spaces();

                if self.input.get(self.position) != Some(&b')') {
                    return Err("Missing closing parenthesis".to_string());
                }

                self.position += 1;
                Ok(value)
            }
            _ => self.number(),
        }
    }

    fn number(&mut self) -> Result<f64, String> {
        self.skip_spaces();

        let start = self.position;
        let mut digits = 0;

        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
            digits += 1;
        }

        if self.input.get(self.position) == Some(&b'.') {
            self.position += 1;

            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.position += 1;
                digits += 1;
            }
        }

        if digits == 0 {
            return Err("Expected a number".to_string());
        }

        let text = std::str::from_utf8(&self.input[start..self.position])
            .map_err(|_| "Invalid number".to_string())?;

        let value = text
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())?;

        self.ensure_finite(value)?;
        Ok(value)
    }

    fn ensure_finite(&self, value: f64) -> Result<(), String> {
        if value.is_finite() {
            Ok(())
        } else {
            Err("Calculation result is outside the supported range".to_string())
        }
    }
}

pub fn calculate(expression: &str) -> Result<f64, String> {
    if expression.trim().is_empty() || expression.len() > 256 {
        return Err("Expression must contain 1–256 bytes".to_string());
    }

    if !expression.is_ascii() {
        return Err("Only basic ASCII arithmetic expressions are supported".to_string());
    }

    let mut parser = Parser::new(expression);
    let result = parser.expression(0)?;

    parser.skip_spaces();

    if parser.position != parser.input.len() {
        return Err("Expression contains unsupported characters".to_string());
    }

    if !result.is_finite() {
        return Err("Calculation result is invalid".to_string());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn calculates_basic_arithmetic() {
        assert_eq!(calculate("2 + 3 * 4").unwrap(), 14.0);
    }

    #[test]
    fn calculates_parentheses_and_unary_numbers() {
        assert_eq!(calculate("-(5 + 3) * 2").unwrap(), -16.0);
    }

    #[test]
    fn rejects_division_by_zero() {
        assert!(calculate("10 / 0").is_err());
    }

    #[test]
    fn rejects_unsupported_input() {
        assert!(calculate("2 + unknown").is_err());
        assert!(calculate("2 ** 3").is_err());
    }
}

#[cfg(test)]
mod schema_tests {
    use super::{validate_arguments, validate_schema};
    use serde_json::json;

    #[test]
    fn accepts_valid_nested_object() {
        let schema = json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "minLength": 2},
                        "age": {"type": "integer", "minimum": 1, "maximum": 120}
                    },
                    "required": ["name"],
                    "additionalProperties": false
                }
            },
            "required": ["user"],
            "additionalProperties": false
        });

        let arguments = json!({
            "user": {
                "name": "Daniel",
                "age": 17
            }
        });

        assert!(validate_schema(&schema).is_ok());
        assert!(validate_arguments(&schema, &arguments).is_ok());
    }

    #[test]
    fn rejects_nested_missing_required_property() {
        let schema = json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"}
                    },
                    "required": ["name"],
                    "additionalProperties": false
                }
            },
            "required": ["user"]
        });

        let arguments = json!({"user": {}});

        assert!(validate_arguments(&schema, &arguments).is_err());
    }

    #[test]
    fn validates_array_item_types() {
        let schema = json!({
            "type": "object",
            "properties": {
                "values": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "minItems": 1,
                    "maxItems": 3
                }
            },
            "required": ["values"]
        });

        assert!(validate_arguments(&schema, &json!({"values": [1, 2, 3]})).is_ok());
        assert!(validate_arguments(&schema, &json!({"values": [1, "two"]})).is_err());
        assert!(validate_arguments(&schema, &json!({"values": []})).is_err());
        assert!(validate_arguments(&schema, &json!({"values": [1, 2, 3, 4]})).is_err());
    }

    #[test]
    fn enforces_numeric_boundaries() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 10
                }
            },
            "required": ["count"]
        });

        assert!(validate_arguments(&schema, &json!({"count": 1})).is_ok());
        assert!(validate_arguments(&schema, &json!({"count": 10})).is_ok());
        assert!(validate_arguments(&schema, &json!({"count": 0})).is_err());
        assert!(validate_arguments(&schema, &json!({"count": 11})).is_err());
    }

    #[test]
    fn rejects_unsupported_schema_types() {
        let schema = json!({
            "type": "object",
            "properties": {
                "value": {"type": "null"}
            }
        });

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn rejects_contradictory_string_length_boundaries() {
        let schema = json!({
            "type": "object",
            "properties": {
                "value": {
                    "type": "string",
                    "minLength": 10,
                    "maxLength": 3
                }
            }
        });

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn rejects_contradictory_numeric_boundaries() {
        let schema = json!({
            "type": "object",
            "properties": {
                "value": {
                    "type": "number",
                    "minimum": 10,
                    "maximum": 3
                }
            }
        });

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn rejects_contradictory_array_boundaries() {
        let schema = json!({
            "type": "object",
            "properties": {
                "values": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "minItems": 5,
                    "maxItems": 2
                }
            }
        });

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn allows_optional_properties_by_default() {
        let schema = json!({
            "type": "object",
            "properties": {
                "required_value": {"type": "string"},
                "optional_value": {"type": "boolean"}
            },
            "required": ["required_value"]
        });

        assert!(validate_arguments(&schema, &json!({"required_value": "hello"})).is_ok());
    }
}

#[cfg(test)]
mod registry_tests {
    use super::ToolRegistry;
    use crate::permissions::Capability;
    use serde_json::json;

    #[test]
    fn calculator_is_registered() {
        let registry = ToolRegistry::with_builtins().unwrap();
        let definitions = registry.definitions();

        assert!(definitions.iter().any(|tool| tool.name == "calculator"));
    }

    #[test]
    fn calculator_executes_with_permission() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry
            .execute(
                "calculator",
                &json!({"expression": "100 + 38"}),
                &[Capability::AgentToolExecute],
            )
            .unwrap();

        assert!(result.success);
        assert_eq!(result.tool_name, "calculator");
        assert_eq!(result.output["result"], 138.0);
    }

    #[test]
    fn calculator_rejects_missing_permission() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry.execute("calculator", &json!({"expression": "2 + 2"}), &[]);

        assert!(result.is_err());
    }

    #[test]
    fn calculator_rejects_missing_argument() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry.execute("calculator", &json!({}), &[Capability::AgentToolExecute]);

        assert!(result.is_err());
    }

    #[test]
    fn calculator_rejects_unexpected_argument() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry.execute(
            "calculator",
            &json!({
                "expression": "2 + 2",
                "extra": true
            }),
            &[Capability::AgentToolExecute],
        );

        assert!(result.is_err());
    }

    #[test]
    fn calculator_rejects_wrong_argument_type() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry.execute(
            "calculator",
            &json!({"expression": 42}),
            &[Capability::AgentToolExecute],
        );

        assert!(result.is_err());
    }

    #[test]
    fn unknown_tool_is_rejected() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry.execute("unknown_tool", &json!({}), &[Capability::AgentToolExecute]);

        assert!(result.is_err());
    }

    #[test]
    fn custom_tool_can_be_registered_and_executed() {
        let mut registry = ToolRegistry::new();

        registry
            .register(
                "example",
                "Returns a simple example result.",
                "1.0.0",
                json!({
                    "type": "object",
                    "properties": {
                        "message": {"type": "string"}
                    },
                    "required": ["message"],
                    "additionalProperties": false
                }),
                Capability::AgentToolExecute,
                |arguments| {
                    Ok(json!({
                        "received": arguments["message"]
                    }))
                },
            )
            .unwrap();

        assert!(
            registry
                .definitions()
                .iter()
                .any(|tool| tool.name == "example")
        );

        let result = registry
            .execute(
                "example",
                &json!({"message": "hello"}),
                &[Capability::AgentToolExecute],
            )
            .unwrap();

        assert_eq!(result.output["received"], "hello");
    }

    #[test]
    fn handler_failure_returns_structured_result() {
        let mut registry = ToolRegistry::new();

        registry
            .register(
                "failing_tool",
                "A tool that deliberately returns an error for testing.",
                "1.0.0",
                json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
                Capability::AgentToolExecute,
                |_| Err("simulated handler failure".to_string()),
            )
            .unwrap();

        let result = registry
            .execute("failing_tool", &json!({}), &[Capability::AgentToolExecute])
            .unwrap();

        assert!(!result.success);
        assert_eq!(result.status, super::ToolExecutionStatus::Failed);
        assert_eq!(result.error.as_deref(), Some("simulated handler failure"));
        assert!(result.output.is_null());
    }

    #[test]
    fn successful_execution_records_duration_and_status() {
        let registry = ToolRegistry::with_builtins().unwrap();

        let result = registry
            .execute(
                "calculator",
                &json!({"expression": "2 + 2"}),
                &[Capability::AgentToolExecute],
            )
            .unwrap();

        assert!(result.success);
        assert_eq!(result.status, super::ToolExecutionStatus::Succeeded);
        assert_eq!(result.error, None);
        assert_eq!(result.output["result"], 4.0);
        // A duration of zero milliseconds is valid for a fast tool.
        assert!(result.duration_ms < 60_000);
    }

    #[test]
    fn duplicate_tool_registration_is_rejected() {
        let mut registry = ToolRegistry::new();

        let register = || {
            json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            })
        };

        registry
            .register(
                "example",
                "Example tool",
                "1.0.0",
                register(),
                Capability::AgentToolExecute,
                |_| Ok(json!({})),
            )
            .unwrap();

        let result = registry.register(
            "example",
            "Duplicate tool",
            "1.0.0",
            register(),
            Capability::AgentToolExecute,
            |_| Ok(json!({})),
        );

        assert!(result.is_err());
    }
}
