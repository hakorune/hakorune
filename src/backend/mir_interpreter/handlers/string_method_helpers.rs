use super::*;

#[derive(Debug, Clone, Copy)]
pub(super) struct ArgParsePolicy {
    pub allow_empty: bool,
    pub allow_extra: bool,
}

impl ArgParsePolicy {
    pub const STRICT: Self = Self {
        allow_empty: false,
        allow_extra: false,
    };
    pub const LENIENT: Self = Self {
        allow_empty: true,
        allow_extra: true,
    };
}

pub(super) fn parse_index_of_args(
    this: &mut MirInterpreter,
    args: &[ValueId],
    policy: ArgParsePolicy,
    err_label: &str,
) -> Result<(String, Option<i64>), VMError> {
    if args.is_empty() {
        return Err(this.err_invalid(err_label));
    }
    let needle = this.reg_load(args[0])?.to_string();
    let start = if args.len() >= 2 {
        Some(this.reg_load(args[1])?.as_integer().unwrap_or(0))
    } else {
        None
    };
    if !policy.allow_extra && args.len() > 2 {
        return Err(this.err_invalid(err_label));
    }
    Ok((needle, start))
}

pub(super) fn parse_last_index_of_args(
    this: &mut MirInterpreter,
    args: &[ValueId],
    policy: ArgParsePolicy,
    err_label: &str,
) -> Result<String, VMError> {
    if args.is_empty() {
        return Err(this.err_invalid(err_label));
    }
    if !policy.allow_extra && args.len() > 1 {
        return Err(this.err_invalid(err_label));
    }
    Ok(this.reg_load(args[0])?.to_string())
}

pub(super) fn parse_substring_args(
    this: &mut MirInterpreter,
    args: &[ValueId],
    policy: ArgParsePolicy,
    err_label: &str,
) -> Result<(i64, Option<i64>), VMError> {
    match args.len() {
        0 => {
            if policy.allow_empty {
                Ok((0, None))
            } else {
                Err(this.err_invalid(err_label))
            }
        }
        1 => Ok((this.reg_load(args[0])?.as_integer().unwrap_or(0), None)),
        2 => Ok((
            this.reg_load(args[0])?.as_integer().unwrap_or(0),
            Some(this.reg_load(args[1])?.as_integer().unwrap_or(0)),
        )),
        _ => {
            if policy.allow_extra {
                Ok((
                    this.reg_load(args[0])?.as_integer().unwrap_or(0),
                    Some(this.reg_load(args[1])?.as_integer().unwrap_or(0)),
                ))
            } else {
                Err(this.err_invalid(err_label))
            }
        }
    }
}

pub(super) fn eval_string_char_predicate(method: &str, text: &str) -> Option<bool> {
    match method {
        "is_space" => Some(matches!(text, " " | "\t" | "\n" | "\r")),
        "is_alpha" => Some(
            text.chars()
                .next()
                .map(|c| ('A'..='Z').contains(&c) || ('a'..='z').contains(&c) || c == '_')
                .unwrap_or(false),
        ),
        _ => None,
    }
}

pub(super) fn try_eval_string_char_predicate(
    this: &mut MirInterpreter,
    method: &str,
    args: &[ValueId],
) -> Result<Option<VMValue>, VMError> {
    let Some(expected) = eval_string_char_predicate(method, "") else {
        return Ok(None);
    };
    if args.len() != 1 {
        return Err(this.err_invalid(format!("{} requires 1 argument", method)));
    }
    let ch = this.reg_load(args[0])?.to_string();
    let value = eval_string_char_predicate(method, &ch).unwrap_or(expected);
    Ok(Some(VMValue::Bool(value)))
}

#[cfg(test)]
mod tests {
    use super::eval_string_char_predicate;

    #[test]
    fn string_char_predicate_space_contract() {
        assert_eq!(eval_string_char_predicate("is_space", " "), Some(true));
        assert_eq!(eval_string_char_predicate("is_space", "\t"), Some(true));
        assert_eq!(eval_string_char_predicate("is_space", "x"), Some(false));
        assert_eq!(eval_string_char_predicate("is_space", ""), Some(false));
    }

    #[test]
    fn string_char_predicate_alpha_contract() {
        assert_eq!(eval_string_char_predicate("is_alpha", "A"), Some(true));
        assert_eq!(eval_string_char_predicate("is_alpha", "_"), Some(true));
        assert_eq!(eval_string_char_predicate("is_alpha", "9"), Some(false));
        assert_eq!(eval_string_char_predicate("is_alpha", ""), Some(false));
    }

    #[test]
    fn string_char_predicate_unknown_method_returns_none() {
        assert_eq!(eval_string_char_predicate("missing", "A"), None);
    }
}
