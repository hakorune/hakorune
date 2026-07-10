use std::collections::BTreeSet;

pub use hakorune_frontend_grammar::contract::GrammarProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildMode {
    Test,
    Debug,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserBuildConfig {
    pub grammar_profile: GrammarProfile,
    pub mode: BuildMode,
    pub known_features: BTreeSet<String>,
    pub enabled_features: BTreeSet<String>,
    pub target_os: String,
    pub target_arch: String,
    pub backend_kind: String,
}

impl Default for ParserBuildConfig {
    fn default() -> Self {
        Self {
            grammar_profile: GrammarProfile::Canonical,
            mode: BuildMode::Release,
            known_features: BTreeSet::new(),
            enabled_features: BTreeSet::new(),
            target_os: std::env::consts::OS.to_string(),
            target_arch: std::env::consts::ARCH.to_string(),
            backend_kind: "unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grammar_profile_defaults_to_canonical() {
        assert_eq!(
            ParserBuildConfig::default().grammar_profile,
            GrammarProfile::Canonical
        );
    }
}
