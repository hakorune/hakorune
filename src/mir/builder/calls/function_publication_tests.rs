use crate::mir::function::{FunctionSignature, MirFunction, MirModule};
use crate::mir::{BasicBlockId, EffectMask, MirType};

use super::function_session::{publish_function_draft, FunctionDraftPublicationErrorV1};

#[test]
fn canonical_draft_publication_rejects_duplicate_without_overwrite() {
    let signature = FunctionSignature {
        name: "duplicate/0".to_string(),
        params: Vec::new(),
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let first = MirFunction::new(signature.clone(), BasicBlockId::new(1));
    let duplicate = MirFunction::new(signature, BasicBlockId::new(2));
    let mut module = MirModule::new("canonical-duplicate".to_string());
    module.add_function(first);

    let error = publish_function_draft(Some(&mut module), duplicate, true).unwrap_err();

    assert!(matches!(
        error,
        FunctionDraftPublicationErrorV1::Duplicate(_)
    ));
    assert_eq!(
        module.get_function("duplicate/0").unwrap().entry_block,
        BasicBlockId::new(1)
    );
}
