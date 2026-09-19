//! Physical Normal result roles; never source types or identity issuers.
use crate::mir::{Effect, EffectMask, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeNormalResultKind {
    I64,
    Handle,
    Map,
    MapKey,
    MapOutcome,
    /// Borrowed Map view produced by a checked ArrayIndex. The view has no
    /// End authority and must be consumed by the next typed read.
    MapView,
    /// Borrowed Text view produced by a checked MapGetText. The view has no
    /// End authority and cannot cross a return or call boundary.
    TextView,
}

/// Physical scalar payload; source capability cannot supply this distinction.
/// `BorrowedHandle` carries a non-consuming handle snapshot — the map never
/// owns it, and an i64 read on the tagged entry Faults instead of
/// conflating handle bits with a scalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapValueKind {
    I64,
    Bool,
    BorrowedHandle,
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
    /// Store an owned UTF-8 payload; the bytes are sealed on the source
    /// row and carried inline like a prepared key.
    InstallText {
        map: ValueId,
        key: ValueId,
        utf8: String,
    },
    /// Store an owned empty array (`[]` literal): no value operand — the
    /// sealed `NestedArray { elements: [] }` row carries no payload, so
    /// the installed marker owns the empty-array meaning directly.
    InstallEmptyArray {
        map: ValueId,
        key: ValueId,
    },
    /// Store an ordered array of already-lowered child Map values by
    /// borrowed reference. The parent owns the array residence, while the
    /// caller's MapLocal roots remain the sole End owners.
    InstallBorrowedArray {
        map: ValueId,
        key: ValueId,
        elements: Box<[ValueId]>,
    },
    /// Read one checked i64 entry by a sealed UTF-8 key. The map operand is
    /// borrowed for the read only — owned locals keep their own End and
    /// borrowed formals keep caller ownership; the op never consumes either.
    CheckedGetI64 {
        map: ValueId,
        utf8: String,
    },
    /// Read the length of an exact checked Array entry. The map remains live;
    /// the result is an i64 and the operation never publishes an Array view.
    ArrayLength {
        map: ValueId,
        utf8: String,
    },
    /// Read one Map element from an owned Array entry. The parent Map remains
    /// live; the result is a borrowed Map view with no independent cleanup.
    ArrayIndexMap {
        map: ValueId,
        utf8: String,
        index: i64,
    },
    /// Read an owned Text entry from a borrowed Map view. The result is a
    /// borrowed Text view and cannot be returned or stored by this lane.
    MapGetText {
        map: ValueId,
        utf8: String,
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
            Self::InstallIndexed { .. }
            | Self::InstallValue { .. }
            | Self::InstallText { .. }
            | Self::InstallEmptyArray { .. }
            | Self::InstallBorrowedArray { .. } => Some(InvokeNormalResultKind::MapOutcome),
            Self::CheckedGetI64 { .. } | Self::ArrayLength { .. } => {
                Some(InvokeNormalResultKind::I64)
            }
            Self::ArrayIndexMap { .. } => Some(InvokeNormalResultKind::MapView),
            Self::MapGetText { .. } => Some(InvokeNormalResultKind::TextView),
            Self::EndOutcome { .. } | Self::End { .. } => None,
        }
    }
    pub fn effects(&self) -> EffectMask {
        match self {
            Self::New | Self::PrepareKey { .. } => EffectMask::CONTROL.add(Effect::Alloc),
            Self::InstallIndexed { .. }
            | Self::InstallValue { .. }
            | Self::InstallText { .. }
            | Self::InstallEmptyArray { .. }
            | Self::InstallBorrowedArray { .. } => {
                EffectMask::WRITE.add(Effect::Alloc).add(Effect::Control)
            }
            Self::CheckedGetI64 { .. } | Self::ArrayLength { .. } => {
                EffectMask::READ.add(Effect::Control)
            }
            Self::ArrayIndexMap { .. } | Self::MapGetText { .. } => {
                EffectMask::READ.add(Effect::Control)
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
            Self::InstallText { map, key, .. } | Self::InstallEmptyArray { map, key } => {
                vec![*map, *key]
            }
            Self::InstallBorrowedArray { map, key, elements } => std::iter::once(*map)
                .chain(std::iter::once(*key))
                .chain(elements.iter().copied())
                .collect(),
            Self::CheckedGetI64 { map, .. } | Self::ArrayLength { map, .. } => vec![*map],
            Self::ArrayIndexMap { map, .. } | Self::MapGetText { map, .. } => vec![*map],
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
            Self::InstallText { map, key, .. } | Self::InstallEmptyArray { map, key } => {
                rewrite(map);
                rewrite(key);
            }
            Self::InstallBorrowedArray { map, key, elements } => {
                rewrite(map);
                rewrite(key);
                for element in elements.iter_mut() {
                    rewrite(element);
                }
            }
            Self::CheckedGetI64 { map, .. } | Self::ArrayLength { map, .. } => rewrite(map),
            Self::ArrayIndexMap { map, .. } | Self::MapGetText { map, .. } => rewrite(map),
            Self::EndOutcome { outcome } => rewrite(outcome),
            Self::End { map } => rewrite(map),
        }
    }
}
