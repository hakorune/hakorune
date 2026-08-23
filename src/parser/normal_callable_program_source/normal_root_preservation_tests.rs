use crate::parser::callable_parameter_source::{
    ParserNormalRootPreservationIssuerV1, ParserNormalRootPreservationRejectV1,
    ParserNormalRootPreservationV1, ParserNormalRootRoleV1,
};
use crate::parser::initial_callable_program_source::InitialCallableFinalSlotV1;
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{ParsedNormalCallableProgramV1, PreparedNormalCallableProgramSourceV1};

fn prepared(source: &str) -> PreparedNormalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let ParsedNormalCallableProgramV1::SourceBacked(prepared) = parsed else {
        panic!("fixture must remain source-backed")
    };
    prepared
}

#[test]
fn app_root_relation_accepts_exact_main_with_top_level_callable_sibling() {
    let final_source =
        prepared("function helper() { return 2 }\nstatic box Main { main() { return 1 } }")
            .begin_transform()
            .finish_exact()
            .expect("exact App root relation");

    assert!(matches!(
        final_source.normal_root_source(),
        ParserNormalRootPreservationV1::Ready(preserved)
            if preserved.role() == ParserNormalRootRoleV1::App
    ));
}

#[test]
fn app_root_relation_rejects_structurally_equal_foreign_callable_identity() {
    let first = prepared("static box Main { main() { return 1 } }");
    let foreign = prepared("static box Main { main() { return 1 } }");
    let (ast, _, _, _, authority, _, root) = first.into_transform_parts();
    let (_, foreign_rows, foreign_slots, _, _, _, _) = foreign.into_transform_parts();

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root,
        &authority,
        &ast,
        &ast,
        &foreign_rows,
        &foreign_slots,
    )
    .expect_err("foreign opaque identity must not pair by shape or ordinal");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::AppCallableIdentityMissing
    );
}

#[test]
fn app_root_relation_rejects_foreign_parser_witness_before_pairing() {
    let first = prepared("static box Main { main() { return 1 } }");
    let foreign = prepared("static box Main { main() { return 1 } }");
    let (ast, rows, slots, _, _, _, root) = first.into_transform_parts();
    let (_, _, _, _, foreign_authority, _, _) = foreign.into_transform_parts();

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root,
        &foreign_authority,
        &ast,
        &ast,
        &rows,
        &slots,
    )
    .expect_err("foreign parser witness must reject before App pairing");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::ParserWitnessMismatch
    );
}

#[test]
fn app_root_relation_rejects_unpaired_final_slot() {
    let source = prepared("static box Main { main() { return 1 } }");
    let (ast, rows, slots, _, authority, _, root) = source.into_transform_parts();
    let mut slots = slots.into_vec();
    slots[0] = InitialCallableFinalSlotV1::TopLevel { statement: 0 };

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root, &authority, &ast, &ast, &rows, &slots,
    )
    .expect_err("App identity must retain its paired BoxMethod slot");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::AppCallableFinalSlotMismatch
    );
}

#[test]
fn app_root_relation_rejects_callable_pairing_cardinality_drift() {
    let source = prepared("static box Main { main() { return 1 } }");
    let (ast, rows, _, _, authority, _, root) = source.into_transform_parts();

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root,
        &authority,
        &ast,
        &ast,
        &rows,
        &[],
    )
    .expect_err("parallel callable/slot drift must reject");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::CallablePairingCardinalityMismatch {
            sources: 1,
            slots: 0,
        }
    );
}

#[test]
fn main_helper_stays_terminal_before_app_root_relation() {
    let final_source = prepared("static box Main { main() { return 1 } helper() { return 2 } }")
        .begin_transform()
        .finish_exact()
        .expect("typed non-ready Main source");

    assert!(matches!(
        final_source.normal_root_source(),
        ParserNormalRootPreservationV1::Terminal(_)
    ));
}
