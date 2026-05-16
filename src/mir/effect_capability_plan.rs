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
    fn rune_contract(function: &str, requires: Vec<EffectRequirement>, source: &str) -> Self {
        Self {
            function: function.to_string(),
            requires,
            verified: false,
            source: source.to_string(),
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
    let mut saw_profile_requirement = false;
    for rune in runes {
        match rune.name.as_str() {
            "Contract" => {
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
            "Profile" => {
                let Some(expansion) = rune
                    .args
                    .first()
                    .and_then(|name| crate::rune_profile_registry::expansion(name))
                else {
                    continue;
                };
                for contract in expansion.contracts {
                    let Some(requirement) = EffectRequirement::from_contract_arg(contract) else {
                        continue;
                    };
                    if !requires.contains(&requirement) {
                        requires.push(requirement);
                    }
                    saw_profile_requirement = true;
                }
            }
            _ => {}
        }
    }

    if requires.is_empty() {
        Vec::new()
    } else {
        requires.sort();
        let source = if saw_profile_requirement {
            "rune_profile"
        } else {
            "rune_contract"
        };
        vec![EffectPlan::rune_contract(function, requires, source)]
    }
}

fn source_uses_capability_allow(capability: &str) -> Option<&'static str> {
    match capability {
        "osvm" => Some("hako.osvm"),
        "atomic" => Some("hako.atomic"),
        "rawbuf" => Some("hako.rawbuf"),
        "random" => Some("hako.random"),
        _ => None,
    }
}

pub fn capability_plans_from_sources(
    function: &str,
    runes: &[RuneAttr],
    declared_uses: &[String],
) -> Vec<CapabilityPlan> {
    let mut allow = Vec::new();
    let mut saw_profile_capability = false;
    let mut saw_source_uses_capability = false;
    for rune in runes {
        if rune.name != "Profile" {
            continue;
        }
        let Some(expansion) = rune
            .args
            .first()
            .and_then(|name| crate::rune_profile_registry::expansion(name))
        else {
            continue;
        };
        for capability in expansion.capabilities {
            let value = capability.to_string();
            if !allow.contains(&value) {
                allow.push(value);
            }
            saw_profile_capability = true;
        }
    }
    for capability in declared_uses {
        let Some(value) = source_uses_capability_allow(capability) else {
            continue;
        };
        let value = value.to_string();
        if !allow.contains(&value) {
            allow.push(value);
        }
        saw_source_uses_capability = true;
    }
    if allow.is_empty() {
        Vec::new()
    } else {
        allow.sort();
        let source = match (saw_profile_capability, saw_source_uses_capability) {
            (true, true) => "rune_profile+source_uses",
            (true, false) => "rune_profile",
            (false, true) => "source_uses",
            (false, false) => unreachable!("allow is non-empty without a source"),
        };
        vec![CapabilityPlan {
            function: function.to_string(),
            allow,
            verified: false,
            source: source.to_string(),
        }]
    }
}

pub fn capability_plans_from_runes(function: &str, runes: &[RuneAttr]) -> Vec<CapabilityPlan> {
    capability_plans_from_sources(function, runes, &[])
}

pub fn refresh_function_effect_capability_plans(function: &mut MirFunction) {
    function.metadata.effect_plans =
        effect_plans_from_runes(&function.signature.name, &function.metadata.runes);
    function.metadata.capability_plans = capability_plans_from_sources(
        &function.signature.name,
        &function.metadata.runes,
        &function.metadata.declared_capability_uses,
    );
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

    #[test]
    fn source_uses_random_becomes_metadata_only_capability_plan() {
        let plans = capability_plans_from_sources(
            "Main.secure/0",
            &[],
            &["random".to_string(), "osvm".to_string()],
        );

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].function, "Main.secure/0");
        assert_eq!(plans[0].allow, vec!["hako.osvm", "hako.random"]);
        assert_eq!(plans[0].source, "source_uses");
        assert!(!plans[0].verified);
    }

    #[test]
    fn source_declared_uses_emit_canonical_capability_plan_ids() {
        let plans = capability_plans_from_sources(
            "Main.low/0",
            &[],
            &[
                "rawbuf".to_string(),
                "unknown".to_string(),
                "atomic".to_string(),
                "osvm".to_string(),
                "random".to_string(),
            ],
        );

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].function, "Main.low/0");
        assert_eq!(
            plans[0].allow,
            vec!["hako.atomic", "hako.osvm", "hako.random", "hako.rawbuf"]
        );
        assert_eq!(plans[0].source, "source_uses");
        assert!(!plans[0].verified);
    }

    #[test]
    fn profile_expands_to_effect_and_capability_plans() {
        let effect_plans =
            effect_plans_from_runes("Main.fast/0", &[rune("Profile", "allocator.fast")]);
        assert_eq!(effect_plans.len(), 1);
        assert_eq!(
            effect_plans[0]
                .requires
                .iter()
                .map(|requirement| requirement.as_str())
                .collect::<Vec<_>>(),
            vec!["no_alloc", "no_safepoint"]
        );
        assert_eq!(effect_plans[0].source, "rune_profile");

        let capability_plans =
            capability_plans_from_runes("Main.fast/0", &[rune("Profile", "allocator.fast")]);
        assert_eq!(capability_plans.len(), 1);
        assert_eq!(
            capability_plans[0].allow,
            vec!["hako.mem", "hako.ptr", "hako.tls"]
        );
        assert_eq!(capability_plans[0].source, "rune_profile");
        assert!(!capability_plans[0].verified);
    }
}
