//! Main-crate bridge for RuneAttr profile validation.
//!
//! `attrs.rs` is passive AST metadata. Profile-name knowledge currently lives
//! in the main crate registry, so keep that dependency behind this bridge until
//! profile vocabulary is either moved or injected into the frontend AST crate.

pub(crate) fn supported_name(name: &str) -> bool {
    crate::rune_profile_registry::supported_name(name)
}

pub(crate) fn supported_names_msg() -> &'static str {
    crate::rune_profile_registry::SUPPORTED_PROFILE_NAMES_MSG
}
