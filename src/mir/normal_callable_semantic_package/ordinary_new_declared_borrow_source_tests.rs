//! Original typed formals share source borrowing; physical activation is separate.
use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog as issue;

fn source(bridge: &str, actual: &str) -> String {
    let (prefix, actual) = match actual {
        "owned_wrap" => ("local obj = new Wrap(0)", "obj"),
        "owned_other" => ("local obj = new Other(0)", "obj"),
        other => ("", other),
    };
    format!(
        "box Wrap {{ v: i64 birth(v) {{ me.v = v }} }}
        box Other {{ v: i64 birth(v) {{ me.v = v }} }}
        box Door {{ birth() {{ }}
            probe(p): i64 {{ if p == null {{ return 0 }} return p.v }}
            bridge(h: Wrap): i64 {{ {bridge} }}
        }}
        static box Main {{ main() {{ local recv = new Door()
            {prefix} local out = recv.bridge({actual}) return 0 }} }}"
    )
}

#[test]
fn declared_formal_and_copy_share_existing_forwarded_source_and_class() {
    for (bridge, actual) in [
        (
            "local recv = new Door() local out = recv.probe(h) return 0",
            "owned_wrap",
        ),
        (
            "local recv = new Door() local out = recv.probe(h) return 0",
            "null",
        ),
        (
            "local alias = h local recv = new Door() local out = recv.probe(alias) return 0",
            "null",
        ),
        (
            "local alias = h local recv = new Door() local out = recv.probe(alias) return 0",
            "owned_wrap",
        ),
    ] {
        let package = issue(&source(bridge, actual)).expect("declared source cohort");
        let prepared = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .expect("source borrowing");
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| {
                row.parameters.iter().any(|row| {
                    matches!(&row.kind, CallableParameterContractKindV1::DeclaredObject(class)
                if class.as_ref() == "Wrap")
                })
            })
            .unwrap();
        let formal = contract.parameters[0].binding;
        assert_eq!(
            prepared.definitions[&contract.owner].origins.get(&formal),
            Some(&formal)
        );
        let view = prepared
            .formal_object_view(formal)
            .expect("declared class even for null input");
        assert!(view.is_declared());
        assert_eq!(view.class(), "Wrap");
        let forwarded = package.ordinary_new_claim_ledger.borrowed_formal_actuals.values()
            .filter_map(|row| row.as_ref().ok()).flat_map(|row| row.opaque_actuals.iter())
            .find(|row| matches!(row.source, super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Forwarded { .. }))
            .expect("same existing Forwarded arm");
        let super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Forwarded {
            binding,
            formal: origin,
        } = forwarded.source
        else {
            unreachable!()
        };
        assert_eq!(origin, formal);
        assert_eq!(binding.owner(), contract.owner);
        let callee_view = prepared.formal_object_view(forwarded.formal).unwrap();
        assert_eq!(callee_view.class(), "Wrap");
        assert!(!callee_view.is_declared());
    }
}

#[test]
fn ignored_declared_formal_rejects_scalar_and_foreign_class_incoming() {
    for actual in ["7", "true", "owned_other"] {
        let error = issue(&source("return 0", actual))
            .err()
            .expect("declared class/null boundary");
        let message = format!("{error:?}");
        assert!(
            message.contains("borrowed-view/declared-object-class")
                || message.contains("borrowed-actual/declared-object-domain")
                || message.contains("borrowed-actual/declared-object-class"),
            "{message}"
        );
    }
}

#[test]
fn declared_object_never_mints_numeric_view_authority() {
    for bridge in [
        "if h > 0 { return 1 } return 0",
        "local alias = h if alias > 0 { return 1 } return 0",
    ] {
        let package = issue(&source(bridge, "null")).expect("outside transport profile");
        let prepared = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| {
                row.parameters.iter().any(|row| {
                    matches!(row.kind, CallableParameterContractKindV1::DeclaredObject(_))
                })
            })
            .unwrap();
        assert!(
            !prepared.definitions.contains_key(&contract.owner),
            "{bridge}"
        );
    }
}

#[test]
fn array_index_requires_numeric_origin_even_with_an_exact_receiver_loan() {
    use super::super::borrowed_formal_uses::{
        draft_borrowed_formal_uses_v1, BorrowedFormalUseDraftKindV1,
    };
    use crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1;
    use crate::mir::resolved_semantics::CallableHomeAbiIssuerV1;

    for (parameter, copy, index, allowed) in [
        ("p", "", "p", true),
        ("p: Wrap", "", "p", false),
        ("p: Wrap", "local alias = p", "alias", false),
    ] {
        let source = format!(
            "box Wrap {{ v: i64 birth(v) {{ me.v = v }} }}
             box Probe {{ sizes: ArrayBox birth() {{ }}
                 probe({parameter}, value): i64 {{ {copy}
                     if value > 0 {{ return 0 }}
                     me.sizes.set({index}, value) return 0 }} }}
             static box Main {{ main() {{ return 0 }} }}"
        );
        let package = issue(&source).expect("original source-backed contracts");
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.parameters.len() == 2)
            .expect("probe contract");
        let catalog = issue_callable_parameter_contract_v1(package.batch())
            .expect("original parameter catalog");
        let loans =
            CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(package.batch(), &catalog)
                .expect("exact instance entry loans");
        let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
            &package.selected,
            package.batch(),
            loans.for_batch_slot(contract.batch_slot),
            contract.batch_slot,
        )
        .expect("Array field belongs to exact receiver");
        let draft = package
            .batch()
            .with_lowering_input(contract.batch_slot, |input| {
                draft_borrowed_formal_uses_v1(
                    input,
                    contract,
                    package.instance_constructors(),
                    Some(receiver),
                )
            })
            .expect("original source loan")
            .expect("argument use draft");
        assert_eq!(
            draft.uses.iter().any(|row| matches!(
                row.kind,
                BorrowedFormalUseDraftKindV1::ArrayElementValue { .. }
            )),
            allowed,
            "index authority: {parameter} / {index}"
        );
    }
}

#[test]
fn declared_rebind_and_escape_stay_outside_the_closed_source_profile() {
    for bridge in [
        "h = null return 0",
        "return h",
        "local alias = h alias = null return 0",
        "local closure = fn() { return h } return 0",
        "local alias = h local closure = fn() { return alias } return 0",
    ] {
        let package = issue(&source(bridge, "null")).expect("unselected source shape");
        let prepared = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| {
                row.parameters.iter().any(|row| {
                    matches!(row.kind, CallableParameterContractKindV1::DeclaredObject(_))
                })
            })
            .unwrap();
        assert!(
            !prepared.definitions.contains_key(&contract.owner),
            "{bridge}"
        );
    }
}
