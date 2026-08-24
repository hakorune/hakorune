use crate::mir::MirModule;

/// Production pipeline sites that schedule compatibility/canonicalization.
///
/// The transform itself remains owned by `canonicalize_callsites`; this enum
/// names the timing seam so callers do not each own an implicit schedule rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallsiteCanonicalizeScheduleSite {
    MirCompilerPostRc,
    ProgramJsonV0Bridge,
    MirJsonV0Loader,
}

pub fn canonicalize_for_site(
    module: &mut MirModule,
    site: CallsiteCanonicalizeScheduleSite,
) -> usize {
    let allow_legacy_target_rewrite =
        !matches!(site, CallsiteCanonicalizeScheduleSite::ProgramJsonV0Bridge);
    super::pass::canonicalize_callsites_for_site(module, allow_legacy_target_rewrite)
}
