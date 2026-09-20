use crate::mir::builder::{CompilationContext, VerifiedSourceBackedDynamicCallableV1};
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicExitTransactionCoSealV1;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;

#[derive(Clone, Copy)]
pub(in crate::mir) enum SelectedCallableSemanticRefV1<'loan> {
    Ordinary,
    Dynamic {
        program: &'loan VerifiedDynamicExitTransactionCoSealV1,
        source: &'loan std::rc::Rc<VerifiedSourceBackedDynamicCallableV1>,
    },
}

pub(crate) struct PreparedNormalCallableSemanticPackageInstallV1<'context> {
    pub(super) context: &'context mut CompilationContext,
    pub(super) package: VerifiedNormalCallableSemanticPackageV1,
    pub(super) map_lifecycle_undertaking: Option<super::super::MapLifecycleUndertakingV1>,
}
