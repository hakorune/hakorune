//! Private lifecycle observations for clone-scrubbing function metadata slots.
//!
//! `None` is not expressive enough at this boundary: an empty ordinary slot
//! and a scrubbed clone are different lifecycle states.  The vocabulary stays
//! below the MIR metadata/census boundary; callers receive only the co-sealed
//! pair view.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinearSlotObservation<'a, T> {
    Empty,
    Occupied(&'a T),
    Scrubbed,
}
