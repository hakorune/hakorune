use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, HomeDemandV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableParameterContractKindV1 {
    OpaqueHandle,
    ExactTrivial(ExactTrivialParameterAbiV1),
    ExactText(ExactTextFormalAbiV1),
}

impl CallableParameterContractKindV1 {
    pub(crate) const fn home_demand(self) -> HomeDemandV1 {
        match self {
            Self::OpaqueHandle => HomeDemandV1::Handle,
            Self::ExactTrivial(_) => HomeDemandV1::Trivial,
            Self::ExactText(_) => HomeDemandV1::Handle,
        }
    }

    pub(crate) const fn exact_trivial_abi(self) -> Option<ExactTrivialParameterAbiV1> {
        match self {
            Self::OpaqueHandle => None,
            Self::ExactTrivial(abi) => Some(abi),
            Self::ExactText(_) => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableParameterContractV1 {
    ordinal: u32,
    binding: BindingRefV1,
    kind: CallableParameterContractKindV1,
}

impl VerifiedCallableParameterContractV1 {
    pub(super) const fn new(
        ordinal: u32,
        binding: BindingRefV1,
        kind: CallableParameterContractKindV1,
    ) -> Self {
        Self {
            ordinal,
            binding,
            kind,
        }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn kind(&self) -> CallableParameterContractKindV1 {
        self.kind
    }

    pub(crate) const fn home_demand(&self) -> HomeDemandV1 {
        self.kind.home_demand()
    }
}

#[derive(Debug)]
pub(super) struct VerifiedCallableParameterContractDeclarationV1 {
    batch_slot: u32,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    mode: CallableParameterDeclarationModeV1,
    parameters: Box<[VerifiedCallableParameterContractV1]>,
}

impl VerifiedCallableParameterContractDeclarationV1 {
    pub(super) const fn new(
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        mode: CallableParameterDeclarationModeV1,
        parameters: Box<[VerifiedCallableParameterContractV1]>,
    ) -> Self {
        Self {
            batch_slot,
            owner,
            function_origin,
            mode,
            parameters,
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallableParameterContractCatalogV1<'batch> {
    _batch: &'batch VerifiedResolvedCallableSemanticBatchV1,
    declarations: Box<[VerifiedCallableParameterContractDeclarationV1]>,
}

impl<'batch> VerifiedCallableParameterContractCatalogV1<'batch> {
    pub(super) const fn new(
        batch: &'batch VerifiedResolvedCallableSemanticBatchV1,
        declarations: Box<[VerifiedCallableParameterContractDeclarationV1]>,
    ) -> Self {
        Self {
            _batch: batch,
            declarations,
        }
    }

    pub(crate) fn declarations(
        &self,
    ) -> impl ExactSizeIterator<Item = VerifiedCallableParameterContractDeclarationRefV1<'_>> {
        self.declarations
            .iter()
            .map(|declaration| VerifiedCallableParameterContractDeclarationRefV1 { declaration })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedCallableParameterContractDeclarationRefV1<'a> {
    declaration: &'a VerifiedCallableParameterContractDeclarationV1,
}

impl<'a> VerifiedCallableParameterContractDeclarationRefV1<'a> {
    pub(crate) const fn batch_slot(self) -> u32 {
        self.declaration.batch_slot
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

    pub(crate) fn parameters(self) -> &'a [VerifiedCallableParameterContractV1] {
        &self.declaration.parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableParameterDeclarationModeV1 {
    StaticBoxMethod,
    InstanceBoxMethod,
}
