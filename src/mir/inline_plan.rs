use crate::ast::RuneAttr;
use crate::mir::inline_leaf::{check_leaf_inline_shape, InlineLeafViolation};
use crate::mir::{BasicBlockId, MirFunction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineRequest {
    None,
    Prefer,
    Avoid,
    Required,
}

impl InlineRequest {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Prefer => "prefer",
            Self::Avoid => "avoid",
            Self::Required => "required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineHotness {
    Hot,
    Cold,
}

impl InlineHotness {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hot => "hot",
            Self::Cold => "cold",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlinePlan {
    pub function: String,
    pub request: InlineRequest,
    pub hotness: Option<InlineHotness>,
    pub max_ir: Option<u32>,
    pub requires: Vec<String>,
    pub verified: bool,
    pub fallback: String,
    pub source: String,
}

impl InlinePlan {
    pub fn from_hint(function: &str, hint: &str) -> Option<Self> {
        let (request, hotness) = match hint {
            "inline" => (InlineRequest::Prefer, None),
            "noinline" => (InlineRequest::Avoid, None),
            "hot" => (InlineRequest::None, Some(InlineHotness::Hot)),
            "cold" => (InlineRequest::None, Some(InlineHotness::Cold)),
            _ => return None,
        };

        Some(Self {
            function: function.to_string(),
            request,
            hotness,
            max_ir: None,
            requires: Vec::new(),
            verified: false,
            fallback: "keep_call".to_string(),
            source: "rune_hint".to_string(),
        })
    }

    pub fn from_lowering(function: &str, lowering: &str) -> Option<Self> {
        if lowering != "inline_required" {
            return None;
        }

        Some(Self {
            function: function.to_string(),
            request: InlineRequest::Required,
            hotness: None,
            max_ir: None,
            requires: vec!["no_alloc".to_string(), "no_safepoint".to_string()],
            verified: false,
            fallback: "fail_fast".to_string(),
            source: "rune_lowering".to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredInlineViolation {
    pub tag: &'static str,
    pub block: Option<BasicBlockId>,
    pub instruction_index: Option<usize>,
    pub reason: String,
}

impl RequiredInlineViolation {
    fn missing_contract(function: &MirFunction, contract: &str) -> Self {
        Self {
            tag: "missing-contract",
            block: None,
            instruction_index: None,
            reason: format!(
                "[inline-plan/missing-contract] fn={} contract={} reason=required inline needs verified Contract({})",
                function.signature.name, contract, contract
            ),
        }
    }

    fn from_leaf_violation(violation: InlineLeafViolation) -> Self {
        Self {
            tag: violation.tag,
            block: violation.block,
            instruction_index: violation.instruction_index,
            reason: violation.reason,
        }
    }
}

pub fn inline_plans_from_runes(function: &str, runes: &[RuneAttr]) -> Vec<InlinePlan> {
    runes
        .iter()
        .filter_map(|rune| {
            let value = rune.args.first()?;
            match rune.name.as_str() {
                "Hint" => InlinePlan::from_hint(function, value),
                "Lowering" => InlinePlan::from_lowering(function, value),
                _ => None,
            }
        })
        .collect()
}

pub fn required_inline_plan_violations(
    function: &MirFunction,
    plan: &InlinePlan,
) -> Vec<RequiredInlineViolation> {
    if plan.request != InlineRequest::Required {
        return Vec::new();
    }

    let mut violations = Vec::new();
    for required in &plan.requires {
        if !has_contract(&function.metadata.runes, required) {
            violations.push(RequiredInlineViolation::missing_contract(
                function, required,
            ));
        }
    }
    violations.extend(
        check_leaf_inline_shape(function, plan.max_ir)
            .into_iter()
            .map(RequiredInlineViolation::from_leaf_violation),
    );
    violations
}

pub fn required_inline_plan_verified(function: &MirFunction, plan: &InlinePlan) -> bool {
    required_inline_plan_violations(function, plan).is_empty()
}

pub fn refresh_function_inline_plans(function: &mut MirFunction) {
    let mut plans = inline_plans_from_runes(&function.signature.name, &function.metadata.runes);
    for plan in &mut plans {
        if plan.request == InlineRequest::Required {
            plan.verified = required_inline_plan_verified(function, plan);
        }
    }
    function.metadata.inline_plans = plans;
}

fn has_contract(runes: &[RuneAttr], contract: &str) -> bool {
    runes.iter().any(|rune| {
        rune.name == "Contract" && rune.args.first().map(String::as_str) == Some(contract)
    })
}
