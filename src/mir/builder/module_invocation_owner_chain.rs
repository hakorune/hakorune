//! CUT0-I0-ID0-P0: disconnected invocation-brand propagation.
//!
//! The non-Clone source token owns the invocation ID exactly once. Later
//! owners carry only its opaque copyable brand, so foreign pairing fails
//! before any owner mutation. This vocabulary is disconnected from every
//! production ingress; COLLECT0-S0 will connect the existing owners.

use super::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct ModuleBuilderInvocationSessionV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    _seal: ModuleBuilderInvocationSessionSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModuleBuilderInvocationSessionSealV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationDraftSourceProofV1 {
    Raw { token: ModuleInvocationTokenV1 },
    CanonicalSingle { token: ModuleInvocationTokenV1 },
    CallableBatch { token: ModuleInvocationTokenV1 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationReceiptKindV1 {
    Raw,
    CanonicalSingle,
    CallableBatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct InvocationBrandedReceiptV1 {
    brand: ModuleInvocationBrandV1,
    kind: InvocationReceiptKindV1,
    _seal: InvocationBrandedReceiptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InvocationBrandedReceiptSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationBrandErrorV1 {
    ForeignOwner { expected: u64, actual: u64 },
    ReceiptKindMismatch {
        family: ModuleInvocationFamilyV1,
        kind: InvocationReceiptKindV1,
    },
}

impl std::fmt::Display for InvocationBrandErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][invocation_brand] {self:?}")
    }
}

impl std::error::Error for InvocationBrandErrorV1 {}

#[derive(Debug)]
pub(in crate::mir::builder) struct InvocationBranded<T> {
    brand: ModuleInvocationBrandV1,
    payload: T,
    _seal: InvocationBrandedOwnerSealV1,
}

#[derive(Debug)]
struct InvocationBrandedOwnerSealV1;

pub(in crate::mir::builder) type BrandedShellV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedCollectorV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedLedgerV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedCompleteV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedDrainedV1<T> = InvocationBranded<T>;
pub(in crate::mir::builder) type BrandedFinalizedV1<T> = InvocationBranded<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct PreparedModuleExternalCommitV1 {
    brand: ModuleInvocationBrandV1,
    _seal: PreparedModuleExternalCommitSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PreparedModuleExternalCommitSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CollectedInvocationDraftSetV1 {
    source: InvocationDraftSourceProofV1,
    shell: BrandedShellV1<()>,
    collector: BrandedCollectorV1<()>,
    receipts: Box<[InvocationBrandedReceiptV1]>,
    _seal: CollectedInvocationDraftSetSealV1,
}

#[derive(Debug)]
struct CollectedInvocationDraftSetSealV1;

impl InvocationDraftSourceProofV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_token(token: ModuleInvocationTokenV1) -> Self {
        match token.family() {
            ModuleInvocationFamilyV1::Raw => Self::Raw { token },
            ModuleInvocationFamilyV1::CanonicalAPlus
            | ModuleInvocationFamilyV1::BindingSsaTrivial => Self::CanonicalSingle { token },
            ModuleInvocationFamilyV1::BindingSsaAcyclic
            | ModuleInvocationFamilyV1::BindingSsaRecursive => Self::CallableBatch { token },
        }
    }

    pub(in crate::mir::builder) fn family(&self) -> ModuleInvocationFamilyV1 {
        match self {
            Self::Raw { token }
            | Self::CanonicalSingle { token }
            | Self::CallableBatch { token } => token.family(),
        }
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Raw { token }
            | Self::CanonicalSingle { token }
            | Self::CallableBatch { token } => token.brand(),
        }
    }
}

impl ModuleBuilderInvocationSessionV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_source(source: &InvocationDraftSourceProofV1) -> Self {
        Self {
            brand: source.brand(),
            family: source.family(),
            _seal: ModuleBuilderInvocationSessionSealV1,
        }
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn family(&self) -> ModuleInvocationFamilyV1 {
        self.family
    }
}

impl InvocationBrandedReceiptV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(
        brand: ModuleInvocationBrandV1,
        kind: InvocationReceiptKindV1,
    ) -> Self {
        Self {
            brand,
            kind,
            _seal: InvocationBrandedReceiptSealV1,
        }
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) const fn kind(&self) -> InvocationReceiptKindV1 {
        self.kind
    }
}

impl<T> InvocationBranded<T> {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(
        brand: ModuleInvocationBrandV1,
        payload: T,
    ) -> Self {
        Self {
            brand,
            payload,
            _seal: InvocationBrandedOwnerSealV1,
        }
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn payload(&self) -> &T {
        &self.payload
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn into_payload(self) -> T {
        self.payload
    }
}

impl PreparedModuleExternalCommitV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_finalized<T>(finalized: BrandedFinalizedV1<T>) -> Self {
        Self {
            brand: finalized.brand(),
            _seal: PreparedModuleExternalCommitSealV1,
        }
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }
}

impl CollectedInvocationDraftSetV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_parts(
        source: InvocationDraftSourceProofV1,
        shell: BrandedShellV1<()>,
        collector: BrandedCollectorV1<()>,
        receipts: Vec<InvocationBrandedReceiptV1>,
    ) -> Result<Self, InvocationBrandErrorV1> {
        let brand = source.brand();
        check_brand(brand, shell.brand())?;
        check_brand(brand, collector.brand())?;
        for receipt in &receipts {
            check_brand(brand, receipt.brand())?;
            if !receipt_kind_matches(source.family(), receipt.kind()) {
                return Err(InvocationBrandErrorV1::ReceiptKindMismatch {
                    family: source.family(),
                    kind: receipt.kind(),
                });
            }
        }
        Ok(Self {
            source,
            shell,
            collector,
            receipts: receipts.into_boxed_slice(),
            _seal: CollectedInvocationDraftSetSealV1,
        })
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn complete(self) -> BrandedCompleteV1<Self> {
        let brand = self.brand();
        InvocationBranded::from_test(brand, self)
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.source.brand()
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn receipt_count(&self) -> usize {
        self.receipts.len()
    }
}

#[cfg(test)]
pub(in crate::mir::builder) fn advance_to_prepared_commit(
    source: InvocationDraftSourceProofV1,
    shell: BrandedShellV1<()>,
    collector: BrandedCollectorV1<()>,
    receipts: Vec<InvocationBrandedReceiptV1>,
) -> Result<PreparedModuleExternalCommitV1, InvocationBrandErrorV1> {
    let collected = CollectedInvocationDraftSetV1::from_parts(source, shell, collector, receipts)?;
    let complete = collected.complete();
    let drained = InvocationBranded::from_test(complete.brand(), complete);
    let finalized = InvocationBranded::from_test(drained.brand(), drained);
    Ok(PreparedModuleExternalCommitV1::from_finalized(finalized))
}

fn check_brand(
    expected: ModuleInvocationBrandV1,
    actual: ModuleInvocationBrandV1,
) -> Result<(), InvocationBrandErrorV1> {
    if expected.same(actual) {
        Ok(())
    } else {
        Err(InvocationBrandErrorV1::ForeignOwner {
            expected: expected.ordinal(),
            actual: actual.ordinal(),
        })
    }
}

fn receipt_kind_matches(
    family: ModuleInvocationFamilyV1,
    kind: InvocationReceiptKindV1,
) -> bool {
    match kind {
        InvocationReceiptKindV1::Raw => family == ModuleInvocationFamilyV1::Raw,
        InvocationReceiptKindV1::CanonicalSingle => matches!(
            family,
            ModuleInvocationFamilyV1::CanonicalAPlus
                | ModuleInvocationFamilyV1::BindingSsaTrivial
        ),
        InvocationReceiptKindV1::CallableBatch => matches!(
            family,
            ModuleInvocationFamilyV1::BindingSsaAcyclic
                | ModuleInvocationFamilyV1::BindingSsaRecursive
        ),
    }
}
