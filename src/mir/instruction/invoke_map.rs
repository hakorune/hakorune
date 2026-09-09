//! Physical opaque roles; never source types, host handles or identity issuers.
use crate::mir::{Effect, EffectMask, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeNormalResultKind {
    Handle,
    Map,
    MapKey,
    MapOutcome,
}

/// Physical scalar payload; source capability cannot supply this distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapValueKind {
    I64,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapInvokeOperation {
    New,
    PrepareKey {
        utf8: String,
    },
    InstallIndexed {
        map: ValueId,
        key: ValueId,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
    },
    InstallValue {
        map: ValueId,
        key: ValueId,
        value: ValueId,
        kind: MapValueKind,
    },
    EndOutcome {
        outcome: ValueId,
    },
    End {
        map: ValueId,
    },
}

impl MapInvokeOperation {
    pub fn normal_result_kind(&self) -> Option<InvokeNormalResultKind> {
        match self {
            Self::New => Some(InvokeNormalResultKind::Map),
            Self::PrepareKey { .. } => Some(InvokeNormalResultKind::MapKey),
            Self::InstallIndexed { .. } | Self::InstallValue { .. } => {
                Some(InvokeNormalResultKind::MapOutcome)
            }
            Self::EndOutcome { .. } | Self::End { .. } => None,
        }
    }
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::New | Self::PrepareKey { .. } => EffectMask::CONTROL.add(Effect::Alloc),
            Self::InstallIndexed { .. } | Self::InstallValue { .. } => {
                EffectMask::WRITE.add(Effect::Alloc).add(Effect::Control)
            }
            Self::EndOutcome { .. } | Self::End { .. } => EffectMask::WRITE
                .union(EffectMask::MUT)
                .union(EffectMask::IO)
                .add(Effect::Control),
        }
    }
    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::New | Self::PrepareKey { .. } => vec![],
            Self::InstallIndexed {
                map, key, value, ..
            }
            | Self::InstallValue {
                map, key, value, ..
            } => vec![*map, *key, *value],
            Self::EndOutcome { outcome } => vec![*outcome],
            Self::End { map } => vec![*map],
        }
    }
    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::New | Self::PrepareKey { .. } => {}
            Self::InstallIndexed {
                map, key, value, ..
            }
            | Self::InstallValue {
                map, key, value, ..
            } => {
                rewrite(map);
                rewrite(key);
                rewrite(value);
            }
            Self::EndOutcome { outcome } => rewrite(outcome),
            Self::End { map } => rewrite(map),
        }
    }
}
