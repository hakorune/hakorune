use hakorune_mir_core::MirType;

/// Pure monotone decision for one proposed lowering-time type fact.
///
/// Missing and `Unknown` are non-facts. Every other `MirType`, including
/// `Void`, is exact. This type has no storage or commit capability.
#[derive(Debug)]
pub struct TypeFactDecisionV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreparedTypeFactPublicationV1 {
    Publish(MirType),
    Idempotent(MirType),
    PreserveExisting(MirType),
    NoPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeFactDecisionErrorV1 {
    ConcreteFactConflict {
        existing: MirType,
        proposed: MirType,
    },
    UnknownProposal {
        existing: Option<MirType>,
    },
}

impl std::fmt::Display for TypeFactDecisionErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConcreteFactConflict { existing, proposed } => write!(
                f,
                "[freeze:contract][lowering_facts/type_decision/concrete_fact_conflict] \
                 existing={existing:?} proposed={proposed:?}"
            ),
            Self::UnknownProposal { existing } => write!(
                f,
                "[freeze:contract][lowering_facts/type_decision/unknown_proposal] \
                 existing={existing:?}"
            ),
        }
    }
}

impl std::error::Error for TypeFactDecisionErrorV1 {}

impl TypeFactDecisionV1 {
    /// Prepare one type publication without mutating a fact store.
    ///
    /// `None` means no proposal. `Some(Unknown)` is an explicit request to
    /// write a non-fact and therefore rejects instead of regressing an exact
    /// fact or materializing an `Unknown` entry.
    pub fn prepare(
        existing: Option<&MirType>,
        proposed: Option<&MirType>,
    ) -> Result<PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1> {
        let existing = exact_fact(existing).cloned();

        match proposed {
            Some(MirType::Unknown) => Err(TypeFactDecisionErrorV1::UnknownProposal { existing }),
            None => Ok(match existing {
                Some(existing) => PreparedTypeFactPublicationV1::PreserveExisting(existing),
                None => PreparedTypeFactPublicationV1::NoPublication,
            }),
            Some(proposed) => match existing {
                None => Ok(PreparedTypeFactPublicationV1::Publish(proposed.clone())),
                Some(existing) if existing == *proposed => {
                    Ok(PreparedTypeFactPublicationV1::Idempotent(existing))
                }
                Some(existing) => Err(TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing,
                    proposed: proposed.clone(),
                }),
            },
        }
    }
}

fn exact_fact(ty: Option<&MirType>) -> Option<&MirType> {
    match ty {
        Some(MirType::Unknown) | None => None,
        Some(ty) => Some(ty),
    }
}
