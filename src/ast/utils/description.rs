use crate::ast::ASTNode;
use std::fmt;

impl ASTNode {
    /// AST nodeの詳細情報を取得 (デバッグ用)
    pub fn info(&self) -> String {
        match self {
            ASTNode::Program { statements, .. } => {
                format!("Program({} statements)", statements.len())
            }
            ASTNode::Assignment { target, .. } => {
                format!("Assignment(target: {})", target.info())
            }
            ASTNode::Print { .. } => "Print".to_string(),
            ASTNode::If { .. } => "If".to_string(),
            ASTNode::Loop {
                condition: _, body, ..
            } => {
                format!("Loop({} statements)", body.len())
            }
            ASTNode::LoopRange {
                var_name,
                start: _,
                end: _,
                body,
                ..
            } => {
                format!("LoopRange(var={}, {} statements)", var_name, body.len())
            }
            ASTNode::Return { value, .. } => {
                if value.is_some() {
                    "Return(with value)".to_string()
                } else {
                    "Return(void)".to_string()
                }
            }
            ASTNode::Break { .. } => "Break".to_string(),
            ASTNode::Continue { .. } => "Continue".to_string(),
            ASTNode::UsingStatement { namespace_name, .. } => {
                format!("UsingStatement({})", namespace_name)
            }
            ASTNode::ImportStatement { path, alias, .. } => {
                if let Some(a) = alias {
                    format!("ImportStatement({}, as {})", path, a)
                } else {
                    format!("ImportStatement({})", path)
                }
            }
            ASTNode::BuildGate {
                predicate,
                then_items,
                else_items,
                ..
            } => {
                let else_count = else_items.as_ref().map_or(0, Vec::len);
                format!(
                    "BuildGate({:?}, then={}, else={})",
                    predicate,
                    then_items.len(),
                    else_count
                )
            }
            ASTNode::BoxDeclaration {
                name,
                fields,
                methods,
                constructors,
                is_interface,
                is_record,
                is_sync,
                extends,
                implements,
                ..
            } => {
                let mut desc = if *is_record {
                    format!("RecordDeclaration({}, {} fields", name, fields.len())
                } else if *is_interface {
                    format!("InterfaceBox({}, {} methods", name, methods.len())
                } else if *is_sync {
                    format!(
                        "SyncBoxDeclaration({}, {} fields, {} methods, {} constructors",
                        name,
                        fields.len(),
                        methods.len(),
                        constructors.len()
                    )
                } else {
                    format!(
                        "BoxDeclaration({}, {} fields, {} methods, {} constructors",
                        name,
                        fields.len(),
                        methods.len(),
                        constructors.len()
                    )
                };

                if !extends.is_empty() {
                    desc.push_str(&format!(", extends [{}]", extends.join(", ")));
                }

                if !implements.is_empty() {
                    desc.push_str(&format!(", implements [{}]", implements.join(", ")));
                }

                desc.push(')');
                desc
            }
            ASTNode::EnumDeclaration {
                name,
                variants,
                type_parameters,
                ..
            } => {
                if type_parameters.is_empty() {
                    format!("EnumDeclaration({}, {} variants)", name, variants.len())
                } else {
                    format!(
                        "EnumDeclaration({}<{}>, {} variants)",
                        name,
                        type_parameters.join(", "),
                        variants.len()
                    )
                }
            }
            ASTNode::BrandDeclaration {
                name,
                underlying_type_name,
                ..
            } => format!("BrandDeclaration({}: {})", name, underlying_type_name),
            ASTNode::TypeAliasDeclaration {
                name,
                target_type_name,
                ..
            } => format!("TypeAliasDeclaration({} = {})", name, target_type_name),
            ASTNode::FunctionDeclaration {
                name,
                params,
                body,
                is_static,
                is_override,
                ..
            } => {
                let static_str = if *is_static { "static " } else { "" };
                let override_str = if *is_override { "override " } else { "" };
                format!(
                    "FunctionDeclaration({}{}{}({}), {} statements)",
                    override_str,
                    static_str,
                    name,
                    params.join(", "),
                    body.len()
                )
            }
            ASTNode::GlobalVar { name, .. } => {
                format!("GlobalVar({})", name)
            }
            ASTNode::StaticConstTable {
                name,
                element_type,
                values,
                ..
            } => {
                format!(
                    "StaticConstTable({}: {}[{}])",
                    name,
                    element_type,
                    values.len()
                )
            }
            ASTNode::Literal { .. } => "Literal".to_string(),
            ASTNode::Variable { name, .. } => {
                format!("Variable({})", name)
            }
            ASTNode::UnaryOp { operator, .. } => {
                format!("UnaryOp({})", operator)
            }
            ASTNode::BinaryOp { operator, .. } => {
                format!("BinaryOp({})", operator)
            }
            ASTNode::CheckExpr { name, items, .. } => {
                let name = name.as_deref().unwrap_or("<anonymous>");
                format!("CheckExpr({}, {} items)", name, items.len())
            }
            ASTNode::MethodCall {
                method, arguments, ..
            } => {
                format!("MethodCall({}, {} args)", method, arguments.len())
            }
            ASTNode::FieldAccess { field, .. } => {
                format!("FieldAccess({})", field)
            }
            ASTNode::New {
                class,
                arguments,
                field_initializers,
                type_arguments,
                ..
            } => {
                if type_arguments.is_empty() {
                    format!(
                        "New({}, {} args, {} init)",
                        class,
                        arguments.len(),
                        field_initializers.len()
                    )
                } else {
                    format!(
                        "New({}<{}>, {} args, {} init)",
                        class,
                        type_arguments.join(", "),
                        arguments.len(),
                        field_initializers.len()
                    )
                }
            }
            ASTNode::This { .. } => "This".to_string(),
            ASTNode::Me { .. } => "Me".to_string(),
            ASTNode::FromCall {
                parent,
                method,
                arguments,
                ..
            } => {
                format!("FromCall({}.{}, {} args)", parent, method, arguments.len())
            }
            ASTNode::ThisField { field, .. } => {
                format!("ThisField({})", field)
            }
            ASTNode::MeField { field, .. } => {
                format!("MeField({})", field)
            }
            ASTNode::Local { variables, .. } => {
                format!("Local({})", variables.join(", "))
            }
            ASTNode::Outbox { variables, .. } => {
                format!("Outbox({})", variables.join(", "))
            }
            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                format!("FunctionCall({}, {} args)", name, arguments.len())
            }
            ASTNode::Call { .. } => "Call".to_string(),
            ASTNode::Nowait { variable, .. } => {
                format!("Nowait({})", variable)
            }
            ASTNode::TaskScope {
                source_keyword,
                body,
                ..
            } => {
                format!("TaskScope({}, {} statements)", source_keyword, body.len())
            }
            ASTNode::ContextScope {
                source_keyword,
                name,
                body,
                ..
            } => {
                format!(
                    "ContextScope({} {}, {} statements)",
                    source_keyword,
                    name,
                    body.len()
                )
            }
            ASTNode::FastMemRegion { contract, body, .. } => {
                format!("FastMemRegion({}, {} statements)", contract, body.len())
            }
            ASTNode::Arrow { .. } => "Arrow(>>)".to_string(),
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => {
                let mut desc = format!(
                    "TryCatch({} try statements, {} catch clauses",
                    try_body.len(),
                    catch_clauses.len()
                );
                if finally_body.is_some() {
                    desc.push_str(", has finally");
                }
                desc.push(')');
                desc
            }
            ASTNode::Throw { .. } => "Throw".to_string(),
            ASTNode::AwaitExpression { expression, .. } => {
                format!("Await({:?})", expression)
            }
            ASTNode::MatchExpr { .. } => "MatchExpr".to_string(),
            ASTNode::EnumMatchExpr { .. } => "EnumMatchExpr".to_string(),
            ASTNode::QMarkPropagate { .. } => "QMarkPropagate".to_string(),
            ASTNode::Lambda { params, body, .. } => {
                format!("Lambda({} params, {} statements)", params.len(), body.len())
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                format!("ArrayLiteral({} elements)", elements.len())
            }
            ASTNode::MapLiteral { entries, .. } => {
                format!("MapLiteral({} entries)", entries.len())
            }
            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => {
                format!(
                    "RecordLiteral({}, {} fields)",
                    record_type_name,
                    fields.len()
                )
            }
            ASTNode::RecordUpdate { updates, .. } => {
                format!("RecordUpdate({} fields)", updates.len())
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                format!(
                    "BlockExpr({} prelude stmts, tail={})",
                    prelude_stmts.len(),
                    tail_expr.node_type()
                )
            }
            ASTNode::Index { target, index, .. } => {
                format!("Index(target={:?}, index={:?})", target, index)
            }
            ASTNode::ScopeBox { .. } => "ScopeBox".to_string(),
            ASTNode::GroupedAssignmentExpr { lhs, .. } => {
                format!("GroupedAssignmentExpr(lhs={})", lhs)
            }
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}
