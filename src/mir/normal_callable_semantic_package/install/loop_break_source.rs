use super::InstalledNormalCallableSemanticPackageV1;

/// Borrowed one-shot handle for the package-owned LoopBreak source rows.
/// Interior mutability stays inside the installed package so a selected
/// lowering callback can request its row only when it enters the
/// source-backed path.
pub(in crate::mir) struct LoopBreakSourcePackageTakeHandle<'package> {
    pub(in crate::mir) installed: &'package InstalledNormalCallableSemanticPackageV1,
}

impl LoopBreakSourcePackageTakeHandle<'_> {
    pub(in crate::mir) fn take_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Result<super::super::LoopBreakSourcePackageLoanV1, String> {
        self.installed
            .loop_break_source
            .borrow_mut()
            .take_for_owner(owner)
    }
}
