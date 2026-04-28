use crate::parser::NyashParser;

struct EnvGuard {
    key: &'static str,
    old: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let old = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, old }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(value) = &self.old {
            std::env::set_var(self.key, value);
        } else {
            std::env::remove_var(self.key);
        }
    }
}

#[test]
fn birth_once_cycle_is_detected_inside_arrow_return_expression() {
    let _guard = EnvGuard::set("NYASH_ENABLE_UNIFIED_MEMBERS", "1");

    let err = NyashParser::parse_from_string(
        r#"
box CyclicBirthOnce {
  birth_once a: IntegerBox => me.b
  birth_once b: IntegerBox => me.a
}
"#,
    )
    .unwrap_err();

    let err = err.to_string();
    assert!(
        err.contains("birth_once declarations must not have cyclic dependencies"),
        "unexpected error: {err}"
    );
}
