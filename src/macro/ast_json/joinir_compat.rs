use nyash_rust::ast::{ASTNode, ContractKind};
use serde_json::{json, Value};

use super::shared;
mod constructors;
mod helpers;
mod json_to_ast;
use helpers::{attrs_to_json, literal_to_joinir_json};

pub fn ast_to_json(ast: &ASTNode) -> Value {
    match ast.clone() {
        ASTNode::Program { statements, .. } => json!({
            "kind": "Program",
            "statements": statements.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => json!({
            "kind": "BlockExpr",
            "prelude_stmts": prelude_stmts.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "tail_expr": ast_to_json(&tail_expr),
        }),
        ASTNode::BoxDeclaration {
            name,
            fields,
            field_decls,
            public_fields,
            private_fields,
            methods,
            constructors,
            init_fields,
            weak_fields,
            delegates,
            invariants,
            transitions,
            is_interface,
            is_record,
            extends,
            implements,
            type_parameters,
            is_sync,
            is_static,
            static_init,
            attrs,
            ..
        } => json!({
            "kind": "BoxDeclaration",
            "name": name,
            "fields": fields,
            "field_decls": field_decls.into_iter().map(|decl| json!({
                "name": decl.name,
                "declared_type": decl.declared_type_name,
                "is_weak": decl.is_weak,
                "default_value": decl.default_value.map(|expr| ast_to_json(&expr)),
            })).collect::<Vec<_>>(),
            "public_fields": public_fields,
            "private_fields": private_fields,
            "methods": methods
                .into_iter()
                .map(|(key, decl)| json!({"key": key, "decl": ast_to_json(&decl)}))
                .collect::<Vec<_>>(),
            "constructors": constructors
                .into_iter()
                .map(|(key, decl)| json!({"key": key, "decl": ast_to_json(&decl)}))
                .collect::<Vec<_>>(),
            "init_fields": init_fields,
            "weak_fields": weak_fields,
            "delegates": delegates.into_iter().map(|decl| json!({
                "field_name": decl.field_name,
                "exposes": decl.exposes.into_iter().map(|expose| json!({
                    "source_name": expose.source_name,
                    "exposed_name": expose.exposed_name,
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
            "invariants": invariants.into_iter().map(|expr| ast_to_json(&expr)).collect::<Vec<_>>(),
            "transitions": transitions.into_iter().map(|decl| json!({
                "from": decl.from_state,
                "to": decl.to_state,
                "method": decl.method_name,
            })).collect::<Vec<_>>(),
            "is_interface": is_interface,
            "is_record": is_record,
            "extends": extends,
            "implements": implements,
            "type_parameters": type_parameters,
            "is_sync": is_sync,
            "is_static": is_static,
            "static_init": static_init.map(|stmts| stmts.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()),
            "attrs": attrs_to_json(&attrs),
        }),
        ASTNode::EnumDeclaration {
            name,
            variants,
            type_parameters,
            attrs,
            ..
        } => json!({
            "kind": "EnumDeclaration",
            "name": name,
            "variants": variants.into_iter().map(|variant| json!({
                "name": variant.name,
                "payload_type": variant.payload_type_name,
                "tuple_payload_types": variant.tuple_payload_type_names,
                "record_fields": variant.record_field_decls.into_iter().map(|field| json!({
                    "name": field.name,
                    "declared_type": field.declared_type_name,
                    "is_weak": field.is_weak,
                    "default_value": field.default_value.map(|expr| ast_to_json(&expr)),
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
            "type_parameters": type_parameters,
            "attrs": attrs_to_json(&attrs),
        }),
        ASTNode::BrandDeclaration {
            name,
            underlying_type_name,
            ..
        } => json!({
            "kind": "BrandDeclaration",
            "name": name,
            "underlying_type": underlying_type_name,
        }),
        ASTNode::TypeAliasDeclaration {
            name,
            target_type_name,
            ..
        } => json!({
            "kind": "TypeAliasDeclaration",
            "name": name,
            "target_type": target_type_name,
        }),
        // Phase 54: Loop with JoinIR-compatible fields
        ASTNode::Loop {
            condition, body, ..
        } => json!({
            "kind": "Loop",
            "type": "Loop",  // JoinIR Frontend expects "type"
            "condition": ast_to_json(&condition),
            "cond": ast_to_json(&condition),  // JoinIR expects "cond"
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        ASTNode::LoopRange {
            var_name,
            start,
            end,
            body,
            ..
        } => json!({
            "kind": "LoopRange",
            "type": "LoopRange",
            "var_name": var_name,
            "start": ast_to_json(&start),
            "end": ast_to_json(&end),
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        ASTNode::TaskScope {
            body,
            source_keyword,
            ..
        } => json!({
            "kind": "TaskScope",
            "type": "TaskScope",
            "spelling": source_keyword,
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        ASTNode::ContextScope {
            name,
            declared_type_name,
            value,
            body,
            source_keyword,
            ..
        } => json!({
            "kind": "ContextScope",
            "type": "ContextScope",
            "spelling": source_keyword,
            "name": name,
            "declared_type": declared_type_name,
            "value": ast_to_json(&value),
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        ASTNode::FastMemRegion { contract, body, .. } => json!({
            "kind": "FastMemRegion",
            "type": "FastMemRegion",
            "contract": contract,
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        // Phase 54: Print with JoinIR-compatible fields
        ASTNode::Print { expression, .. } => json!({
            "kind": "Print",
            "type": "Print",  // JoinIR Frontend expects "type"
            "expression": ast_to_json(&expression),
            "expr": ast_to_json(&expression),  // JoinIR expects "expr"
        }),
        // Phase 54: Return with JoinIR-compatible fields
        ASTNode::Return { value, .. } => json!({
            "kind": "Return",
            "type": "Return",  // JoinIR Frontend expects "type"
            "value": value.as_ref().map(|v| ast_to_json(v)),
        }),
        // Phase 56: Break with JoinIR-compatible type field
        ASTNode::Break { .. } => json!({
            "kind": "Break",
            "type": "Break"  // JoinIR Frontend expects "type"
        }),
        // Phase 56: Continue with JoinIR-compatible type field
        ASTNode::Continue { .. } => json!({
            "kind": "Continue",
            "type": "Continue"  // JoinIR Frontend expects "type"
        }),
        // Phase 54: Assignment with JoinIR-compatible fields
        ASTNode::Assignment { target, value, .. } => {
            // Extract variable name if target is a simple Variable
            let target_str = match target.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => "complex".to_string(), // FieldAccess or other complex target
            };
            json!({
                "kind": "Assignment",
                "type": "Assignment",  // JoinIR Frontend expects "type"
                "target": target_str,  // JoinIR expects string variable name
                "lhs": ast_to_json(&target),  // Keep full AST for complex cases
                "value": ast_to_json(&value),
                "expr": ast_to_json(&value),  // JoinIR expects "expr"
            })
        }
        // Phase 54: Local with JoinIR-compatible fields
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } => {
            // For single-variable declarations, add "name" and "expr" for JoinIR compatibility
            let (name, expr) = if variables.len() == 1 {
                let n = variables[0].clone();
                let e = initial_values
                    .get(0)
                    .and_then(|opt| opt.as_ref())
                    .map(|v| ast_to_json(v));
                (Some(n), e)
            } else {
                (None, None)
            };

            let inits: Vec<_> = initial_values
                .into_iter()
                .map(|opt| opt.map(|v| ast_to_json(&v)))
                .collect();
            let declared_type = if variables.len() == 1 {
                declared_type_names.get(0).cloned().flatten()
            } else {
                None
            };

            json!({
                "kind": "Local",
                "type": "Local",  // JoinIR Frontend expects "type"
                "name": name,  // Single variable name for JoinIR (null if multiple)
                "expr": expr,  // Single variable init for JoinIR (null if multiple)
                "declared_type": declared_type,
                "declared_type_names": declared_type_names,
                "variables": variables,
                "inits": inits
            })
        }
        // Phase 54: If with JoinIR-compatible fields
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => json!({
            "kind": "If",
            "type": "If",  // JoinIR Frontend expects "type"
            "condition": ast_to_json(&condition),
            "cond": ast_to_json(&condition),  // JoinIR expects "cond"
            "then": then_body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "else": else_body.map(|v| v.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()),
        }),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => json!({
            "kind": "TryCatch",
            "try": try_body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "catch": catch_clauses.into_iter().map(|cc| json!({
                "type": cc.exception_type,
                "var": cc.variable_name,
                "body": cc.body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
            })).collect::<Vec<_>>(),
            "cleanup": finally_body.map(|v| v.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>())
        }),
        ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } => json!({
            "kind": "FunctionDeclaration",
            "name": name,
            "params": params,
            "param_decls": shared::param_decls_to_json(&param_decls, &params),
            "return_type": return_type_name,
            "uses": uses,
            "contracts": contracts.into_iter().map(|clause| json!({
                "kind": match clause.kind {
                    ContractKind::Requires => "requires",
                    ContractKind::Ensures => "ensures",
                },
                "condition": ast_to_json(&clause.condition),
            })).collect::<Vec<_>>(),
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "static": is_static,
            "override": is_override,
            "attrs": attrs_to_json(&attrs),
        }),
        // Phase 52: Variable → Var ノード（JoinIR Frontend 互換）
        ASTNode::Variable { name, .. } => json!({
            "kind": "Variable",
            "type": "Var",  // JoinIR Frontend expects "type": "Var"
            "name": name
        }),
        // Phase 52: Literal → Int/Bool/String ノード（JoinIR Frontend 互換）
        ASTNode::Literal { value, .. } => literal_to_joinir_json(&value),
        // Phase 52: BinaryOp → Binary/Compare ノード（JoinIR Frontend 互換）
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let op_str = shared::bin_to_str(&operator);
            // JoinIR Frontend distinguishes between Binary (arithmetic) and Compare
            let type_str = if shared::is_compare_op(&operator) {
                "Compare"
            } else {
                "Binary"
            };
            json!({
                "kind": "BinaryOp",
                "type": type_str,
                "op": op_str,
                // JoinIR Frontend expects "lhs"/"rhs" not "left"/"right"
                "lhs": ast_to_json(&left),
                "rhs": ast_to_json(&right),
                // Also keep "left"/"right" for backward compatibility
                "left": ast_to_json(&left),
                "right": ast_to_json(&right),
            })
        }
        // Phase 56: UnaryOp → Unary ノード（JoinIR Frontend 互換）
        ASTNode::UnaryOp {
            operator, operand, ..
        } => json!({
            "kind": "UnaryOp",
            "type": "Unary",  // Phase 56: JoinIR Frontend expects "type" field
            "op": shared::un_to_str(&operator),
            "operand": ast_to_json(&operand),
        }),
        // Phase 52: MethodCall → Method ノード（JoinIR Frontend 互換）
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => json!({
            "kind": "MethodCall",
            "type": "Method",  // JoinIR Frontend expects "type": "Method"
            // JoinIR Frontend expects "receiver" not "object"
            "receiver": ast_to_json(&object),
            "object": ast_to_json(&object),  // Keep for backward compatibility
            "method": method,
            // JoinIR Frontend expects "args" not "arguments"
            "args": arguments.iter().map(|a| ast_to_json(a)).collect::<Vec<_>>(),
            "arguments": arguments.into_iter().map(|a| ast_to_json(&a)).collect::<Vec<_>>()  // Keep for backward compatibility
        }),
        // Phase 56: FunctionCall with JoinIR-compatible type field
        ASTNode::FunctionCall {
            name, arguments, ..
        } => json!({
            "kind": "FunctionCall",
            "type": "Call",  // JoinIR Frontend expects "type": "Call"
            "name": name,
            "func": name.clone(),  // JoinIR expects "func" for function name
            "args": arguments.iter().map(|a| ast_to_json(a)).collect::<Vec<_>>(),  // JoinIR expects "args"
            "arguments": arguments.into_iter().map(|a| ast_to_json(&a)).collect::<Vec<_>>()  // Keep for backward compatibility
        }),
        // Phase 56: ArrayLiteral with JoinIR-compatible type field
        ASTNode::ArrayLiteral { elements, .. } => json!({
            "kind": "Array",
            "type": "Array",  // JoinIR Frontend expects "type"
            "elements": elements.into_iter().map(|e| ast_to_json(&e)).collect::<Vec<_>>()
        }),
        // Phase 56: MapLiteral with JoinIR-compatible type field
        ASTNode::MapLiteral { entries, .. } => json!({
            "kind": "Map",
            "type": "Map",  // JoinIR Frontend expects "type"
            "entries": entries.into_iter().map(|(k,v)| json!({"k":k,"v":ast_to_json(&v)})).collect::<Vec<_>>()
        }),
        ASTNode::RecordLiteral {
            record_type_name,
            fields,
            ..
        } => json!({
            "kind": "RecordLiteral",
            "record_type": record_type_name,
            "fields": fields.into_iter().map(|(k,v)| json!({"name":k,"value":ast_to_json(&v)})).collect::<Vec<_>>()
        }),
        ASTNode::RecordUpdate { base, updates, .. } => json!({
            "kind": "RecordUpdate",
            "base": ast_to_json(&base),
            "updates": updates.into_iter().map(|(k,v)| json!({"name":k,"value":ast_to_json(&v)})).collect::<Vec<_>>()
        }),
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => json!({
        "kind":"MatchExpr",
        "scrutinee": ast_to_json(&scrutinee),
            "arms": arms.into_iter().map(|(lit, body)| json!({
                "literal": {
                    "kind": "Literal",
                    "value": shared::lit_to_json(&lit)
                },
                "body": ast_to_json(&body)
            })).collect::<Vec<_>>(),
            "else": ast_to_json(&else_expr),
        }),
        ASTNode::EnumMatchExpr {
            enum_name,
            scrutinee,
            arms,
            else_expr,
            ..
        } => json!({
            "kind":"EnumMatchExpr",
            "enum_name": enum_name,
            "scrutinee": ast_to_json(&scrutinee),
            "arms": arms.into_iter().map(|arm| json!({
                "variant_name": arm.variant_name,
                "binding_name": arm.binding_name,
                "body": ast_to_json(&arm.body)
            })).collect::<Vec<_>>(),
            "else": else_expr.as_ref().map(|expr| ast_to_json(expr)),
        }),
        // Phase 52: FieldAccess → Field ノード（JoinIR Frontend 互換）
        ASTNode::FieldAccess { object, field, .. } => json!({
            "kind": "FieldAccess",
            "type": "Field",  // JoinIR Frontend expects "type": "Field"
            "object": ast_to_json(&object),
            "field": field
        }),
        ASTNode::Index { target, index, .. } => json!({
            "kind": "Index",
            "type": "Index",
            "target": ast_to_json(&target),
            "index": ast_to_json(&index)
        }),
        // Phase 52: Me → Var("me") ノード（JoinIR Frontend 互換）
        ASTNode::Me { .. } => json!({
            "kind": "Me",
            "type": "Var",  // JoinIR Frontend expects "type": "Var"
            "name": "me"
        }),
        // Phase 52: New → NewBox ノード（JoinIR Frontend 互換）
        ASTNode::New {
            class,
            arguments,
            field_initializers,
            ..
        } => json!({
            "kind": "New",
            "type": "NewBox",  // JoinIR Frontend expects "type": "NewBox"
            "box_name": class,
            "args": arguments.into_iter().map(|a| ast_to_json(&a)).collect::<Vec<_>>(),
            "field_initializers": field_initializers
                .into_iter()
                .map(|(name, expr)| json!({"field": name, "value": ast_to_json(&expr)}))
                .collect::<Vec<_>>()
        }),
        other => json!({"kind":"Unsupported","debug": format!("{:?}", other)}),
    }
}
