//! Strict, read-only ProgramV0 body validation before Hako snapshot traversal.

use super::schema::WireClassificationV0;
use super::strict_json::{parse_strict_json, StrictJsonValue};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramV0BodyViewError {
    InvalidInput {
        path: String,
        reason: String,
    },
    Unsupported {
        path: String,
        node_kind: String,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedProgramV0BodyView {
    pub(super) root: StrictJsonValue,
}

impl ValidatedProgramV0BodyView {
    pub fn body_len(&self) -> usize {
        self.root
            .object_field("body")
            .and_then(StrictJsonValue::array_items)
            .map_or(0, <[StrictJsonValue]>::len)
    }
}

pub fn read_program_v0_body(
    input: &str,
) -> Result<ValidatedProgramV0BodyView, ProgramV0BodyViewError> {
    let root = parse_strict_json(input).map_err(|reason| invalid("$", reason))?;
    validate_root(&root)?;
    Ok(ValidatedProgramV0BodyView { root })
}

fn validate_root(root: &StrictJsonValue) -> Result<(), ProgramV0BodyViewError> {
    require_object(root, "$", &ROOT_FIELDS)?;
    let version = required(root, "version", "$.version")?;
    if version.json_integer_i64() != Some(0) {
        return Err(invalid("$.version", "program.version_must_be_zero"));
    }
    let kind = required(root, "kind", "$.kind")?;
    if kind.string() != Some("Program") {
        return Err(invalid("$.kind", "program.kind_must_be_program"));
    }
    let body = required(root, "body", "$.body")?;
    let items = body
        .array_items()
        .ok_or_else(|| wrong_type("$.body", "array", body))?;
    for (index, statement) in items.iter().enumerate() {
        validate_stmt(statement, &format!("$.body[{index}]"))?;
    }
    validate_optional_root_fields(root)?;
    Ok(())
}

fn validate_optional_root_fields(root: &StrictJsonValue) -> Result<(), ProgramV0BodyViewError> {
    for name in ["attrs", "imports"] {
        if let Some(value) = root.object_field(name) {
            require_object_shape(value, &format!("$.{name}"))?;
        }
    }
    for name in [
        "defs",
        "user_box_decls",
        "record_decls",
        "enum_decls",
        "static_data_plans",
        "static_table_contract_specs",
        "brand_decls",
        "type_alias_decls",
    ] {
        if let Some(value) = root.object_field(name) {
            value
                .array_items()
                .ok_or_else(|| wrong_type(format!("$.{name}"), "array", value))?;
        }
    }
    Ok(())
}

fn validate_stmt(value: &StrictJsonValue, path: &str) -> Result<(), ProgramV0BodyViewError> {
    let kind = node_kind(value, path)?;
    match classify_stmt_kind(kind) {
        Some(WireClassificationV0::Accepted) => {}
        Some(WireClassificationV0::KnownUnsupported) => {
            return Err(unsupported(path, kind, "unsupported.wire_kind"))
        }
        Some(WireClassificationV0::SchemaMismatchStop) => {
            return Err(unsupported(path, kind, "transport.schema_mismatch_stop"))
        }
        None => return Err(invalid(path, format!("unknown statement tag: {kind}"))),
    }
    match kind {
        "Local" => {
            require_object(value, path, &["type", "name", "expr", "declared_type"])?;
            require_string(value, "name", &format!("{path}.name"))?;
            validate_optional_string(value, "declared_type", &format!("{path}.declared_type"))?;
            validate_expr(
                required(value, "expr", &format!("{path}.expr"))?,
                &format!("{path}.expr"),
            )
        }
        "Expr" => {
            require_object(value, path, &["type", "expr"])?;
            validate_expr(
                required(value, "expr", &format!("{path}.expr"))?,
                &format!("{path}.expr"),
            )
        }
        "If" => {
            require_object(value, path, &["type", "cond", "then", "else"])?;
            validate_expr(
                required(value, "cond", &format!("{path}.cond"))?,
                &format!("{path}.cond"),
            )?;
            validate_body(
                required(value, "then", &format!("{path}.then"))?,
                &format!("{path}.then"),
            )?;
            if let Some(otherwise) = value.object_field("else") {
                if !matches!(otherwise, StrictJsonValue::Null) {
                    validate_body(otherwise, &format!("{path}.else"))?;
                }
            }
            Ok(())
        }
        "Loop" => {
            require_object(value, path, &["type", "cond", "body"])?;
            validate_expr(
                required(value, "cond", &format!("{path}.cond"))?,
                &format!("{path}.cond"),
            )?;
            validate_body(
                required(value, "body", &format!("{path}.body"))?,
                &format!("{path}.body"),
            )
        }
        "LoopRange" => {
            require_object(value, path, &["type", "var_name", "start", "end", "body"])?;
            require_string(value, "var_name", &format!("{path}.var_name"))?;
            validate_expr(
                required(value, "start", &format!("{path}.start"))?,
                &format!("{path}.start"),
            )?;
            validate_expr(
                required(value, "end", &format!("{path}.end"))?,
                &format!("{path}.end"),
            )?;
            validate_body(
                required(value, "body", &format!("{path}.body"))?,
                &format!("{path}.body"),
            )
        }
        "Return" => {
            require_object(value, path, &["type", "expr"])?;
            validate_expr(
                required(value, "expr", &format!("{path}.expr"))?,
                &format!("{path}.expr"),
            )
        }
        "Break" | "Continue" => require_object(value, path, &["type"]),
        _ => unreachable!("accepted statement classification must be exhaustive"),
    }
}

fn validate_expr(value: &StrictJsonValue, path: &str) -> Result<(), ProgramV0BodyViewError> {
    let kind = node_kind(value, path)?;
    match classify_expr_kind(kind) {
        Some(WireClassificationV0::Accepted) => {}
        Some(WireClassificationV0::KnownUnsupported) => {
            return Err(unsupported(path, kind, "unsupported.wire_kind"))
        }
        Some(WireClassificationV0::SchemaMismatchStop) => {
            return Err(unsupported(path, kind, "transport.schema_mismatch_stop"))
        }
        None => return Err(invalid(path, format!("unknown expression tag: {kind}"))),
    }
    let unary = match kind {
        "Int" => {
            require_object(value, path, &["type", "value", "declared_type"])?;
            validate_optional_string(value, "declared_type", &format!("{path}.declared_type"))?;
            let scalar = required(value, "value", &format!("{path}.value"))?;
            if scalar.exact_i64().is_none() {
                return Err(invalid(format!("{path}.value"), "int.not_canonical_i64"));
            }
            return Ok(());
        }
        "Str" => &["type", "value"][..],
        "Bool" => &["type", "value"][..],
        "Null" => &["type"][..],
        "Var" => &["type", "name"][..],
        "Call" => &["type", "name", "args"][..],
        "Method" => &["type", "recv", "method", "args"][..],
        "Field" => &["type", "recv", "field"][..],
        "Binary" | "Compare" | "Logical" => &["type", "op", "lhs", "rhs"][..],
        _ => unreachable!("accepted expression classification must be exhaustive"),
    };
    require_object(value, path, unary)?;
    match kind {
        "Str" => require_string(value, "value", &format!("{path}.value")),
        "Bool" => require_bool(value, "value", &format!("{path}.value")),
        "Null" => Ok(()),
        "Var" => require_string(value, "name", &format!("{path}.name")),
        "Call" => {
            require_string(value, "name", &format!("{path}.name"))?;
            validate_args(
                required(value, "args", &format!("{path}.args"))?,
                &format!("{path}.args"),
            )
        }
        "Method" => {
            validate_expr(
                required(value, "recv", &format!("{path}.recv"))?,
                &format!("{path}.recv"),
            )?;
            require_string(value, "method", &format!("{path}.method"))?;
            validate_args(
                required(value, "args", &format!("{path}.args"))?,
                &format!("{path}.args"),
            )
        }
        "Field" => {
            validate_expr(
                required(value, "recv", &format!("{path}.recv"))?,
                &format!("{path}.recv"),
            )?;
            require_string(value, "field", &format!("{path}.field"))
        }
        "Binary" | "Compare" | "Logical" => {
            let operator = require_string_value(value, "op", &format!("{path}.op"))?;
            if !operator_allowed(kind, operator) {
                return Err(invalid(format!("{path}.op"), "operator.invalid_for_kind"));
            }
            validate_expr(
                required(value, "lhs", &format!("{path}.lhs"))?,
                &format!("{path}.lhs"),
            )?;
            validate_expr(
                required(value, "rhs", &format!("{path}.rhs"))?,
                &format!("{path}.rhs"),
            )
        }
        _ => unreachable!(),
    }
}

fn validate_body(value: &StrictJsonValue, path: &str) -> Result<(), ProgramV0BodyViewError> {
    let items = value
        .array_items()
        .ok_or_else(|| wrong_type(path, "array", value))?;
    for (index, statement) in items.iter().enumerate() {
        validate_stmt(statement, &format!("{path}[{index}]"))?;
    }
    Ok(())
}

fn validate_args(value: &StrictJsonValue, path: &str) -> Result<(), ProgramV0BodyViewError> {
    let items = value
        .array_items()
        .ok_or_else(|| wrong_type(path, "array", value))?;
    for (index, expression) in items.iter().enumerate() {
        validate_expr(expression, &format!("{path}[{index}]"))?;
    }
    Ok(())
}

fn node_kind<'a>(
    value: &'a StrictJsonValue,
    path: &str,
) -> Result<&'a str, ProgramV0BodyViewError> {
    require_object_shape(value, path)?;
    let field = required(value, "type", &format!("{path}.type"))?;
    field
        .string()
        .ok_or_else(|| wrong_type(format!("{path}.type"), "string", field))
}

fn require_object(
    value: &StrictJsonValue,
    path: &str,
    allowed: &[&str],
) -> Result<(), ProgramV0BodyViewError> {
    let fields = require_object_shape(value, path)?;
    let allowed: HashSet<&str> = allowed.iter().copied().collect();
    if let Some((name, _)) = fields
        .iter()
        .find(|(name, _)| !allowed.contains(name.as_str()))
    {
        return Err(invalid(
            format!("{path}.{name}"),
            "object.forbidden_unknown_field",
        ));
    }
    Ok(())
}

fn require_object_shape<'a>(
    value: &'a StrictJsonValue,
    path: &str,
) -> Result<&'a [(String, StrictJsonValue)], ProgramV0BodyViewError> {
    value
        .object_fields()
        .ok_or_else(|| wrong_type(path, "object", value))
}

fn required<'a>(
    value: &'a StrictJsonValue,
    name: &str,
    path: &str,
) -> Result<&'a StrictJsonValue, ProgramV0BodyViewError> {
    value
        .object_field(name)
        .ok_or_else(|| invalid(path, "object.required_field_missing"))
}

fn require_string(
    value: &StrictJsonValue,
    name: &str,
    path: &str,
) -> Result<(), ProgramV0BodyViewError> {
    require_string_value(value, name, path)?;
    Ok(())
}

fn require_string_value<'a>(
    value: &'a StrictJsonValue,
    name: &str,
    path: &str,
) -> Result<&'a str, ProgramV0BodyViewError> {
    let field = required(value, name, path)?;
    field
        .string()
        .ok_or_else(|| wrong_type(path, "string", field))
}

fn validate_optional_string(
    value: &StrictJsonValue,
    name: &str,
    path: &str,
) -> Result<(), ProgramV0BodyViewError> {
    if let Some(field) = value.object_field(name) {
        if !matches!(field, StrictJsonValue::Null | StrictJsonValue::String(_)) {
            return Err(wrong_type(path, "string_or_null", field));
        }
    }
    Ok(())
}

fn require_bool(
    value: &StrictJsonValue,
    name: &str,
    path: &str,
) -> Result<(), ProgramV0BodyViewError> {
    let field = required(value, name, path)?;
    if !matches!(field, StrictJsonValue::Bool(_)) {
        return Err(wrong_type(path, "bool", field));
    }
    Ok(())
}

const ROOT_FIELDS: [&str; 13] = [
    "version",
    "kind",
    "body",
    "attrs",
    "defs",
    "imports",
    "user_box_decls",
    "record_decls",
    "enum_decls",
    "static_data_plans",
    "static_table_contract_specs",
    "brand_decls",
    "type_alias_decls",
];

fn wrong_type(
    path: impl Into<String>,
    expected: &str,
    actual: &StrictJsonValue,
) -> ProgramV0BodyViewError {
    invalid(
        path,
        format!("type.expected_{expected}.got_{}", actual.kind_name()),
    )
}

fn invalid(path: impl Into<String>, reason: impl Into<String>) -> ProgramV0BodyViewError {
    ProgramV0BodyViewError::InvalidInput {
        path: path.into(),
        reason: reason.into(),
    }
}

fn unsupported(path: &str, node_kind: &str, reason: &str) -> ProgramV0BodyViewError {
    ProgramV0BodyViewError::Unsupported {
        path: path.to_owned(),
        node_kind: node_kind.to_owned(),
        reason: reason.to_owned(),
    }
}

fn classify_stmt_kind(kind: &str) -> Option<WireClassificationV0> {
    Some(match kind {
        "Local" | "Expr" | "If" | "Loop" | "LoopRange" | "Return" | "Break" | "Continue" => {
            WireClassificationV0::Accepted
        }
        "Extern" | "TaskScope" | "Throw" | "Try" | "FiniReg" => {
            WireClassificationV0::KnownUnsupported
        }
        "FastMemRegion" => WireClassificationV0::SchemaMismatchStop,
        _ => return None,
    })
}

fn classify_expr_kind(kind: &str) -> Option<WireClassificationV0> {
    Some(match kind {
        "Int" | "Str" | "Bool" | "Null" | "Var" | "Binary" | "Compare" | "Logical" | "Call"
        | "Method" | "Field" => WireClassificationV0::Accepted,
        "Extern" | "ArrayLiteral" | "New" | "Throw" | "BlockExpr" | "Ternary" | "Match"
        | "EnumCtor" | "EnumMatch" => WireClassificationV0::KnownUnsupported,
        "Float" | "BrandConstruct" | "BrandUnwrap" | "RecordField" | "RecordLiteral"
        | "RecordUpdate" => WireClassificationV0::SchemaMismatchStop,
        _ => return None,
    })
}

fn operator_allowed(kind: &str, operator: &str) -> bool {
    match kind {
        "Binary" => matches!(
            operator,
            "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>"
        ),
        "Compare" => matches!(operator, "==" | "!=" | "<" | ">" | "<=" | ">="),
        "Logical" => matches!(operator, "&&" | "||"),
        _ => false,
    }
}
