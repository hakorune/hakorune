use std::collections::BTreeMap;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};

use super::function_proof::{prove_function, FunctionProofOutcomeV1};
use super::{CallableResultCatalogErrorV1, VerifiedCallableResultDispositionV1};

#[derive(Debug)]
pub(crate) struct VerifiedSameModuleCallableResultCatalogV1 {
    rows_by_key: BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
}

impl VerifiedSameModuleCallableResultCatalogV1 {
    pub(crate) fn verify(
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let static_declarations = declarations.static_declarations().collect::<Vec<_>>();
        let static_count = static_declarations.len();
        let rows_by_key = static_declarations
            .into_iter()
            .map(|(key, declaration)| {
                let disposition = match prove_function(declaration)? {
                    FunctionProofOutcomeV1::Exact(requirements) => {
                        VerifiedCallableResultDispositionV1::exact_i64(key, requirements)?
                    }
                    FunctionProofOutcomeV1::Unavailable(reason) => {
                        VerifiedCallableResultDispositionV1::Unavailable(reason)
                    }
                };
                Ok((key.clone(), disposition))
            })
            .collect::<Result<BTreeMap<_, _>, CallableResultCatalogErrorV1>>()?;
        if rows_by_key.len() != static_count {
            return Err(CallableResultCatalogErrorV1::ResultRowCardinalityMismatch {
                static_declarations: static_count,
                rows: rows_by_key.len(),
            });
        }
        Ok(Self { rows_by_key })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows_by_key.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows_by_key.is_empty()
    }

    pub(crate) fn disposition(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&VerifiedCallableResultDispositionV1> {
        self.rows_by_key.get(key)
    }

    pub(crate) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &VerifiedCallableResultDispositionV1,
        ),
    > {
        self.rows_by_key.iter()
    }
}
