//! Affine String live-state operations, shared by the path verifier.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Residence {
    Home(ValueId),
    Carrier(ValueId),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Carrier {
    Static,
    Owned(ValueId),
    Ended,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Live {
    pub(super) temporary: Vec<ValueId>,
    pub(super) homes: Vec<Residence>,
    pub(super) carriers: BTreeMap<ValueId, Carrier>,
    pub(super) replacement: Option<(ValueId, ValueId)>,
    pub(super) hold: Option<ValueId>,
    pub(super) call_pending: Option<ValueId>,
}
impl Live {
    pub(super) fn has_home(&self, value: ValueId) -> bool {
        self.homes.contains(&Residence::Home(value))
    }
    pub(super) fn contains(&self, value: &ValueId) -> bool {
        self.temporary.contains(value) || self.has_home(*value)
    }
    pub(super) fn active(&self, home: Residence) -> bool {
        match home {
            Residence::Home(_) => true,
            Residence::Carrier(slot) => matches!(self.carriers.get(&slot), Some(Carrier::Owned(_))),
        }
    }
    pub(super) fn is_empty(&self) -> bool {
        self.temporary.is_empty()
            && !self.homes.iter().any(|home| self.active(*home))
            && self
                .carriers
                .values()
                .all(|state| !matches!(state, Carrier::Owned(_)))
            && self.replacement.is_none()
            && self.hold.is_none()
            && self.call_pending.is_none()
    }
    pub(super) fn compatible_join(&self, other: &Self, source: &TextSourceImage) -> bool {
        let raw_homes = |homes: &[Residence]| {
            homes
                .iter()
                .filter_map(|home| match home {
                    Residence::Home(value) => Some(*value),
                    Residence::Carrier(_) => None,
                })
                .collect::<Vec<_>>()
        };
        self.temporary == other.temporary
            && raw_homes(&self.homes) == raw_homes(&other.homes)
            && self.replacement == other.replacement
            && self.hold == other.hold
            && self.call_pending == other.call_pending
            && self
                .carriers
                .keys()
                .chain(other.carriers.keys())
                .all(|slot| source.has_slot(*slot))
    }
    pub(super) fn descriptor_live(&self, value: ValueId, source: &TextSourceImage) -> bool {
        match source.origin(value) {
            Some(Origin::Slot(slot)) => matches!(
                self.carriers.get(&slot),
                Some(Carrier::Static | Carrier::Owned(_))
            ),
            _ => false,
        }
    }
    pub(super) fn end(
        &mut self,
        value: ValueId,
        faulted: bool,
        retained: &[ValueId],
        source: &TextSourceImage,
    ) -> bool {
        if let Some(top) = self.temporary.last() {
            if *top != value {
                return false;
            }
            self.temporary.pop();
            if self
                .call_pending
                .is_some_and(|held| self.temporary.last() == Some(&held))
            {
                self.call_pending = None;
            }
            return true;
        }
        if let Some((new, old)) = self.replacement {
            if !faulted {
                if value != old || self.homes.last() != Some(&Residence::Home(new)) {
                    return false;
                }
                let Some(index) = self
                    .homes
                    .iter()
                    .position(|home| *home == Residence::Home(old))
                else {
                    return false;
                };
                self.homes.remove(index);
                self.replacement = None;
                return true;
            }
            if value == new {
                self.replacement = None;
            }
        }
        let mut keep = Vec::new();
        if !faulted {
            for retained in retained {
                let home = match source.origin(*retained) {
                    Some(Origin::Raw(raw)) if self.has_home(raw) => Residence::Home(raw),
                    Some(Origin::Slot(slot))
                        if matches!(self.carriers.get(&slot), Some(Carrier::Owned(_))) =>
                    {
                        Residence::Carrier(slot)
                    }
                    Some(Origin::Static) => continue,
                    Some(Origin::Slot(slot))
                        if self.carriers.get(&slot) == Some(&Carrier::Static) =>
                    {
                        continue
                    }
                    _ => return false,
                };
                if home == Residence::Home(value) {
                    return false;
                }
                keep.push(home);
            }
        }
        let index = self.homes.iter().rposition(|home| {
            self.active(*home)
                && !keep.contains(home)
                && (faulted || !matches!(home, Residence::Home(value) if Some(*value) == self.hold))
        });
        let Some(index) = index else {
            return false;
        };
        if self.homes[index] != Residence::Home(value) {
            return false;
        }
        self.homes.remove(index);
        if self.hold == Some(value) {
            self.hold = None;
        }
        true
    }
    pub(super) fn end_carrier(
        &mut self,
        value: ValueId,
        faulted: bool,
        source: &TextSourceImage,
    ) -> bool {
        let Some(Origin::Slot(slot)) = source.origin(value) else {
            return false;
        };
        let Some(state @ (Carrier::Static | Carrier::Owned(_))) = self.carriers.get(&slot).copied()
        else {
            return false;
        };
        if !self.temporary.is_empty() {
            return false;
        }
        if let Some((new, old)) = self.replacement.filter(|_| !faulted) {
            if old != value || self.homes.last() != Some(&Residence::Home(new)) {
                return false;
            }
            self.replacement = None;
        } else if matches!(state, Carrier::Owned(_))
            && self.homes.iter().rev().find(|home| self.active(**home))
                != Some(&Residence::Carrier(slot))
        {
            return false;
        }
        if matches!(state, Carrier::Owned(_)) {
            self.carriers.insert(slot, Carrier::Ended);
        }
        true
    }
    pub(super) fn edge(
        &mut self,
        predecessor: BasicBlockId,
        target: BasicBlockId,
        source: &TextSourceImage,
    ) -> bool {
        let Some(transfers) = source.edges.get(&(predecessor, target)) else {
            return true;
        };
        // Resolve every incoming against the SAME predecessor state before any
        // simultaneous Phi transfer can change another descriptor's meaning.
        let mut pending = Vec::new();
        for (slot, value) in transfers {
            let Some(origin) = source.origin(*value) else {
                return false;
            };
            let state = match origin {
                Origin::Static => Carrier::Static,
                Origin::Raw(value) if self.has_home(value) => Carrier::Owned(value),
                Origin::Slot(previous) if previous == *slot => match self.carriers.get(&previous) {
                    Some(state @ (Carrier::Static | Carrier::Owned(_))) => *state,
                    _ => return false,
                },
                // Distinct source carrier lineages need an explicit affine
                // forwarding proof; a matching payload never supplies it.
                _ => return false,
            };
            pending.push((*slot, origin, state));
        }
        for (slot, origin, state) in pending {
            if matches!(origin, Origin::Slot(previous) if previous == slot) {
                continue;
            }
            if matches!(self.carriers.get(&slot), Some(Carrier::Owned(_))) {
                return false;
            }
            self.homes.retain(|home| *home != Residence::Carrier(slot));
            match origin {
                Origin::Raw(value) => {
                    let Some(index) = self
                        .homes
                        .iter()
                        .position(|home| *home == Residence::Home(value))
                    else {
                        return false;
                    };
                    self.homes[index] = Residence::Carrier(slot);
                }
                Origin::Static => self.homes.push(Residence::Carrier(slot)),
                Origin::Slot(_) => return false,
            }
            self.carriers.insert(slot, state);
        }
        true
    }
}
