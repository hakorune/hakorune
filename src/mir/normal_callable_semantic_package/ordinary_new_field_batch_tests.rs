//! Cached-site drift and duplicate rejection must not commit an earlier row.
use super::*;
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, ResolvedLexicalRefV1, SourcePathSegmentV1, SourcePathV1,
};

fn rows() -> Vec<(OwnedExprSiteV1, field_reads::LocalFieldRead)> {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Pool { first: i64 = 0 second: i64 = 0 birth() { } }
         static box Main { main() { local pool = new Pool()
            local a = pool.first local b = pool.second return a } }",
    ).expect("exact field read fixture");
    let mut rows = Vec::new();
    for declaration in package.batch().declarations() {
        package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                for shape in input.body_shape().expect("body shape").expressions() {
                    let BodyExpressionShapeV1::FieldAccess { site, object, .. } = shape else {
                        continue;
                    };
                    let Some(ResolvedLexicalRefV1::Local(receiver)) =
                        input.function().variable_ref(object)
                    else {
                        panic!("receiver");
                    };
                    let owned = OwnedExprSiteV1::new(declaration.owner(), site.clone());
                    let (field, result) = package
                        .ordinary_new_claim_ledger
                        .staged_local_field_read(&owned)
                        .expect("staged read");
                    rows.push((
                        owned,
                        field_reads::LocalFieldRead {
                            receiver_site: object.clone(),
                            receiver,
                            home: receiver,
                            field,
                            result,
                            progress: field_reads::Progress::Pending,
                        },
                    ));
                }
            })
            .expect("source loan");
    }
    assert_eq!(rows.len(), 2);
    rows
}

#[test]
fn scalar_expression_batch_cached_drift_rejects_before_new_row_commit() {
    let mut rows = rows();
    let (site, old) = rows.pop().unwrap();
    let mut drift = field_reads::LocalFieldRead {
        receiver_site: SourcePathV1::from_node(old.receiver_site.node())
            .child(SourcePathSegmentV1::Lhs)
            .expr(),
        receiver: old.receiver,
        home: old.home,
        field: old.field,
        result: old.result.clone(),
        progress: field_reads::Progress::Pending,
    };
    let original_site = old.receiver_site.clone();
    let mut staged = BTreeMap::from([(site.clone(), old)]);
    let fresh = rows[0].0.clone();
    rows.push((site.clone(), drift));
    assert!(matches!(
        stage_local_field_read_batch(&mut staged, rows),
        Err(OrdinaryNewCoSealIssueV1::DuplicateSite { .. })
    ));
    assert_eq!(staged.len(), 1);
    assert!(!staged.contains_key(&fresh));
    assert_eq!(staged[&site].receiver_site, original_site);
    // Equal descriptors preserve the old row (including its affine progress).
    drift = field_reads::LocalFieldRead {
        receiver_site: original_site,
        receiver: staged[&site].receiver,
        home: staged[&site].home,
        field: staged[&site].field,
        result: staged[&site].result.clone(),
        progress: field_reads::Progress::Pending,
    };
    assert_eq!(
        stage_local_field_read_batch(&mut staged, vec![(site, drift)])
            .unwrap()
            .len(),
        1
    );
    assert_eq!(staged.len(), 1);
}

#[test]
fn scalar_expression_batch_duplicate_rejects_without_commit() {
    let mut rows = rows();
    let (site, row) = rows.pop().unwrap();
    let duplicate = field_reads::LocalFieldRead {
        receiver_site: row.receiver_site.clone(),
        receiver: row.receiver,
        home: row.home,
        field: row.field,
        result: row.result.clone(),
        progress: field_reads::Progress::Pending,
    };
    rows.push((site.clone(), row));
    rows.push((site, duplicate));
    let mut staged = BTreeMap::new();
    assert!(matches!(
        stage_local_field_read_batch(&mut staged, rows),
        Err(OrdinaryNewCoSealIssueV1::DuplicateSite { .. })
    ));
    assert!(staged.is_empty());
}
