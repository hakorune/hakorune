use super::*;

#[test]
fn relation_issuer_brands_root_and_destination_together() {
    let mut issuer = HomeRelationBrandIssuerV1::issue().expect("brand should issue");
    let root = issuer.root(7).expect("root should issue once");
    let destination = issuer
        .destination(11)
        .expect("destination should issue once");

    assert_eq!(root.brand(), issuer.brand());
    assert_eq!(destination.brand(), issuer.brand());
    assert_eq!(root.source_ordinal(), 7);
    assert_eq!(destination.source_ordinal(), 11);
    issuer
        .require_same_brand(root, destination)
        .expect("same issuer relations should match");
}

#[test]
fn relation_issuer_rejects_duplicate_source_slots() {
    let mut issuer = HomeRelationBrandIssuerV1::issue().expect("brand should issue");
    issuer.root(3).expect("first root should issue");
    assert!(matches!(
        issuer.root(3),
        Err(HomeRelationRejectV1::DuplicateRootSource { source_ordinal: 3 })
    ));

    issuer
        .destination(5)
        .expect("first destination should issue");
    assert!(matches!(
        issuer.destination(5),
        Err(HomeRelationRejectV1::DuplicateDestinationSource { source_ordinal: 5 })
    ));
}

#[test]
fn relation_issuer_rejects_foreign_brand() {
    let mut first = HomeRelationBrandIssuerV1::issue().expect("first brand should issue");
    let mut second = HomeRelationBrandIssuerV1::issue().expect("second brand should issue");
    let root = first.root(1).expect("first root should issue");
    let destination = second
        .destination(2)
        .expect("second destination should issue");

    assert!(matches!(
        first.require_same_brand(root, destination),
        Err(HomeRelationRejectV1::ForeignBrand)
    ));
}

#[test]
fn capability_vocabulary_has_no_implicit_unknown_case() {
    assert_eq!(HomeDemandV1::Handle, HomeDemandV1::Handle);
    assert_eq!(HomeDemandV1::Trivial, HomeDemandV1::Trivial);
    assert_eq!(HomeResultRelationV1::Unit, HomeResultRelationV1::Unit);
    assert_eq!(
        HomeResultRelationV1::FromParameter(2),
        HomeResultRelationV1::FromParameter(2)
    );
}
