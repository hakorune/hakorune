//! REMATFACT0-S0 private vocabulary for exact PHI-rematerialization receipts.
//!
//! This module has no production issuer, candidate executor, fact-map write,
//! or module-repair consumer. Its only job is to make the later producer
//! receipt, candidate reservation, and one-shot projection boundaries explicit.

use crate::mir::builder::fact_session::FunctionFactGenerationV1;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeSet;

/// Physical producer families P0 can eventually rematerialize.
///
/// Listing a family does not admit it. M0/G0 must prove that every selected
/// physical producer for the family issues a matching receipt before a
/// candidate executor may consume it.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::mir::builder) enum ExactProducerFamilyV1 {
    Const,
    Compare,
    Copy,
    BinOp,
    UnaryOp,
    Select,
    SubstringCall,
}

/// Opaque definition identity for one successful physical producer.
///
/// `recipe_fingerprint` is deliberately not computed from a later MIR scan.
/// The future issuer receives it from its physical-producer transaction.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::mir::builder) struct ProducerDefinitionIdentityV1 {
    generation: FunctionFactGenerationV1,
    value: ValueId,
    family: ExactProducerFamilyV1,
    recipe_fingerprint: u64,
}

impl ProducerDefinitionIdentityV1 {
    pub(in crate::mir::builder) const fn generation(self) -> FunctionFactGenerationV1 {
        self.generation
    }

    pub(in crate::mir::builder) const fn value(self) -> ValueId {
        self.value
    }

    pub(in crate::mir::builder) const fn family(self) -> ExactProducerFamilyV1 {
        self.family
    }
}

/// One exact result fact retained from a successful physical producer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct ExactProducerTypeReceiptV1 {
    source_definition: ProducerDefinitionIdentityV1,
    exact_type: MirType,
}

impl ExactProducerTypeReceiptV1 {
    pub(in crate::mir::builder) const fn source_definition(&self) -> ProducerDefinitionIdentityV1 {
        self.source_definition
    }

    pub(in crate::mir::builder) fn exact_type(&self) -> &MirType {
        &self.exact_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum ExactProducerReceiptErrorV1 {
    UnknownIsNotExact,
    ForeignGeneration {
        ledger: FunctionFactGenerationV1,
        definition: FunctionFactGenerationV1,
    },
    DuplicateDefinition(ProducerDefinitionIdentityV1),
}

impl std::fmt::Display for ExactProducerReceiptErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_remat/producer_receipt] {self:?}"
        )
    }
}

impl std::error::Error for ExactProducerReceiptErrorV1 {}

/// Open only while the associated function session is lowering.
#[derive(Debug)]
pub(in crate::mir::builder) struct OpenExactProducerReceiptLedgerV1 {
    generation: FunctionFactGenerationV1,
    rows: Vec<ExactProducerTypeReceiptV1>,
}

impl OpenExactProducerReceiptLedgerV1 {
    pub(in crate::mir::builder) fn new(generation: FunctionFactGenerationV1) -> Self {
        Self {
            generation,
            rows: Vec::new(),
        }
    }

    /// Records one exact result only after its physical producer has succeeded.
    pub(in crate::mir::builder) fn record_success(
        &mut self,
        definition: ProducerDefinitionIdentityV1,
        exact_type: MirType,
    ) -> Result<(), ExactProducerReceiptErrorV1> {
        if exact_type == MirType::Unknown {
            return Err(ExactProducerReceiptErrorV1::UnknownIsNotExact);
        }
        if definition.generation != self.generation {
            return Err(ExactProducerReceiptErrorV1::ForeignGeneration {
                ledger: self.generation,
                definition: definition.generation,
            });
        }
        if self
            .rows
            .iter()
            .any(|row| row.source_definition == definition)
        {
            return Err(ExactProducerReceiptErrorV1::DuplicateDefinition(definition));
        }
        self.rows.push(ExactProducerTypeReceiptV1 {
            source_definition: definition,
            exact_type,
        });
        Ok(())
    }

    pub(in crate::mir::builder) fn seal(self) -> SealedExactProducerReceiptLedgerV1 {
        SealedExactProducerReceiptLedgerV1 {
            generation: self.generation,
            rows: self.rows.into_boxed_slice(),
            _seal: SealedExactProducerReceiptLedgerSealV1,
        }
    }
}

/// Immutable receipt source for a later candidate preflight.
#[derive(Debug)]
pub(in crate::mir::builder) struct SealedExactProducerReceiptLedgerV1 {
    generation: FunctionFactGenerationV1,
    rows: Box<[ExactProducerTypeReceiptV1]>,
    _seal: SealedExactProducerReceiptLedgerSealV1,
}

#[derive(Debug)]
struct SealedExactProducerReceiptLedgerSealV1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum ExactProducerReceiptLookupErrorV1 {
    ForeignGeneration {
        ledger: FunctionFactGenerationV1,
        definition: FunctionFactGenerationV1,
    },
    MissingProducerReceipt(ProducerDefinitionIdentityV1),
}

impl std::fmt::Display for ExactProducerReceiptLookupErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_remat/producer_receipt_lookup] {self:?}"
        )
    }
}

impl std::error::Error for ExactProducerReceiptLookupErrorV1 {}

impl SealedExactProducerReceiptLedgerV1 {
    pub(in crate::mir::builder) fn lookup(
        &self,
        definition: ProducerDefinitionIdentityV1,
    ) -> Result<&ExactProducerTypeReceiptV1, ExactProducerReceiptLookupErrorV1> {
        if definition.generation != self.generation {
            return Err(ExactProducerReceiptLookupErrorV1::ForeignGeneration {
                ledger: self.generation,
                definition: definition.generation,
            });
        }
        self.rows
            .iter()
            .find(|row| row.source_definition == definition)
            .ok_or(ExactProducerReceiptLookupErrorV1::MissingProducerReceipt(
                definition,
            ))
    }
}

/// Candidate-local session identity. It cannot pair receipts from another
/// function-generation and tracks reservations without owning a type map.
#[derive(Debug)]
pub(in crate::mir::builder) struct CandidateFunctionFactSessionV1 {
    source_generation: FunctionFactGenerationV1,
    reserved_destinations: BTreeSet<ValueId>,
}

impl CandidateFunctionFactSessionV1 {
    pub(in crate::mir::builder) fn new(source_generation: FunctionFactGenerationV1) -> Self {
        Self {
            source_generation,
            reserved_destinations: BTreeSet::new(),
        }
    }

    pub(in crate::mir::builder) fn reserve_fresh_destination(
        &mut self,
        destination: ValueId,
    ) -> Result<CandidateFreshFactReservationV1, CandidateFactReservationErrorV1> {
        if !self.reserved_destinations.insert(destination) {
            return Err(CandidateFactReservationErrorV1::DuplicateDestination(
                destination,
            ));
        }
        Ok(CandidateFreshFactReservationV1 {
            source_generation: self.source_generation,
            destination,
            _seal: CandidateFreshFactReservationSealV1,
        })
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CandidateFreshFactReservationV1 {
    source_generation: FunctionFactGenerationV1,
    destination: ValueId,
    _seal: CandidateFreshFactReservationSealV1,
}

#[derive(Debug)]
struct CandidateFreshFactReservationSealV1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum CandidateFactReservationErrorV1 {
    DuplicateDestination(ValueId),
}

impl std::fmt::Display for CandidateFactReservationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_remat/fact_reservation] {self:?}"
        )
    }
}

impl std::error::Error for CandidateFactReservationErrorV1 {}

/// Verified source node from the future PHI-rematerialization candidate plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct VerifiedPhiRematerializationNodeV1 {
    predecessor: BasicBlockId,
    source_definition: ProducerDefinitionIdentityV1,
}

impl VerifiedPhiRematerializationNodeV1 {
    pub(in crate::mir::builder) const fn predecessor(&self) -> BasicBlockId {
        self.predecessor
    }
}

/// Move-only bridge from a sealed source receipt to one later candidate commit.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedPhiRematExactTypeProjectionV1 {
    source_receipt: ExactProducerTypeReceiptV1,
    node: VerifiedPhiRematerializationNodeV1,
    reservation: CandidateFreshFactReservationV1,
    _seal: PhiRematExactTypeProjectionSealV1,
}

#[derive(Debug)]
struct PhiRematExactTypeProjectionSealV1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum PhiRematExactTypeProjectionErrorV1 {
    Receipt(ExactProducerReceiptLookupErrorV1),
    ForeignCandidateSession {
        receipt: FunctionFactGenerationV1,
        candidate: FunctionFactGenerationV1,
    },
}

impl std::fmt::Display for PhiRematExactTypeProjectionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_remat/exact_type_projection] {self:?}"
        )
    }
}

impl std::error::Error for PhiRematExactTypeProjectionErrorV1 {}

impl PreparedPhiRematExactTypeProjectionV1 {
    pub(in crate::mir::builder) fn prepare(
        ledger: &SealedExactProducerReceiptLedgerV1,
        node: VerifiedPhiRematerializationNodeV1,
        reservation: CandidateFreshFactReservationV1,
    ) -> Result<Self, PhiRematExactTypeProjectionErrorV1> {
        let source_receipt = ledger
            .lookup(node.source_definition)
            .map_err(PhiRematExactTypeProjectionErrorV1::Receipt)?
            .clone();
        if source_receipt.source_definition.generation != reservation.source_generation {
            return Err(
                PhiRematExactTypeProjectionErrorV1::ForeignCandidateSession {
                    receipt: source_receipt.source_definition.generation,
                    candidate: reservation.source_generation,
                },
            );
        }
        Ok(Self {
            source_receipt,
            node,
            reservation,
            _seal: PhiRematExactTypeProjectionSealV1,
        })
    }

    #[cfg(test)]
    pub(super) fn test_parts(&self) -> (ValueId, BasicBlockId, ValueId, &MirType) {
        (
            self.source_receipt.source_definition.value,
            self.node.predecessor,
            self.reservation.destination,
            &self.source_receipt.exact_type,
        )
    }
}

#[cfg(test)]
pub(super) mod test_support {
    use super::{
        ExactProducerFamilyV1, ProducerDefinitionIdentityV1, VerifiedPhiRematerializationNodeV1,
    };
    use crate::mir::builder::fact_session::FunctionFactGenerationV1;
    use crate::mir::{BasicBlockId, ValueId};

    pub(in crate::mir::builder::ssa::phi_input_materializer) const fn generation(
        value: u64,
    ) -> FunctionFactGenerationV1 {
        FunctionFactGenerationV1::for_test(value, 0)
    }

    pub(in crate::mir::builder::ssa::phi_input_materializer) const fn definition(
        generation: FunctionFactGenerationV1,
        value: ValueId,
        family: ExactProducerFamilyV1,
        recipe_fingerprint: u64,
    ) -> ProducerDefinitionIdentityV1 {
        ProducerDefinitionIdentityV1 {
            generation,
            value,
            family,
            recipe_fingerprint,
        }
    }

    pub(in crate::mir::builder::ssa::phi_input_materializer) const fn node(
        predecessor: BasicBlockId,
        source_definition: ProducerDefinitionIdentityV1,
    ) -> VerifiedPhiRematerializationNodeV1 {
        VerifiedPhiRematerializationNodeV1 {
            predecessor,
            source_definition,
        }
    }
}
