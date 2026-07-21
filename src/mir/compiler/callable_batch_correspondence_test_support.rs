//! Shared test-only projection for ROUTEINV-P0d callable-batch proofs.

use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, CanonicalCallableSymbolV1};

use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

pub(in crate::mir::compiler) fn borrowed_catalog_header_rows(
    module: &VerifiedResolvedCallableModuleV1,
) -> Vec<(CanonicalCallableKeyV1, String, usize)> {
    let catalog = module.source().catalog();
    let functions = module.functions_by_key();
    assert_eq!(catalog.len(), functions.len());

    functions
        .keys()
        .map(|key| {
            let header = catalog.index().lookup(key).unwrap();
            let physical =
                CanonicalCallableSymbolV1::from_name_arity(key.name(), key.arity() as usize);
            assert_eq!(header.source_key(), key);
            assert_eq!(header.symbol(), &physical);
            assert_eq!(header.signature().arity(), key.arity() as usize);
            assert!(module.function(key).is_some());
            (
                key.clone(),
                header.symbol().as_mir_name().to_string(),
                header.signature().arity(),
            )
        })
        .collect()
}
