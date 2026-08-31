//! Cataloged instance-method source scope wiring.
//!
//! Kept separate from the parent adapter so the parent remains below the
//! source-size boundary.  This is a behavior-neutral BoxShape split: the
//! package, source scope, and exactly-once validation remain unchanged.

use super::*;

impl<'package, 'loan, 'port, 'collector, 'target>
    NormalCallableSemanticPackagePortAdapterV1<'package, 'loan, 'port, 'collector, 'target>
{
    pub(super) fn with_cataloged_callable_source_scope<R>(
        &mut self,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        execute: impl FnOnce(
            &mut RawInvocationChildPortV1<'port, 'collector>,
            super::super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
            NormalCatalogedBoxMethodDraftAdmissionV1,
            ResolvedCallablePhysicalSignatureLoanV1<'_>,
        ) -> Result<R, String>,
    ) -> Result<R, String> {
        let inner = &mut *self.inner;
        let ordinary_new_claim_ledger = self.package.ordinary_new_claim_ledger();
        self.package
            .with_selected_cataloged_lowering_input_and_signature(admission, |input, signature| {
                super::validate_selected_cataloged_input(&input)?;
                super::validate_selected_signature_loan(&input, &signature)?;
                if matches!(
                    input.selected().semantic(),
                    crate::mir::normal_callable_semantic_package::SelectedCallableSemanticRefV1::Dynamic { .. }
                ) {
                    return Err(
                        "[freeze:contract][mir/callable-semantic-package/dynamic-instance-route]"
                            .to_owned(),
                    );
                }
                let (selected, admission, _physical_header) = input.into_lowering_and_admission();
                let lineage = super::super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                    admission.source_key().clone(),
                );
                super::with_selected_source_scope(
                    inner,
                    lineage,
                    selected,
                    Rc::clone(&ordinary_new_claim_ledger),
                    |inner, transport| execute(inner, transport, admission, signature),
                )
            })
            .map_err(super::package_issue)?
    }
}
