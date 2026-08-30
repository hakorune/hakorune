#[test]
fn source_lease_rejects_duplicate_unknown_and_mismatched_forest_roles() {
    let unit = unit(SOURCE);
    let (input, root) = input_and_root(&unit);
    let (root_site, loop_site, write, read) = sites(input, &root);
    let function = input.function();
    let duplicate = issue_generic_source_lease_v1(
        function,
        function.owner(),
        function.function_origin(),
        function.source_kind(),
        root_site.clone(),
        loop_site.clone(),
        [
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::NestedWrite,
                OwnedExprSiteV1::new(function.owner(), write.clone()),
            ),
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::NestedWrite,
                OwnedExprSiteV1::new(function.owner(), write.clone()),
            ),
        ],
    );
    assert_eq!(duplicate, Err(GenericSourceLeaseRejectV1::DuplicateRole));

    let unknown = issue_generic_source_lease_v1(
        function,
        function.owner(),
        function.function_origin(),
        function.source_kind(),
        root_site.clone(),
        loop_site.clone(),
        [
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::NestedWrite,
                OwnedExprSiteV1::new(function.owner(), write.clone()),
            ),
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::Unknown,
                OwnedExprSiteV1::new(function.owner(), read.clone()),
            ),
        ],
    );
    assert_eq!(unknown, Err(GenericSourceLeaseRejectV1::UnsupportedRole));

    let wrong_loop = SourceStmtSiteV1::from_node(super::super::SourcePathV1::root_body(1).node());
    let mismatch = issue_generic_source_lease_v1(
        function,
        function.owner(),
        function.function_origin(),
        function.source_kind(),
        root_site,
        wrong_loop,
        [
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::NestedWrite,
                OwnedExprSiteV1::new(function.owner(), write),
            ),
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::PostLoopRead,
                OwnedExprSiteV1::new(function.owner(), read),
            ),
        ],
    );
    assert_eq!(mismatch, Err(GenericSourceLeaseRejectV1::ForestMismatch));
}

#[test]
fn source_lease_rejects_identity_and_role_input_mismatches() {
    let unit = unit(SOURCE);
    let (input, root) = input_and_root(&unit);
    let (root_site, loop_site, write, read) = sites(input, &root);
    let function = input.function();
    let roles = || {
        [
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::NestedWrite,
                OwnedExprSiteV1::new(function.owner(), write.clone()),
            ),
            GenericSourceRoleSiteV1::new(
                GenericSourceRoleKindV1::PostLoopRead,
                OwnedExprSiteV1::new(function.owner(), read.clone()),
            ),
        ]
    };
    assert_eq!(
        issue_generic_source_lease_v1(
            function,
            function.owner(),
            FunctionOriginV1::new(99, 99),
            function.source_kind(),
            root_site.clone(),
            loop_site.clone(),
            roles(),
        ),
        Err(GenericSourceLeaseRejectV1::ForeignOrigin)
    );
    assert_eq!(
        issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            SemanticOwnerSourceKindV1::Script,
            root_site.clone(),
            loop_site.clone(),
            roles(),
        ),
        Err(GenericSourceLeaseRejectV1::SourceKindMismatch)
    );
    assert_eq!(
        issue_generic_source_lease_v1(
            function,
            function.owner(),
            function.function_origin(),
            function.source_kind(),
            root_site,
            loop_site,
            [
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), write),
                ),
                GenericSourceRoleSiteV1::new(
                    GenericSourceRoleKindV1::PostLoopRead,
                    OwnedExprSiteV1::new(function.owner(), read),
                ),
            ],
        ),
        Err(GenericSourceLeaseRejectV1::RolePlacementMismatch)
    );
}
