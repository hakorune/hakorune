/*!
 * MIR Effect System - shared pure bitset substrate.
 *
 * This crate keeps the effect mask substrate separate from the larger
 * `src/mir` workspace so future crate seams can be split mechanically.
 */

use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// Effect flags for tracking side effects and enabling optimizations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectMask(u16);

/// Individual effect types for the 4-category MIR hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Pure = 0x0001,
    Mut = 0x0002,
    Io = 0x0004,
    Control = 0x0008,
    ReadHeap = 0x0010,
    WriteHeap = 0x0020,
    P2P = 0x0040,
    FFI = 0x0080,
    Panic = 0x0100,
    Alloc = 0x0200,
    Global = 0x0400,
    Async = 0x0800,
    Unsafe = 0x1000,
    Debug = 0x2000,
    Barrier = 0x4000,
}

impl EffectMask {
    pub const PURE: Self = Self(Effect::Pure as u16);
    pub const MUT: Self = Self(Effect::Mut as u16);
    pub const IO: Self = Self(Effect::Io as u16);
    pub const CONTROL: Self = Self(Effect::Control as u16);
    pub const READ: Self = Self(Effect::ReadHeap as u16);
    pub const READ_ALIAS: Self = Self::READ;
    pub const WRITE: Self = Self((Effect::WriteHeap as u16) | (Effect::ReadHeap as u16));
    pub const P2P: Self = Self(Effect::P2P as u16);
    pub const PANIC: Self = Self(Effect::Panic as u16);
    pub const ALL: Self = Self(0xFFFF);

    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub fn bits(self) -> u16 {
        self.0
    }

    pub fn add(self, effect: Effect) -> Self {
        Self(self.0 | (effect as u16))
    }

    pub fn remove(self, effect: Effect) -> Self {
        Self(self.0 & !(effect as u16))
    }

    pub fn contains(self, effect: Effect) -> bool {
        (self.0 & (effect as u16)) != 0
    }

    pub fn contains_any(self, mask: EffectMask) -> bool {
        (self.0 & mask.0) != 0
    }

    pub fn contains_all(self, mask: EffectMask) -> bool {
        (self.0 & mask.0) == mask.0
    }

    pub fn union(self, other: EffectMask) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersection(self, other: EffectMask) -> Self {
        Self(self.0 & other.0)
    }

    pub fn is_pure(self) -> bool {
        !self.contains(Effect::ReadHeap)
            && !self.is_mut()
            && !self.is_io()
            && !self.is_control()
    }

    pub fn is_mut(self) -> bool {
        self.contains(Effect::Mut)
            || self.contains(Effect::WriteHeap)
            || self.contains(Effect::Alloc)
    }

    pub fn is_io(self) -> bool {
        self.contains(Effect::Io)
            || self.contains(Effect::P2P)
            || self.contains(Effect::FFI)
            || self.contains(Effect::Global)
            || self.contains(Effect::Async)
            || self.contains(Effect::Unsafe)
            || self.contains(Effect::Debug)
            || self.contains(Effect::Barrier)
            || self.contains(Effect::Panic)
    }

    pub fn is_control(self) -> bool {
        self.contains(Effect::Control)
    }

    pub fn primary_category(self) -> Effect {
        if self.is_control() {
            Effect::Control
        } else if self.is_io() {
            Effect::Io
        } else if self.is_mut() {
            Effect::Mut
        } else {
            Effect::Pure
        }
    }

    pub fn is_read_only(self) -> bool {
        !self.is_mut() && !self.is_io()
    }

    pub fn is_parallel_safe(self) -> bool {
        !self.contains(Effect::WriteHeap)
            && !self.contains(Effect::Global)
            && !self.contains(Effect::Unsafe)
    }

    pub fn is_moveable(self) -> bool {
        self.is_pure() || self.is_read_only()
    }

    pub fn effect_names(self) -> Vec<&'static str> {
        let mut names = Vec::new();

        if self.contains(Effect::Pure) {
            names.push("pure");
        }
        if self.contains(Effect::Mut) {
            names.push("mut");
        }
        if self.contains(Effect::Io) {
            names.push("io");
        }
        if self.contains(Effect::Control) {
            names.push("control");
        }
        if self.contains(Effect::ReadHeap) {
            names.push("read");
        }
        if self.contains(Effect::WriteHeap) {
            names.push("write");
        }
        if self.contains(Effect::P2P) {
            names.push("p2p");
        }
        if self.contains(Effect::FFI) {
            names.push("ffi");
        }
        if self.contains(Effect::Panic) {
            names.push("panic");
        }
        if self.contains(Effect::Alloc) {
            names.push("alloc");
        }
        if self.contains(Effect::Global) {
            names.push("global");
        }
        if self.contains(Effect::Async) {
            names.push("async");
        }
        if self.contains(Effect::Unsafe) {
            names.push("unsafe");
        }
        if self.contains(Effect::Debug) {
            names.push("debug");
        }
        if self.contains(Effect::Barrier) {
            names.push("barrier");
        }

        names
    }
}

impl fmt::Display for EffectMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06x}", self.bits())
    }
}

impl BitOr for EffectMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for EffectMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for EffectMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for EffectMask {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for EffectMask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for EffectMask {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for EffectMask {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
