//! Read-only API corroborates identities issued by the original forward join.
use super::*;

const SOURCE: &str = "box Item { value: i64 birth() { me.value = 5 } }
    box Leaf { flag: i64 birth() { me.flag = 0 }
        read(p: Item) { if p == null { return 7 } return p.value } }
    box Parent { child: Leaf birth() { me.child = new Leaf() }
        run(p: Item) { return me.child.read(p) } }
    static box Main { main() { local parent = new Parent() local item = new Item()
        return parent.run(item) } }";

#[test]
fn forward_identity_loan_keeps_original_contract_use_and_call_coordinates() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(SOURCE)
        .expect("source-issued forwarding caller");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let site = source
        .incoming
        .iter()
        .find(|row| row.source.stored_receiver().is_some())
        .unwrap()
        .source
        .call_site();
    let target = package
        .ordinary_new_claim_ledger
        .take_lexical_instance_call(site.owner(), site.site())
        .unwrap()
        .unwrap()
        .source_target()
        .clone();
    let need = PreparedSourceCallNeedV1::Lexical(target.clone());
    let forwards = forward_identities_v1(&need, &package.parameter_contracts, &source.definitions)
        .expect("original forward issuer");
    assert_eq!(forwards.len(), 1);
    let row = &forwards[0];
    let draft = &source.definitions[&target.call_site().owner()];
    let original = draft
        .uses
        .iter()
        .find(|use_row| use_row.site == *row.site())
        .unwrap();
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == target.callee_owner())
        .unwrap();
    let formal = &contract.parameters[row.ordinal() as usize];
    assert_eq!(row.binding(), original.binding);
    assert_eq!(row.source_formal(), original.formal);
    assert_eq!(draft.origins[&row.binding()], row.source_formal());
    assert_eq!(row.call(), target.call_site());
    assert_eq!(row.target(), target.target());
    assert_eq!(
        row.site().site(),
        &target.argument_sites()[row.ordinal() as usize]
    );
    assert_eq!(row.callee_formal(), formal.binding);
    assert_eq!(row.ordinal(), formal.ordinal);
    assert_eq!(row, &row.clone());
}

#[test]
fn forward_identity_loan_retains_missing_duplicate_and_foreign_use_rejection() {
    for mutation in 0..3 {
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(SOURCE)
            .expect("source-issued forwarding caller");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let site = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .incoming
            .iter()
            .find(|row| row.source.stored_receiver().is_some())
            .unwrap()
            .source
            .call_site()
            .clone();
        let target = ledger
            .take_lexical_instance_call(site.owner(), site.site())
            .unwrap()
            .unwrap()
            .source_target()
            .clone();
        let source = ledger
            .borrowed_formal_source
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap();
        let need = PreparedSourceCallNeedV1::Lexical(target.clone());
        let forwards =
            forward_identities_v1(&need, &package.parameter_contracts, &source.definitions)
                .unwrap();
        let original = &forwards[0];
        let draft = source
            .definitions
            .get_mut(&target.call_site().owner())
            .unwrap();
        let mut uses = std::mem::take(&mut draft.uses).into_vec();
        let index = uses
            .iter()
            .position(|row| row.site == *original.site())
            .unwrap();
        match mutation {
            0 => {
                uses.remove(index);
            }
            1 => {
                let row = &uses[index];
                let duplicate =
                    super::super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftRowV1 {
                        site: row.site.clone(),
                        binding: row.binding,
                        formal: row.formal,
                        kind: BorrowedFormalUseDraftKindV1::UnresolvedArgument {
                            call: target.call_site().clone(),
                            ordinal: original.ordinal(),
                        },
                    };
                uses.push(duplicate);
            }
            _ => {
                uses[index].site =
                    OwnedExprSiteV1::new(target.callee_owner(), original.site().site().clone())
            }
        }
        draft.uses = uses.into_boxed_slice();
        assert!(
            forward_identities_v1(&need, &package.parameter_contracts, &source.definitions)
                .is_none(),
            "mutation={mutation}"
        );
    }
}
