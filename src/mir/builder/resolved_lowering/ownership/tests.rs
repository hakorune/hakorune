use hakorune_mir_core::BindingId;

use crate::mir::ownership_ssa::FunctionResultOwnershipV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1};
use crate::mir::ValueId;

use super::*;

fn make_owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn subject(owner: FunctionOwnerIdV1, slot: u32) -> LocalBindingSubjectV1 {
    LocalBindingSubjectV1::new(
        BindingRefV1::new(owner, BindingId::new(slot)),
        LocalBindingClassV1::Local,
    )
}

fn value(raw: u32) -> ValueId {
    ValueId::new(raw)
}

fn owned(raw: u32) -> OwnedValueIdV1 {
    OwnedValueIdV1::new(value(raw))
}

fn closing(owner: FunctionOwnerIdV1, binding: u32, current: u32) -> OwnedBindingAtCloseV1 {
    OwnedBindingAtCloseV1::new(subject(owner, binding), owned(current))
}

#[test]
fn assignment_plan_encodes_next_before_previous() {
    let owner = make_owner();
    let target = subject(owner, 0);
    let source = subject(owner, 1);
    let plan = plan_assignment(
        target,
        Some(owned(10)),
        LoweredValueOwnershipV1::BorrowedStrong {
            binding: source,
            value: value(11),
        },
    )
    .unwrap();
    let AssignmentOwnershipPlanV1::Replace(plan) = plan else {
        panic!("borrowed assignment must replace")
    };
    assert_eq!(plan.target(), target);
    assert_eq!(
        plan.next(),
        NextBindingValuePlanV1::CopyBorrowedStrong {
            source,
            value: value(11),
        }
    );
    assert_eq!(plan.previous_after_commit(), Some(owned(10)));
}

#[test]
fn exact_binding_provenance_is_the_only_self_assignment_authority() {
    let owner = make_owner();
    let target = subject(owner, 0);
    let exact = plan_assignment(
        target,
        Some(owned(10)),
        LoweredValueOwnershipV1::BorrowedStrong {
            binding: target,
            value: value(10),
        },
    )
    .unwrap();
    assert_eq!(
        exact,
        AssignmentOwnershipPlanV1::ExactSelfAssignment { binding: target }
    );

    let other = subject(owner, 1);
    let same_raw_value = plan_assignment(
        target,
        Some(owned(10)),
        LoweredValueOwnershipV1::BorrowedStrong {
            binding: other,
            value: value(10),
        },
    )
    .unwrap();
    assert!(matches!(
        same_raw_value,
        AssignmentOwnershipPlanV1::Replace(ReplaceBindingOwnershipPlanV1 { .. })
    ));
}

#[test]
fn assignment_transfers_owned_and_reuses_trivial_values() {
    let owner = make_owner();
    let target = subject(owner, 0);
    let transferred = plan_assignment(
        target,
        Some(owned(10)),
        LoweredValueOwnershipV1::Owned { value: owned(11) },
    )
    .unwrap();
    let AssignmentOwnershipPlanV1::Replace(transferred) = transferred else {
        panic!("owned assignment must replace")
    };
    assert_eq!(
        transferred.next(),
        NextBindingValuePlanV1::TransferOwned { value: owned(11) }
    );
    assert_eq!(transferred.previous_after_commit(), Some(owned(10)));

    let trivial = plan_assignment(
        target,
        None,
        LoweredValueOwnershipV1::Trivial { value: value(12) },
    )
    .unwrap();
    let AssignmentOwnershipPlanV1::Replace(trivial) = trivial else {
        panic!("trivial assignment must replace")
    };
    assert_eq!(
        trivial.next(),
        NextBindingValuePlanV1::ReuseTrivial { value: value(12) }
    );
    assert_eq!(trivial.previous_after_commit(), None);
}

#[test]
fn assignment_rejects_one_owned_token_as_next_and_previous() {
    let owner = make_owner();
    assert!(matches!(
        plan_assignment(
            subject(owner, 0),
            Some(owned(10)),
            LoweredValueOwnershipV1::Owned { value: owned(10) },
        ),
        Err(OwnershipTransitionErrorV1::OwnedNextAliasesPrevious {
            value: actual
        }) if actual == value(10)
    ));
}

#[test]
fn declaration_uses_the_same_closed_next_value_vocabulary() {
    let owner = make_owner();
    let target = LocalBindingSubjectV1::new(
        BindingRefV1::new(owner, BindingId::new(0)),
        LocalBindingClassV1::Outbox,
    );
    let source = LocalBindingSubjectV1::new(
        BindingRefV1::new(owner, BindingId::new(1)),
        LocalBindingClassV1::Parameter,
    );
    let plan = plan_declaration(
        target,
        LoweredValueOwnershipV1::BorrowedStrong {
            binding: source,
            value: value(20),
        },
    )
    .unwrap();
    assert_eq!(plan.target(), target);
    assert_eq!(target.class(), LocalBindingClassV1::Outbox);
    assert_eq!(
        plan.next(),
        NextBindingValuePlanV1::CopyBorrowedStrong {
            source,
            value: value(20),
        }
    );
}

#[test]
fn declaration_transfers_owned_and_reuses_trivial_without_runtime_inference() {
    let owner = make_owner();
    let receiver = LocalBindingSubjectV1::new(
        BindingRefV1::new(owner, BindingId::new(0)),
        LocalBindingClassV1::Receiver,
    );
    let transferred = plan_declaration(
        receiver,
        LoweredValueOwnershipV1::Owned { value: owned(20) },
    )
    .unwrap();
    assert_eq!(receiver.class(), LocalBindingClassV1::Receiver);
    assert_eq!(
        transferred.next(),
        NextBindingValuePlanV1::TransferOwned { value: owned(20) }
    );

    let local = subject(owner, 1);
    let trivial =
        plan_declaration(local, LoweredValueOwnershipV1::Trivial { value: value(21) }).unwrap();
    assert_eq!(
        trivial.next(),
        NextBindingValuePlanV1::ReuseTrivial { value: value(21) }
    );
    assert_eq!(
        LoweredValueOwnershipV1::Owned { value: owned(20) }.value(),
        value(20)
    );
}

#[test]
fn borrowed_inputs_from_a_foreign_function_fail() {
    let owner = make_owner();
    let foreign = make_owner();
    let error = plan_declaration(
        subject(owner, 0),
        LoweredValueOwnershipV1::BorrowedStrong {
            binding: subject(foreign, 1),
            value: value(20),
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        OwnershipTransitionErrorV1::ForeignOwner {
            expected,
            actual,
            ..
        } if expected == owner && actual == foreign
    ));
}

#[test]
fn scope_local_tail_transfers_and_reverse_destroys_the_rest() {
    let owner = make_owner();
    let declarations = [
        closing(owner, 0, 10),
        closing(owner, 1, 11),
        closing(owner, 2, 12),
    ];
    let tail_binding = subject(owner, 1);
    let plan = plan_scope_close(
        owner,
        &declarations,
        ScopeTailOwnershipV1::ScopeLocal {
            binding: tail_binding,
            value: owned(11),
        },
    )
    .unwrap();
    assert_eq!(
        plan.result(),
        ScopeResultOwnershipPlanV1::TransferScopeLocal {
            binding: tail_binding,
            value: owned(11),
        }
    );
    assert_eq!(plan.destroys_in_order(), &[owned(12), owned(10)]);
}

#[test]
fn outer_borrowed_tail_copies_then_closes_every_local() {
    let owner = make_owner();
    let outer = subject(owner, 9);
    let declarations = [closing(owner, 0, 10), closing(owner, 1, 11)];
    let plan = plan_scope_close(
        owner,
        &declarations,
        ScopeTailOwnershipV1::OuterBorrowed {
            binding: outer,
            value: value(30),
        },
    )
    .unwrap();
    assert_eq!(
        plan.result(),
        ScopeResultOwnershipPlanV1::CopyOuterBorrowed {
            source: outer,
            value: value(30),
        }
    );
    assert_eq!(plan.destroys_in_order(), &[owned(11), owned(10)]);
}

#[test]
fn scope_tail_requires_explicit_local_or_outer_provenance() {
    let owner = make_owner();
    let local = subject(owner, 0);
    let declarations = [closing(owner, 0, 10)];
    assert!(matches!(
        plan_scope_close(
            owner,
            &declarations,
            ScopeTailOwnershipV1::OuterBorrowed {
                binding: local,
                value: value(10),
            },
        ),
        Err(OwnershipTransitionErrorV1::OuterBorrowedTailIsScopeLocal { .. })
    ));
    assert!(matches!(
        plan_scope_close(
            owner,
            &declarations,
            ScopeTailOwnershipV1::ForwardOwned { value: owned(10) },
        ),
        Err(OwnershipTransitionErrorV1::ForwardedOwnedStillOwnedByScope { .. })
    ));
}

#[test]
fn scope_local_tail_requires_the_current_binding_ssa_value() {
    let owner = make_owner();
    let local = subject(owner, 0);
    let declarations = [closing(owner, 0, 10)];
    assert!(matches!(
        plan_scope_close(
            owner,
            &declarations,
            ScopeTailOwnershipV1::ScopeLocal {
                binding: local,
                value: owned(11),
            },
        ),
        Err(OwnershipTransitionErrorV1::ScopeLocalTailValueMismatch {
            expected,
            actual,
            ..
        }) if expected == value(10) && actual == value(11)
    ));
    assert!(matches!(
        plan_scope_close(
            owner,
            &declarations,
            ScopeTailOwnershipV1::ScopeLocal {
                binding: subject(owner, 1),
                value: owned(11),
            },
        ),
        Err(OwnershipTransitionErrorV1::ScopeLocalTailMissing { .. })
    ));
}

#[test]
fn temporary_and_trivial_scope_results_keep_reverse_close_order() {
    let owner = make_owner();
    let declarations = [closing(owner, 0, 10), closing(owner, 1, 11)];
    let forwarded = plan_scope_close(
        owner,
        &declarations,
        ScopeTailOwnershipV1::ForwardOwned { value: owned(20) },
    )
    .unwrap();
    assert_eq!(
        forwarded.result(),
        ScopeResultOwnershipPlanV1::ForwardOwned { value: owned(20) }
    );
    assert_eq!(forwarded.destroys_in_order(), &[owned(11), owned(10)]);

    let trivial = plan_scope_close(
        owner,
        &declarations,
        ScopeTailOwnershipV1::Trivial { value: value(21) },
    )
    .unwrap();
    assert_eq!(
        trivial.result(),
        ScopeResultOwnershipPlanV1::ReuseTrivial { value: value(21) }
    );
    assert_eq!(trivial.destroys_in_order(), &[owned(11), owned(10)]);
}

#[test]
fn closing_roots_must_be_unique_and_owner_local() {
    let owner = make_owner();
    let duplicate_binding = [closing(owner, 0, 10), closing(owner, 0, 11)];
    assert!(matches!(
        plan_scope_close(owner, &duplicate_binding, ScopeTailOwnershipV1::None),
        Err(OwnershipTransitionErrorV1::DuplicateClosingBinding { .. })
    ));

    let duplicate_token = [closing(owner, 0, 10), closing(owner, 1, 10)];
    assert!(matches!(
        plan_scope_close(owner, &duplicate_token, ScopeTailOwnershipV1::None),
        Err(OwnershipTransitionErrorV1::DuplicateOwnedToken { .. })
    ));

    let foreign = make_owner();
    assert!(matches!(
        plan_scope_close(
            owner,
            &[closing(foreign, 0, 10)],
            ScopeTailOwnershipV1::None,
        ),
        Err(OwnershipTransitionErrorV1::ForeignOwner { .. })
    ));
}

#[test]
fn function_borrowed_return_copies_before_reverse_root_destroy() {
    let owner = make_owner();
    let roots = [closing(owner, 0, 10), closing(owner, 1, 11)];
    let source = subject(owner, 0);
    let plan = plan_function_exit(
        owner,
        &roots,
        FunctionResultOwnershipV1::Owned,
        FunctionTerminalOwnershipV1::Return(LoweredValueOwnershipV1::BorrowedStrong {
            binding: source,
            value: value(10),
        }),
    )
    .unwrap();
    assert_eq!(
        plan.result(),
        FunctionTerminalResultPlanV1::Return(NextBindingValuePlanV1::CopyBorrowedStrong {
            source,
            value: value(10),
        })
    );
    assert_eq!(plan.destroys_in_order(), &[owned(11), owned(10)]);
}

#[test]
fn function_owned_temporary_return_transfers_without_copy() {
    let owner = make_owner();
    let roots = [closing(owner, 0, 10)];
    let plan = plan_function_exit(
        owner,
        &roots,
        FunctionResultOwnershipV1::Owned,
        FunctionTerminalOwnershipV1::Return(LoweredValueOwnershipV1::Owned { value: owned(20) }),
    )
    .unwrap();
    assert_eq!(
        plan.result(),
        FunctionTerminalResultPlanV1::Return(NextBindingValuePlanV1::TransferOwned {
            value: owned(20),
        })
    );
    assert_eq!(plan.destroys_in_order(), &[owned(10)]);
}

#[test]
fn trivial_return_closes_every_owned_root() {
    let owner = make_owner();
    let roots = [closing(owner, 0, 10), closing(owner, 1, 11)];
    let plan = plan_function_exit(
        owner,
        &roots,
        FunctionResultOwnershipV1::None,
        FunctionTerminalOwnershipV1::Return(LoweredValueOwnershipV1::Trivial { value: value(20) }),
    )
    .unwrap();
    assert_eq!(
        plan.result(),
        FunctionTerminalResultPlanV1::Return(NextBindingValuePlanV1::ReuseTrivial {
            value: value(20),
        })
    );
    assert_eq!(plan.destroys_in_order(), &[owned(11), owned(10)]);
}

#[test]
fn function_result_profile_and_fallthrough_are_sealed() {
    let owner = make_owner();
    let roots = [closing(owner, 0, 10)];
    let fallthrough = plan_function_exit(
        owner,
        &roots,
        FunctionResultOwnershipV1::None,
        FunctionTerminalOwnershipV1::Fallthrough,
    )
    .unwrap();
    assert_eq!(
        fallthrough.result(),
        FunctionTerminalResultPlanV1::Fallthrough
    );
    assert_eq!(fallthrough.destroys_in_order(), &[owned(10)]);

    assert!(matches!(
        plan_function_exit(
            owner,
            &roots,
            FunctionResultOwnershipV1::Owned,
            FunctionTerminalOwnershipV1::Return(LoweredValueOwnershipV1::Trivial {
                value: value(20),
            }),
        ),
        Err(OwnershipTransitionErrorV1::ResultOwnershipMismatch {
            expected: FunctionResultOwnershipV1::Owned,
            actual: FunctionResultOwnershipV1::None,
        })
    ));
}

#[test]
fn unpublished_draft_discard_has_no_runtime_action_surface() {
    assert_eq!(
        plan_unpublished_draft_discard(),
        UnpublishedDraftDiscardOwnershipPlanV1
    );
}
