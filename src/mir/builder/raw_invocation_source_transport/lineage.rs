use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationRootLineageV1 {
    ScriptRoot,
    Main(RawSourceLocatorV1),
    Cataloged(CanonicalSameModuleCallableKeyV1),
    TopLevel(SelectedTopLevelFunctionKeyV1),
    InstanceConstructor(NormalInstanceConstructorSourceKeyV1),
    NestedBoxMethod {
        parent_site: SourceNodeSiteV1,
        method_key: Box<str>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawUnlocatedPortalV1 {
    CallObject,
}

impl RawInvocationRootLineageV1 {
    pub(super) fn allows_bare_function_call_location(&self) -> bool {
        matches!(
            self,
            Self::Cataloged(_) | Self::TopLevel(_) | Self::InstanceConstructor(_)
        )
    }

    pub(in crate::mir::builder) fn nested_box_method(
        parent_site: SourceNodeSiteV1,
        method_key: String,
    ) -> Self {
        Self::NestedBoxMethod {
            parent_site,
            method_key: method_key.into_boxed_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LocatedRawNodeV1<T> {
    node: T,
    root: RawInvocationRootLineageV1,
    site: SourceNodeSiteV1,
    body_kind: SourceBodyKindV1,
}

impl<T> LocatedRawNodeV1<T> {
    pub(super) fn new(
        node: T,
        root: RawInvocationRootLineageV1,
        site: SourceNodeSiteV1,
        body_kind: SourceBodyKindV1,
    ) -> Self {
        Self {
            node,
            root,
            site,
            body_kind,
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        T,
        RawInvocationRootLineageV1,
        SourceNodeSiteV1,
        SourceBodyKindV1,
    ) {
        (self.node, self.root, self.site, self.body_kind)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawInvocationSourceTransportV1<T> {
    Located(LocatedRawNodeV1<T>),
    UnlocatedCompatibility {
        node: T,
        reason: RawUnlocatedPortalV1,
        /// Preserve a source-backed root when exact node location is lost.
        /// The later ingress uses this witness to reject source loss instead
        /// of silently treating it as ordinary compatibility.
        expected_lineage: Option<RawInvocationRootLineageV1>,
    },
}

impl<T> RawInvocationSourceTransportV1<T> {
    pub(in crate::mir::builder) fn root(node: T, root: RawInvocationRootLineageV1) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            root,
            SourcePathV1::function_body().node(),
            SourceBodyKindV1::Function,
        ))
    }

    pub(in crate::mir::builder) fn script_root(node: T) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            RawInvocationRootLineageV1::ScriptRoot,
            SourcePathV1::program_body().node(),
            SourceBodyKindV1::Program,
        ))
    }

    pub(in crate::mir::builder) fn script_semantic_root(node: T) -> Self {
        Self::Located(LocatedRawNodeV1::new(
            node,
            RawInvocationRootLineageV1::ScriptRoot,
            SourcePathV1::program_body().node(),
            SourceBodyKindV1::Program,
        ))
    }

    pub(in crate::mir::builder) fn unlocated(node: T, reason: RawUnlocatedPortalV1) -> Self {
        Self::UnlocatedCompatibility {
            node,
            reason,
            expected_lineage: None,
        }
    }

    pub(super) fn unlocated_with_expected_lineage(
        node: T,
        reason: RawUnlocatedPortalV1,
        expected_lineage: RawInvocationRootLineageV1,
    ) -> Self {
        Self::UnlocatedCompatibility {
            node,
            reason,
            expected_lineage: Some(expected_lineage),
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        T,
        Option<(
            RawInvocationRootLineageV1,
            SourceNodeSiteV1,
            SourceBodyKindV1,
        )>,
        Option<(RawUnlocatedPortalV1, Option<RawInvocationRootLineageV1>)>,
    ) {
        match self {
            Self::Located(located) => {
                let (node, root, site, body_kind) = located.into_parts();
                (node, Some((root, site, body_kind)), None)
            }
            Self::UnlocatedCompatibility {
                node,
                reason,
                expected_lineage,
            } => (node, None, Some((reason, expected_lineage))),
        }
    }
}
