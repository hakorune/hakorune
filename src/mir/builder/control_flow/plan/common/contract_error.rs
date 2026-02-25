use crate::mir::join_ir::lowering::error_tags;

#[derive(Debug, Clone)]
pub(crate) struct ContractViolation {
    pub(crate) msg: String,
    pub(crate) hint: String,
}

impl ContractViolation {
    pub(crate) fn new(msg: &str, hint: &str) -> Self {
        Self {
            msg: msg.to_string(),
            hint: hint.to_string(),
        }
    }
}

pub(crate) fn freeze_contract(tag: &str, violation: &ContractViolation) -> String {
    error_tags::freeze_with_hint(tag, &violation.msg, &violation.hint)
}

pub(crate) enum ExtractDecision<T> {
    Match(T),
    NotApplicable,
    Contract(ContractViolation),
}

impl<T> ExtractDecision<T> {
    pub(crate) fn contract(msg: &str, hint: &str) -> Self {
        Self::Contract(ContractViolation::new(msg, hint))
    }
}

pub(crate) fn finalize_extract<T>(
    decision: ExtractDecision<T>,
    tag: &str,
) -> Result<Option<T>, String> {
    match decision {
        ExtractDecision::Match(value) => Ok(Some(value)),
        ExtractDecision::NotApplicable => Ok(None),
        ExtractDecision::Contract(violation) => Err(freeze_contract(tag, &violation)),
    }
}
