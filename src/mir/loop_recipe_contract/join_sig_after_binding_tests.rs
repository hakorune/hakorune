use super::ids::{LoopBindingKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::{
    port_bindings, LoopJoinEdgeRoleV1, LoopJoinEdgeV1, LoopJoinLoopV1, LoopJoinPayloadV1,
    LoopJoinPortV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1,
};
use super::schema::{
    LoopConditionV1, LoopNodeV1, LoopRecipeBlockV1, LoopRecipeV1, LoopValueClassV1,
};
use super::verify::LoopRecipeVerifierV1;

fn nested_signature() -> super::join_sig::VerifiedLoopJoinSigV1 {
    let artifact: super::schema::LoopRecipeArtifactV1 =
        serde_json::from_str(include_str!("fixtures/nested_predicate_v1.json"))
            .expect("nested fixture");
    let verified = LoopRecipeVerifierV1::verify(artifact.recipe).expect("recipe verifies");
    LoopJoinSigElaboratorV1::elaborate(&verified).expect("join signature")
}

fn payload(binding: u32, value: u32, class: LoopValueClassV1) -> LoopJoinPayloadV1 {
    LoopJoinPayloadV1 {
        binding: LoopBindingKeyV1::new(binding),
        value: LoopValueKeyV1::new(value),
        class,
    }
}

fn row_with_edges(edges: Vec<LoopJoinEdgeV1>) -> LoopJoinLoopV1 {
    LoopJoinLoopV1 {
        key: LoopNodeKeyV1::new(0),
        parent: None,
        condition: None,
        carriers: Vec::new(),
        edges,
    }
}

fn empty_always_signature() -> super::join_sig::VerifiedLoopJoinSigV1 {
    let recipe = LoopRecipeV1 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![LoopNodeV1 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV1::Always,
            body: super::ids::LoopBlockKeyV1::new(0),
        }],
        blocks: vec![LoopRecipeBlockV1 {
            key: super::ids::LoopBlockKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            items: Vec::new(),
        }],
        items: Vec::new(),
        bindings: Vec::new(),
        values: Vec::new(),
        inputs: Vec::new(),
        carriers: Vec::new(),
        exits: Vec::new(),
    };
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("empty Always recipe");
    LoopJoinSigElaboratorV1::elaborate(&verified).expect("empty Always signature")
}

#[test]
fn port_bindings_are_sorted_and_after_capability_is_opaque() {
    let signature = nested_signature();
    let rows = &signature.as_sig().port_bindings;
    assert!(rows.windows(2).all(|pair| {
        (pair[0].loop_key, pair[0].port, pair[0].binding)
            <= (pair[1].loop_key, pair[1].port, pair[1].binding)
    }));

    let root_after: Vec<_> = rows
        .iter()
        .filter(|row| row.loop_key == LoopNodeKeyV1::new(0) && row.port == LoopJoinPortV1::After)
        .map(|row| row.binding.raw())
        .collect();
    let child_after: Vec<_> = rows
        .iter()
        .filter(|row| row.loop_key == LoopNodeKeyV1::new(1) && row.port == LoopJoinPortV1::After)
        .map(|row| row.binding.raw())
        .collect();
    assert_eq!(root_after, vec![0, 1]);
    assert_eq!(child_after, vec![0, 1, 2]);

    let capability = signature
        .require_after_binding(
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(1),
            LoopValueClassV1::I64,
        )
        .expect("root After binding");
    assert_eq!(capability.loop_key(), LoopNodeKeyV1::new(0));
    assert_eq!(capability.binding(), LoopBindingKeyV1::new(1));
    assert_eq!(capability.class(), LoopValueClassV1::I64);
}

#[test]
fn after_capability_rejects_wrong_owner_binding_and_class() {
    let signature = nested_signature();
    assert_eq!(
        signature.require_after_binding(
            LoopNodeKeyV1::new(99),
            LoopBindingKeyV1::new(1),
            LoopValueClassV1::I64,
        ),
        Err(LoopJoinSigRejectReasonV1::AfterBindingUnavailable {
            loop_key: LoopNodeKeyV1::new(99),
            binding: LoopBindingKeyV1::new(1),
        })
    );
    assert_eq!(
        signature.require_after_binding(
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(99),
            LoopValueClassV1::I64,
        ),
        Err(LoopJoinSigRejectReasonV1::AfterBindingUnavailable {
            loop_key: LoopNodeKeyV1::new(0),
            binding: LoopBindingKeyV1::new(99),
        })
    );
    assert_eq!(
        signature.require_after_binding(
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(1),
            LoopValueClassV1::Bool,
        ),
        Err(LoopJoinSigRejectReasonV1::AfterBindingClassMismatch {
            loop_key: LoopNodeKeyV1::new(0),
            port: LoopJoinPortV1::After,
            binding: LoopBindingKeyV1::new(1),
        })
    );
}

#[test]
fn no_after_edges_produce_no_after_capability_rows() {
    let signature = empty_always_signature();
    assert!(signature.as_sig().port_bindings.is_empty());
    assert_eq!(
        signature.require_after_binding(
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(0),
            LoopValueClassV1::I64,
        ),
        Err(LoopJoinSigRejectReasonV1::AfterBindingUnavailable {
            loop_key: LoopNodeKeyV1::new(0),
            binding: LoopBindingKeyV1::new(0),
        })
    );
}

#[test]
fn incoming_port_sets_must_match_without_duplicate_payloads() {
    let base = payload(0, 0, LoopValueClassV1::I64);
    let extra = payload(1, 1, LoopValueClassV1::I64);
    let mismatch = row_with_edges(vec![
        LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Preheader,
            to: LoopJoinPortV1::Header,
            role: LoopJoinEdgeRoleV1::Enter,
            payload: vec![base.clone()],
        },
        LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::Header,
            role: LoopJoinEdgeRoleV1::Backedge,
            payload: vec![base.clone(), extra],
        },
    ]);
    assert_eq!(
        port_bindings(&[mismatch]),
        Err(LoopJoinSigRejectReasonV1::PortBindingSetMismatch {
            loop_key: LoopNodeKeyV1::new(0),
            port: LoopJoinPortV1::Header,
        })
    );

    let duplicate = row_with_edges(vec![LoopJoinEdgeV1 {
        from: LoopJoinPortV1::Preheader,
        to: LoopJoinPortV1::After,
        role: LoopJoinEdgeRoleV1::Break,
        payload: vec![base.clone(), base],
    }]);
    assert_eq!(
        port_bindings(&[duplicate]),
        Err(LoopJoinSigRejectReasonV1::DuplicatePortBinding {
            loop_key: LoopNodeKeyV1::new(0),
            port: LoopJoinPortV1::After,
            binding: LoopBindingKeyV1::new(0),
        })
    );
}

#[test]
fn incoming_port_classes_must_match() {
    let row = row_with_edges(vec![
        LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Preheader,
            to: LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::Break,
            payload: vec![payload(0, 0, LoopValueClassV1::I64)],
        },
        LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::Break,
            payload: vec![payload(0, 1, LoopValueClassV1::Bool)],
        },
    ]);
    assert_eq!(
        port_bindings(&[row]),
        Err(LoopJoinSigRejectReasonV1::PortBindingClassMismatch {
            loop_key: LoopNodeKeyV1::new(0),
            port: LoopJoinPortV1::After,
            binding: LoopBindingKeyV1::new(0),
        })
    );
}
