use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, HomeDemandV1, VerifiedSemanticOwnerForestV1,
};
use crate::parser::ParserCallableParameterSourceCatalogV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableParameterDeclarationModeV1 {
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableParameterDemandV1 {
    ordinal: u32,
    binding: BindingRefV1,
    demand: HomeDemandV1,
}

impl VerifiedCallableParameterDemandV1 {
    pub(super) const fn new(ordinal: u32, binding: BindingRefV1, demand: HomeDemandV1) -> Self {
        Self {
            ordinal,
            binding,
            demand,
        }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn demand(&self) -> HomeDemandV1 {
        self.demand
    }
}

#[derive(Debug)]
pub(super) struct VerifiedCallableParameterDemandDeclarationV1 {
    source_row_index: u32,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    mode: CallableParameterDeclarationModeV1,
    parameters: Box<[VerifiedCallableParameterDemandV1]>,
}

impl VerifiedCallableParameterDemandDeclarationV1 {
    pub(super) const fn new(
        source_row_index: u32,
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        mode: CallableParameterDeclarationModeV1,
        parameters: Box<[VerifiedCallableParameterDemandV1]>,
    ) -> Self {
        Self {
            source_row_index,
            owner,
            function_origin,
            mode,
            parameters,
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableParameterDemandCatalogV1 {
    _source: ParserCallableParameterSourceCatalogV1,
    forests: Box<[VerifiedSemanticOwnerForestV1]>,
    declarations: Box<[VerifiedCallableParameterDemandDeclarationV1]>,
}

impl VerifiedCallableParameterDemandCatalogV1 {
    pub(super) const fn new(
        source: ParserCallableParameterSourceCatalogV1,
        forests: Box<[VerifiedSemanticOwnerForestV1]>,
        declarations: Box<[VerifiedCallableParameterDemandDeclarationV1]>,
    ) -> Self {
        Self {
            _source: source,
            forests,
            declarations,
        }
    }

    pub(crate) fn declarations(
        &self,
    ) -> impl ExactSizeIterator<Item = VerifiedCallableParameterDemandDeclarationRefV1<'_>> {
        self.declarations
            .iter()
            .zip(self.forests.iter())
            .map(
                |(declaration, forest)| VerifiedCallableParameterDemandDeclarationRefV1 {
                    declaration,
                    forest,
                },
            )
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedCallableParameterDemandDeclarationRefV1<'a> {
    declaration: &'a VerifiedCallableParameterDemandDeclarationV1,
    forest: &'a VerifiedSemanticOwnerForestV1,
}

impl<'a> VerifiedCallableParameterDemandDeclarationRefV1<'a> {
    pub(crate) const fn source_row_index(self) -> u32 {
        self.declaration.source_row_index
    }

    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.declaration.owner
    }

    pub(crate) const fn function_origin(self) -> FunctionOriginV1 {
        self.declaration.function_origin
    }

    pub(crate) const fn mode(self) -> CallableParameterDeclarationModeV1 {
        self.declaration.mode
    }

    pub(crate) fn parameters(self) -> &'a [VerifiedCallableParameterDemandV1] {
        &self.declaration.parameters
    }

    pub(crate) const fn resolved_forest(self) -> &'a VerifiedSemanticOwnerForestV1 {
        self.forest
    }
}
