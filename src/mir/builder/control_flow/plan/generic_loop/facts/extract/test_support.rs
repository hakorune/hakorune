//! One process-scoped GenericLoop environment owner for tests.

const MODE_KEYS: [&str; 6] = [
    "NYASH_JOINIR_DEV",
    "HAKO_JOINIR_PLANNER_REQUIRED",
    "HAKO_JOINIR_STRICT",
    "NYASH_JOINIR_STRICT",
    "HAKO_JOINIR_DEBUG",
    "NYASH_JOINIR_DEBUG",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum GenericLoopTestModeV1 {
    Default,
    StrictPlannerRequired,
}

pub(super) fn with_joinir_env<T>(
    joinir_dev: Option<&str>,
    planner_required: Option<&str>,
    f: impl FnOnce() -> T,
) -> T {
    crate::test_support::with_env_vars(&updates(joinir_dev, planner_required, None, None), f)
}

pub(super) fn with_strict_joinir_env<T>(f: impl FnOnce() -> T) -> T {
    crate::test_support::with_env_vars(&updates(Some("1"), Some("1"), Some("1"), None), f)
}

pub(in crate::mir::builder) fn with_default_and_strict_modes<T>(
    mut f: impl FnMut(GenericLoopTestModeV1) -> T,
) -> (T, T) {
    let _config = crate::test_support::ScopedTestConfig::apply(&updates(None, None, None, None));
    let default = f(GenericLoopTestModeV1::Default);

    set_mode(Some("1"), Some("1"), Some("1"), None);
    let strict = f(GenericLoopTestModeV1::StrictPlannerRequired);
    (default, strict)
}

fn updates<'a>(
    joinir_dev: Option<&'a str>,
    planner_required: Option<&'a str>,
    strict: Option<&'a str>,
    debug: Option<&'a str>,
) -> [(&'static str, Option<&'a str>); 6] {
    [
        (MODE_KEYS[0], joinir_dev),
        (MODE_KEYS[1], planner_required),
        (MODE_KEYS[2], strict),
        (MODE_KEYS[3], strict),
        (MODE_KEYS[4], debug),
        (MODE_KEYS[5], debug),
    ]
}

fn set_mode(
    joinir_dev: Option<&str>,
    planner_required: Option<&str>,
    strict: Option<&str>,
    debug: Option<&str>,
) {
    for (name, value) in updates(joinir_dev, planner_required, strict, debug) {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_pair_publishes_default_then_strict_values_under_one_scope() {
        let observed = with_default_and_strict_modes(|mode| {
            (mode, MODE_KEYS.map(|key| std::env::var(key).ok()))
        });

        assert_eq!(observed.0 .0, GenericLoopTestModeV1::Default);
        assert!(observed.0 .1.iter().all(Option::is_none));
        assert_eq!(
            observed.1 .1,
            [
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
                Some("1".to_string()),
                None,
                None,
            ]
        );
    }
}
