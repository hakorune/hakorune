use super::*;
use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIssuerV1};

fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .expect("test issuer")
        .issue()
        .expect("test owner")
}

fn helper(name: &str, arity: u32) -> NormalModuleDraftExpectationV1 {
    NormalModuleDraftExpectationV1::helper(
        CanonicalCallableKeyV1::free_static_for_test(name, arity),
        format!("{name}/{arity}"),
        arity as usize,
    )
}

fn relation(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> NormalModuleEntryRelationV1 {
    NormalModuleEntryRelationV1::new(owner, "Main.main/0", 0, "main", 0)
}

fn base_rows(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Vec<NormalModuleDraftExpectationV1> {
    vec![
        NormalModuleDraftExpectationV1::source_main(owner, "Main.main/0", 0),
        NormalModuleDraftExpectationV1::physical_entry("main", 0),
    ]
}

fn seal(
    rows: Vec<NormalModuleDraftExpectationV1>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<NormalModuleTransactionSchemaV1, RejectedNormalModuleTransactionSchemaV1> {
    NormalModuleTransactionSchemaV1::seal(NormalModuleTransactionDraftV1::new(
        rows,
        relation(owner),
    ))
}

#[test]
fn main_only_and_helper_batches_seal_in_deterministic_role_order() {
    let main_owner = owner();
    let main_only = seal(base_rows(main_owner), main_owner).expect("Main-only schema");
    assert!(matches!(
        main_only.rows()[0].role(),
        NormalModuleDraftRoleV1::SourceMain { .. }
    ));
    assert!(matches!(
        main_only.rows()[1].role(),
        NormalModuleDraftRoleV1::PhysicalEntry
    ));

    let first_key = CanonicalCallableKeyV1::free_static_for_test("a", 1);
    let second_key = CanonicalCallableKeyV1::free_static_for_test("z", 2);
    let mut rows = vec![
        NormalModuleDraftExpectationV1::physical_entry("main", 0),
        NormalModuleDraftExpectationV1::helper(second_key, "z/2", 2),
        NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
        NormalModuleDraftExpectationV1::helper(first_key, "a/1", 1),
    ];
    let schema = seal(rows, main_owner).expect("heterogeneous schema");
    let symbols = schema
        .rows()
        .iter()
        .map(NormalModuleDraftExpectationV1::symbol)
        .collect::<Vec<_>>();
    assert_eq!(symbols, ["Main.main/0", "a/1", "z/2", "main"]);

    rows = vec![
        helper("a", 1),
        NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
        NormalModuleDraftExpectationV1::physical_entry("main", 0),
        helper("z", 2),
    ];
    let reordered = seal(rows, main_owner).expect("reordered schema");
    assert_eq!(schema, reordered);
    assert_eq!(schema.source_entry(), reordered.source_entry());
}

#[test]
fn missing_and_duplicate_required_roles_are_typed_and_retained() {
    let main_owner = owner();
    for (rows, expected) in [
        (
            vec![NormalModuleDraftExpectationV1::physical_entry("main", 0)],
            NormalModuleTransactionSchemaErrorV1::MissingSourceMain,
        ),
        (
            vec![NormalModuleDraftExpectationV1::source_main(
                main_owner,
                "Main.main/0",
                0,
            )],
            NormalModuleTransactionSchemaErrorV1::MissingPhysicalEntry,
        ),
        (
            vec![
                NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
                NormalModuleDraftExpectationV1::source_main(main_owner, "Main.copy/0", 0),
                NormalModuleDraftExpectationV1::physical_entry("main", 0),
            ],
            NormalModuleTransactionSchemaErrorV1::DuplicateSourceMain,
        ),
        (
            vec![
                NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
                NormalModuleDraftExpectationV1::physical_entry("main", 0),
                NormalModuleDraftExpectationV1::physical_entry("entry", 0),
            ],
            NormalModuleTransactionSchemaErrorV1::DuplicatePhysicalEntry,
        ),
    ] {
        let rejected = seal(rows, main_owner).expect_err("typed cardinality rejection");
        assert_eq!(rejected.error(), &expected);
        assert!(!rejected.owner().rows().is_empty());
        rejected.discard();
    }
}

#[test]
fn duplicate_key_and_symbol_are_rejected_before_any_mutation() {
    let main_owner = owner();
    let duplicate_key = CanonicalCallableKeyV1::free_static_for_test("same", 1);
    let rows = vec![
        NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
        NormalModuleDraftExpectationV1::helper(duplicate_key.clone(), "first/1", 1),
        NormalModuleDraftExpectationV1::helper(duplicate_key.clone(), "second/1", 1),
        NormalModuleDraftExpectationV1::physical_entry("main", 0),
    ];
    let rejected = seal(rows, main_owner).expect_err("duplicate key");
    assert_eq!(
        rejected.error(),
        &NormalModuleTransactionSchemaErrorV1::DuplicateKey(FunctionDraftKeyV1::CanonicalCallable(
            duplicate_key
        ))
    );
    rejected.discard();

    let rows = vec![
        NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
        helper("a", 1),
        NormalModuleDraftExpectationV1::helper(
            CanonicalCallableKeyV1::free_static_for_test("b", 1),
            "a/1",
            1,
        ),
        NormalModuleDraftExpectationV1::physical_entry("main", 0),
    ];
    let rejected = seal(rows, main_owner).expect_err("duplicate symbol");
    assert_eq!(
        rejected.error(),
        &NormalModuleTransactionSchemaErrorV1::DuplicateSymbol("a/1".into())
    );
    rejected.discard();
}

#[test]
fn role_key_arity_and_entry_relation_drift_are_typed() {
    let main_owner = owner();
    let helper_key = CanonicalCallableKeyV1::free_static_for_test("helper", 1);
    let malformed = NormalModuleDraftExpectationV1::from_unchecked_parts_for_test(
        NormalModuleDraftRoleV1::SourceMain { owner: main_owner },
        FunctionDraftKeyV1::CanonicalCallable(helper_key),
        "Main.main/0",
        0,
    );
    let rejected = seal(
        vec![
            malformed,
            NormalModuleDraftExpectationV1::physical_entry("main", 0),
        ],
        main_owner,
    )
    .expect_err("role/key mismatch");
    assert_eq!(
        rejected.error(),
        &NormalModuleTransactionSchemaErrorV1::RoleKeyMismatch
    );
    rejected.discard();

    for rows in [
        vec![
            NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 1),
            NormalModuleDraftExpectationV1::physical_entry("main", 0),
        ],
        vec![
            NormalModuleDraftExpectationV1::source_main(main_owner, "Main.main/0", 0),
            NormalModuleDraftExpectationV1::physical_entry("main", 1),
        ],
    ] {
        let rejected = seal(rows, main_owner).expect_err("zero-arity relation");
        assert_eq!(
            rejected.error(),
            &NormalModuleTransactionSchemaErrorV1::ArityMismatch
        );
        rejected.discard();
    }

    let rows = base_rows(main_owner);
    let rejected = NormalModuleTransactionSchemaV1::seal(NormalModuleTransactionDraftV1::new(
        rows,
        NormalModuleEntryRelationV1::new(main_owner, "Other.main/0", 0, "main", 0),
    ))
    .expect_err("entry relation drift");
    assert_eq!(
        rejected.error(),
        &NormalModuleTransactionSchemaErrorV1::EntryRelationMismatch
    );
    rejected.discard();
}
