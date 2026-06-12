use crate::ast::ASTNode;
use crate::parser::{NyashParser, ParseError};
use std::collections::HashMap;

impl NyashParser {
    pub(crate) fn prune_build_gate_program(&self, ast: ASTNode) -> Result<ASTNode, ParseError> {
        match ast {
            ASTNode::Program { statements, span } => Ok(ASTNode::Program {
                statements: self.prune_build_gate_items(statements)?,
                span,
            }),
            other => Ok(other),
        }
    }

    fn prune_build_gate_items(&self, items: Vec<ASTNode>) -> Result<Vec<ASTNode>, ParseError> {
        let mut out = Vec::new();
        for item in items {
            match item {
                ASTNode::BuildGate {
                    predicate,
                    then_items,
                    else_items,
                    span,
                } => {
                    let selected = if self.eval_build_predicate(&predicate, span)? {
                        then_items
                    } else {
                        else_items.unwrap_or_default()
                    };
                    out.extend(self.prune_build_gate_items(selected)?);
                }
                ASTNode::Program { statements, span } => {
                    out.push(ASTNode::Program {
                        statements: self.prune_build_gate_items(statements)?,
                        span,
                    });
                }
                ASTNode::ScopeBox { body, span } => {
                    out.push(ASTNode::ScopeBox {
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::TaskScope {
                    body,
                    source_keyword,
                    span,
                } => {
                    out.push(ASTNode::TaskScope {
                        body: self.prune_build_gate_items(body)?,
                        source_keyword,
                        span,
                    });
                }
                ASTNode::ContextScope {
                    name,
                    declared_type_name,
                    value,
                    body,
                    source_keyword,
                    span,
                } => {
                    out.push(ASTNode::ContextScope {
                        name,
                        declared_type_name,
                        value,
                        body: self.prune_build_gate_items(body)?,
                        source_keyword,
                        span,
                    });
                }
                ASTNode::FastMemRegion {
                    contract,
                    body,
                    span,
                } => {
                    out.push(ASTNode::FastMemRegion {
                        contract,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    span,
                } => {
                    out.push(ASTNode::If {
                        condition,
                        then_body: self.prune_build_gate_items(then_body)?,
                        else_body: else_body
                            .map(|body| self.prune_build_gate_items(body))
                            .transpose()?,
                        span,
                    });
                }
                ASTNode::Loop {
                    condition,
                    body,
                    span,
                } => {
                    out.push(ASTNode::Loop {
                        condition,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::LoopRange {
                    var_name,
                    start,
                    end,
                    body,
                    span,
                } => {
                    out.push(ASTNode::LoopRange {
                        var_name,
                        start,
                        end,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::Return { value, span } => {
                    out.push(ASTNode::Return { value, span });
                }
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
                    span,
                } => {
                    let methods = methods
                        .into_iter()
                        .map(|(key, method)| Ok((key, self.prune_build_gate_node(method)?)))
                        .collect::<Result<HashMap<_, _>, ParseError>>()?;
                    let constructors = constructors
                        .into_iter()
                        .map(|(key, ctor)| Ok((key, self.prune_build_gate_node(ctor)?)))
                        .collect::<Result<HashMap<_, _>, ParseError>>()?;
                    let static_init = static_init
                        .map(|body| self.prune_build_gate_items(body))
                        .transpose()?;
                    out.push(ASTNode::BoxDeclaration {
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
                        span,
                    });
                }
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
                    span,
                } => {
                    out.push(ASTNode::FunctionDeclaration {
                        name,
                        params,
                        param_decls,
                        return_type_name,
                        body: self.prune_build_gate_items(body)?,
                        uses,
                        contracts,
                        is_static,
                        is_override,
                        attrs,
                        span,
                    });
                }
                ASTNode::Lambda { params, body, span } => {
                    out.push(ASTNode::Lambda {
                        params,
                        body: self.prune_build_gate_items(body)?,
                        span,
                    });
                }
                ASTNode::TryCatch {
                    try_body,
                    catch_clauses,
                    finally_body,
                    span,
                } => {
                    let catch_clauses = catch_clauses
                        .into_iter()
                        .map(|clause| {
                            Ok(crate::ast::CatchClause {
                                exception_type: clause.exception_type,
                                variable_name: clause.variable_name,
                                body: self.prune_build_gate_items(clause.body)?,
                                span: clause.span,
                            })
                        })
                        .collect::<Result<Vec<_>, ParseError>>()?;
                    let finally_body = finally_body
                        .map(|body| self.prune_build_gate_items(body))
                        .transpose()?;
                    out.push(ASTNode::TryCatch {
                        try_body: self.prune_build_gate_items(try_body)?,
                        catch_clauses,
                        finally_body,
                        span,
                    });
                }
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    span,
                } => {
                    out.push(ASTNode::BlockExpr {
                        prelude_stmts: self.prune_build_gate_items(prelude_stmts)?,
                        tail_expr,
                        span,
                    });
                }
                ASTNode::GlobalVar { name, value, span } => {
                    out.push(ASTNode::GlobalVar { name, value, span });
                }
                other => out.push(other),
            }
        }
        Ok(out)
    }

    fn prune_build_gate_node(&self, node: ASTNode) -> Result<ASTNode, ParseError> {
        match node {
            ASTNode::Program { statements, span } => Ok(ASTNode::Program {
                statements: self.prune_build_gate_items(statements)?,
                span,
            }),
            ASTNode::ScopeBox { body, span } => Ok(ASTNode::ScopeBox {
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::TaskScope {
                body,
                source_keyword,
                span,
            } => Ok(ASTNode::TaskScope {
                body: self.prune_build_gate_items(body)?,
                source_keyword,
                span,
            }),
            ASTNode::ContextScope {
                name,
                declared_type_name,
                value,
                body,
                source_keyword,
                span,
            } => Ok(ASTNode::ContextScope {
                name,
                declared_type_name,
                value,
                body: self.prune_build_gate_items(body)?,
                source_keyword,
                span,
            }),
            ASTNode::FastMemRegion {
                contract,
                body,
                span,
            } => Ok(ASTNode::FastMemRegion {
                contract,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => Ok(ASTNode::If {
                condition,
                then_body: self.prune_build_gate_items(then_body)?,
                else_body: else_body
                    .map(|body| self.prune_build_gate_items(body))
                    .transpose()?,
                span,
            }),
            ASTNode::Loop {
                condition,
                body,
                span,
            } => Ok(ASTNode::Loop {
                condition,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                span,
            } => Ok(ASTNode::LoopRange {
                var_name,
                start,
                end,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::Return { value, span } => Ok(ASTNode::Return { value, span }),
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
                span,
            } => {
                let methods = methods
                    .into_iter()
                    .map(|(key, method)| Ok((key, self.prune_build_gate_node(method)?)))
                    .collect::<Result<HashMap<_, _>, ParseError>>()?;
                let constructors = constructors
                    .into_iter()
                    .map(|(key, ctor)| Ok((key, self.prune_build_gate_node(ctor)?)))
                    .collect::<Result<HashMap<_, _>, ParseError>>()?;
                let static_init = static_init
                    .map(|body| self.prune_build_gate_items(body))
                    .transpose()?;
                Ok(ASTNode::BoxDeclaration {
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
                    span,
                })
            }
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
                span,
            } => Ok(ASTNode::FunctionDeclaration {
                name,
                params,
                param_decls,
                return_type_name,
                body: self.prune_build_gate_items(body)?,
                uses,
                contracts,
                is_static,
                is_override,
                attrs,
                span,
            }),
            ASTNode::Lambda { params, body, span } => Ok(ASTNode::Lambda {
                params,
                body: self.prune_build_gate_items(body)?,
                span,
            }),
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                span,
            } => {
                let catch_clauses = catch_clauses
                    .into_iter()
                    .map(|clause| {
                        Ok(crate::ast::CatchClause {
                            exception_type: clause.exception_type,
                            variable_name: clause.variable_name,
                            body: self.prune_build_gate_items(clause.body)?,
                            span: clause.span,
                        })
                    })
                    .collect::<Result<Vec<_>, ParseError>>()?;
                let finally_body = finally_body
                    .map(|body| self.prune_build_gate_items(body))
                    .transpose()?;
                Ok(ASTNode::TryCatch {
                    try_body: self.prune_build_gate_items(try_body)?,
                    catch_clauses,
                    finally_body,
                    span,
                })
            }
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                span,
            } => Ok(ASTNode::BlockExpr {
                prelude_stmts: self.prune_build_gate_items(prelude_stmts)?,
                tail_expr,
                span,
            }),
            ASTNode::GlobalVar { name, value, span } => {
                Ok(ASTNode::GlobalVar { name, value, span })
            }
            other => Ok(other),
        }
    }
}
