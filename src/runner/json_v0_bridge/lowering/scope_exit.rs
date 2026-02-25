use super::StmtV0;

enum ScopeExitItemV0 {
    Statement(StmtV0),
    FiniRegistration(Vec<StmtV0>),
}

pub(super) fn normalize_scope_exit_registrations(stmts: &[StmtV0]) -> Result<Vec<StmtV0>, String> {
    let mut items: Vec<ScopeExitItemV0> = Vec::new();
    for stmt in stmts.iter().cloned() {
        match stmt {
            StmtV0::FiniReg { prelude, fini } => {
                validate_fini_body(&fini)?;

                for prelude_stmt in prelude {
                    items.push(ScopeExitItemV0::Statement(prelude_stmt));
                }
                items.push(ScopeExitItemV0::FiniRegistration(fini));
            }
            other => items.push(ScopeExitItemV0::Statement(other)),
        }
    }

    let mut lowered_tail: Vec<StmtV0> = Vec::new();
    for item in items.into_iter().rev() {
        match item {
            ScopeExitItemV0::Statement(stmt) => {
                let mut next = Vec::with_capacity(lowered_tail.len() + 1);
                next.push(stmt);
                next.extend(lowered_tail);
                lowered_tail = next;
            }
            ScopeExitItemV0::FiniRegistration(fini_body) => {
                lowered_tail = vec![StmtV0::Try {
                    try_body: lowered_tail,
                    catches: Vec::new(),
                    finally: fini_body,
                }];
            }
        }
    }

    Ok(lowered_tail)
}

fn validate_fini_body(stmts: &[StmtV0]) -> Result<(), String> {
    for stmt in stmts {
        validate_fini_body_stmt(stmt)?;
    }
    Ok(())
}

fn validate_fini_body_stmt(stmt: &StmtV0) -> Result<(), String> {
    match stmt {
        StmtV0::Return { .. } => Err(
            "[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit] return in fini body"
                .to_string(),
        ),
        StmtV0::Throw { .. } => Err(
            "[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit] throw in fini body"
                .to_string(),
        ),
        StmtV0::Break => Err(
            "[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit] break in fini body"
                .to_string(),
        ),
        StmtV0::Continue => Err(
            "[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit] continue in fini body"
                .to_string(),
        ),
        StmtV0::FiniReg { .. } => Err(
            "[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit] nested FiniReg in fini body"
                .to_string(),
        ),
        StmtV0::If { then, r#else, .. } => {
            for s in then {
                validate_fini_body_stmt(s)?;
            }
            if let Some(elses) = r#else.as_ref() {
                for s in elses {
                    validate_fini_body_stmt(s)?;
                }
            }
            Ok(())
        }
        StmtV0::Loop { body, .. } => {
            for s in body {
                validate_fini_body_stmt(s)?;
            }
            Ok(())
        }
        StmtV0::Try {
            try_body,
            catches,
            finally,
        } => {
            for s in try_body {
                validate_fini_body_stmt(s)?;
            }
            for catch in catches {
                for s in &catch.body {
                    validate_fini_body_stmt(s)?;
                }
            }
            for s in finally {
                validate_fini_body_stmt(s)?;
            }
            Ok(())
        }
        StmtV0::Expr { .. } | StmtV0::Local { .. } | StmtV0::Extern { .. } => Ok(()),
    }
}
