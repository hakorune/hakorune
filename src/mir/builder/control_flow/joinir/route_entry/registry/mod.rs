//! M10b-I0-R0: the ordered recipe-first route scheduler has been retired.
//!
//! The frozen `route_loop` entry now runs one fixed order — located source
//! -> exact membership -> one policy winner -> verified recipe -> one
//! physical admission -> one canonical physicalizer — and this module keeps
//! only the retained route-shape predicates the membership seam evaluates.

pub(crate) mod predicates;
