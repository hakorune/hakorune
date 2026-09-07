use super::super::tests::source;
use super::*;

#[test]
fn recipe_fixes_fault_cleanup_and_terminal_order_before_any_emission() {
    for spec in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"] {
        let (product, window) = source(&format!(
            "local a: Array<{spec}> = [10, 20]\nlocal alias = a\nlocal b: Array<{spec}> = []\nreturn 30"
        ), 0);
        let rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        let mut recipe = rows.lowering_recipe().unwrap();
        let relations: Vec<_> = product
            .expression_source()
            .initializers()
            .filter(|relation| relation.declared_type_name().is_some())
            .cloned()
            .collect();
        assert_eq!(relations.len(), 2);
        assert!(recipe.finish().unwrap_err().contains("recipe-unconsumed"));
        let terminal_site = rows.terminal().unwrap().unwrap().site().node().clone();
        assert!(recipe
            .take_return(&terminal_site)
            .unwrap_err()
            .contains("before-locals"));
        let first = recipe.take_local(&relations[0]).unwrap().unwrap();
        assert_eq!(first.spec().element.source_name(), spec);
        assert_eq!(first.elements().len(), 2);
        assert!(first.allocation_fault().is_empty());
        assert_eq!(
            first.acquired_fault(),
            &[ArrayReleaseRoleV1::IncompleteResidence]
        );
        assert!(recipe
            .take_local(&relations[0])
            .unwrap_err()
            .contains("already-taken"));
        let second = recipe.take_local(&relations[1]).unwrap().unwrap();
        assert!(second.elements().is_empty());
        assert_eq!(
            second.allocation_fault(),
            &[ArrayReleaseRoleV1::Home(relations[0].binding())]
        );
        assert_eq!(
            second.acquired_fault(),
            &[
                ArrayReleaseRoleV1::IncompleteResidence,
                ArrayReleaseRoleV1::Home(relations[0].binding()),
            ]
        );
        let terminal = recipe.take_return(&terminal_site).unwrap().unwrap();
        assert_eq!(terminal.site().node(), &terminal_site);
        assert!(matches!(
            terminal.result(),
            root_terminal::RootResult::Integer { value: 30, .. }
        ));
        assert_eq!(
            terminal.releases(),
            &[
                ArrayReleaseRoleV1::Home(relations[1].binding()),
                ArrayReleaseRoleV1::Home(relations[0].binding()),
            ]
        );
        recipe.finish().unwrap();
        assert!(recipe
            .take_return(&terminal_site)
            .unwrap_err()
            .contains("already-taken"));
        // Issuing/taking a Recipe does not mark the source emission obligations complete.
        assert!(rows.finish_root().is_err());
    }
}

#[test]
fn recipe_rejects_partial_cutpoints_and_source_drift() {
    for mutation in 0..4 {
        let (product, window) = source("local a: Array<i64> = [10]\nreturn", 0);
        let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        let ArraySourceCoverage::Available(row) = rows.rows.values_mut().next().unwrap() else {
            panic!("available")
        };
        let mut cuts = std::mem::replace(&mut row.cutpoints, Box::new([])).into_vec();
        match mutation {
            0 => {
                cuts.remove(4);
            }
            1 => cuts.swap(3, 4),
            2 => {
                cuts.pop();
            }
            3 => {
                row.progress = LocalProgress::InFlight;
            }
            _ => unreachable!(),
        }
        row.cutpoints = cuts.into_boxed_slice();
        assert!(rows.lowering_recipe().is_err());
        let relation = product.expression_source().initializers().next().unwrap();
        let clean = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        let mut recipe = clean.lowering_recipe().unwrap();
        let (foreign, _) = source("local a: Array<i64> = [10]\nreturn", 1);
        let foreign = foreign.expression_source().initializers().next().unwrap();
        assert!(recipe.take_local(foreign).is_err());
        recipe.take_local(relation).unwrap().unwrap();
        let terminal = recipe
            .take_return(clean.terminal().unwrap().unwrap().site().node())
            .unwrap()
            .unwrap();
        assert!(matches!(terminal.result(), root_terminal::RootResult::Unit));
        recipe.finish().unwrap();
    }
}

#[test]
fn missing_completion_and_opaque_child_never_receive_a_default_recipe() {
    for text in [
        "local a: Array<i64> = [10]",
        "local a: Array<i64> = [[10]]\nreturn",
    ] {
        let (product, window) = source(text, 0);
        let rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        assert!(rows.lowering_recipe().is_err(), "{text}");
    }
    let (product, window) = source("local a = 10\nreturn 30", 0);
    let rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
    let recipe = rows.lowering_recipe().unwrap();
    recipe.finish().unwrap();
}
