use crate::ast::RuneAttr;
use crate::mir::MirFunction;

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

pub fn refresh_function_inline_plans(function: &mut MirFunction) {
    function.metadata.inline_plans =
        inline_plans_from_runes(&function.signature.name, &function.metadata.runes);
}
