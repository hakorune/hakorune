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
