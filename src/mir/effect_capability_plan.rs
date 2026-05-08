use crate::ast::RuneAttr;
use crate::mir::MirFunction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectRequirement {
    NoAlloc,
    NoSafepoint,
}

impl EffectRequirement {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoAlloc => "no_alloc",
            Self::NoSafepoint => "no_safepoint",
        }
    }

    fn from_contract_arg(arg: &str) -> Option<Self> {
        match arg {
            "no_alloc" => Some(Self::NoAlloc),
            "no_safepoint" => Some(Self::NoSafepoint),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectPlan {
    pub function: String,
    pub requires: Vec<EffectRequirement>,
    pub verified: bool,
    pub source: String,
}

impl EffectPlan {
    fn rune_contract(function: &str, requires: Vec<EffectRequirement>) -> Self {
        Self {
            function: function.to_string(),
            requires,
            verified: false,
            source: "rune_contract".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityPlan {
    pub function: String,
    pub allow: Vec<String>,
    pub verified: bool,
    pub source: String,
}

pub fn effect_plans_from_runes(function: &str, runes: &[RuneAttr]) -> Vec<EffectPlan> {
    let mut requires = Vec::new();
    for rune in runes {
        if rune.name != "Contract" {
            continue;
        }
        let Some(requirement) = rune
            .args
            .first()
            .and_then(|arg| EffectRequirement::from_contract_arg(arg))
        else {
            continue;
        };
        if !requires.contains(&requirement) {
            requires.push(requirement);
        }
    }

    if requires.is_empty() {
        Vec::new()
    } else {
        requires.sort();
        vec![EffectPlan::rune_contract(function, requires)]
    }
}

pub fn capability_plans_from_runes(_function: &str, _runes: &[RuneAttr]) -> Vec<CapabilityPlan> {
    Vec::new()
}

pub fn refresh_function_effect_capability_plans(function: &mut MirFunction) {
    function.metadata.effect_plans =
        effect_plans_from_runes(&function.signature.name, &function.metadata.runes);
    function.metadata.capability_plans =
        capability_plans_from_runes(&function.signature.name, &function.metadata.runes);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rune(name: &str, arg: &str) -> RuneAttr {
        RuneAttr {
            name: name.to_string(),
            args: vec![arg.to_string()],
        }
    }

    #[test]
    fn effect_capability_plan_preserves_live_contract_obligations() {
        let plans = effect_plans_from_runes(
            "Main.fast/0",
            &[
                rune("Contract", "no_alloc"),
                rune("Contract", "no_safepoint"),
                rune("Contract", "readonly"),
            ],
        );

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].function, "Main.fast/0");
        assert_eq!(
            plans[0]
                .requires
                .iter()
                .map(|requirement| requirement.as_str())
                .collect::<Vec<_>>(),
            vec!["no_alloc", "no_safepoint"]
        );
        assert!(!plans[0].verified);
        assert_eq!(plans[0].source, "rune_contract");
    }

    #[test]
    fn capability_plan_boundary_has_no_parser_surface_yet() {
        assert!(
            capability_plans_from_runes("Main.fast/0", &[rune("Contract", "no_alloc")]).is_empty()
        );
    }
}
