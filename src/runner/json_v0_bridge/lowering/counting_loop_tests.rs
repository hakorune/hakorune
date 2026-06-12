use super::*;

fn var(name: &str) -> ExprV0 {
    ExprV0::Var {
        name: name.to_string(),
    }
}

fn int(value: i64) -> ExprV0 {
    ExprV0::Int {
        value: serde_json::json!(value),
    }
}

fn increment_i() -> StmtV0 {
    StmtV0::Local {
        name: "i".to_string(),
        expr: ExprV0::Binary {
            op: "+".to_string(),
            lhs: Box::new(var("i")),
            rhs: Box::new(int(1)),
        },
    }
}

#[test]
fn detects_simple_counting_loop_candidate() {
    let cond = ExprV0::Compare {
        op: "<".to_string(),
        lhs: Box::new(var("i")),
        rhs: Box::new(var("capacity")),
    };
    let body = vec![StmtV0::Expr { expr: var("work") }, increment_i()];

    let candidate = detect_counting_loop_candidate(&cond, &body).expect("candidate");
    assert_eq!(candidate.index_name, "i");
    assert_eq!(candidate.step, 1);
}

#[test]
fn rejects_increment_before_access_shape() {
    let cond = ExprV0::Compare {
        op: "<".to_string(),
        lhs: Box::new(var("i")),
        rhs: Box::new(var("capacity")),
    };
    let body = vec![increment_i(), StmtV0::Expr { expr: var("work") }];

    assert!(detect_counting_loop_candidate(&cond, &body).is_none());
}
