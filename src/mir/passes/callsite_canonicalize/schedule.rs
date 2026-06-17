use crate::mir::MirModule;

/// Production pipeline sites that schedule callsite canonicalization.
///
/// The transform itself remains owned by `canonicalize_callsites`; this enum
/// names the timing seam so callers do not each own an implicit schedule rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallsiteCanonicalizeScheduleSite {
    MirCompilerPostRc,
    MirOptimizerLateCallAndInline,
    ProgramJsonV0Bridge,
    MirJsonV0Loader,
}

pub fn canonicalize_for_site(
    module: &mut MirModule,
    _site: CallsiteCanonicalizeScheduleSite,
) -> usize {
    super::pass::canonicalize_callsites(module)
}
