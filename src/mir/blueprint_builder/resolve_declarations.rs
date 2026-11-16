//! Declaration collection and explicit signature validation owned by Resolve.
use super::*;

pub(super) fn collect(
    ast: ASTNode,
) -> Result<(DeclarationTable, Vec<Vec<ASTNode>>, Span), Refusal> {
    let ASTNode::Program { statements, span } = ast else {
        return Err(reject(
            "ExpectedModule",
            "expected parsed module",
            ast.span(),
        ));
    };
    let mut table = DeclarationTable {
        entries: Vec::new(),
    };
    let mut bodies = Vec::new();
    let mut owners = BTreeSet::new();
    for node in statements {
        match node {
            ASTNode::BoxDeclaration {
                name,
                is_static: true,
                methods,
                fields,
                field_decls,
                constructors,
                static_init,
                delegates,
                invariants,
                transitions,
                extends,
                implements,
                type_parameters,
                is_sync,
                is_interface,
                is_record,
                attrs,
                span,
                ..
            } => {
                if !fields.is_empty()
                    || !field_decls.is_empty()
                    || !constructors.is_empty()
                    || static_init.is_some()
                    || !delegates.is_empty()
                    || !invariants.is_empty()
                    || !transitions.is_empty()
                    || !extends.is_empty()
                    || !implements.is_empty()
                    || !type_parameters.is_empty()
                    || is_sync
                    || is_interface
                    || is_record
                    || !attrs.is_empty()
                {
                    return Err(reject(
                        "UnsupportedDeclaration",
                        "only plain static scalar declarations are selected",
                        span,
                    ));
                }
                if !owners.insert(name.clone()) {
                    return Err(reject("DuplicateDeclaration", name, span));
                }
                for method in methods.iter_selected_declaration_order() {
                    declare(
                        &mut table,
                        &mut bodies,
                        Some(&name),
                        method.declaration().clone(),
                    )?;
                }
            }
            declaration @ ASTNode::FunctionDeclaration { .. } => {
                declare(&mut table, &mut bodies, None, declaration)?
            }
            other => {
                return Err(reject(
                    "UnsupportedDeclaration",
                    "only static methods and free functions are selected",
                    other.span(),
                ))
            }
        }
    }
    Ok((table, bodies, span))
}
fn declare(
    table: &mut DeclarationTable,
    bodies: &mut Vec<Vec<ASTNode>>,
    owner: Option<&str>,
    node: ASTNode,
) -> Result<(), Refusal> {
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        contracts,
        is_override,
        attrs,
        span,
        ..
    } = node
    else {
        return Err(reject(
            "UnsupportedDeclaration",
            "expected function",
            node.span(),
        ));
    };
    if !uses.is_empty() || !contracts.is_empty() || is_override || !attrs.is_empty() {
        return Err(reject(
            "UnsupportedContract",
            "function uses/contracts/attributes are not selected",
            span,
        ));
    }
    if owner.is_none() && name == "print" {
        return Err(reject("ReservedDeclaration", "print is a builtin", span));
    }
    if params.len() != param_decls.len() {
        return Err(reject(
            "MissingTypeAnnotation",
            "each formal needs a declaration",
            span,
        ));
    }
    let mut parameters = Vec::new();
    for (name, param) in params.iter().zip(&param_decls) {
        if name != &param.name || parameters.iter().any(|(old, _)| old == name) {
            return Err(reject("DuplicateBinding", name, span));
        }
        parameters.push((
            name.clone(),
            declared_type(param.declared_type_name.as_deref(), span)?,
        ));
    }
    let result = declared_type(return_type_name.as_deref(), span)?;
    let key = match owner {
        Some(owner) => Key::static_box_method(owner, &name, params.len() as u32),
        None => Key::free_function(&name, params.len() as u32),
    };
    if table.entries.iter().any(|d| d.key == key) {
        return Err(reject("DuplicateDeclaration", name, span));
    }
    table.entries.push(Declaration {
        id: DeclarationId(table.entries.len()),
        key,
        owner: owner.map(str::to_owned),
        parameters,
        result,
        span,
        io: false,
    });
    bodies.push(body);
    Ok(())
}
