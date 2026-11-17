#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct RawPhysicalRootIdentityV1 {
    main_symbol: &'static str,
    main_arity: usize,
    condition_symbol: &'static str,
    condition_arity: usize,
}

impl RawPhysicalRootIdentityV1 {
    pub(in crate::mir) const fn fixed() -> Self {
        Self {
            main_symbol: "main",
            main_arity: 0,
            condition_symbol: "condition_fn",
            condition_arity: 1,
        }
    }

    pub(in crate::mir) const fn main_symbol(&self) -> &'static str {
        self.main_symbol
    }

    pub(in crate::mir) const fn main_arity(&self) -> usize {
        self.main_arity
    }

    pub(in crate::mir) const fn condition_symbol(&self) -> &'static str {
        self.condition_symbol
    }

    pub(in crate::mir) const fn condition_arity(&self) -> usize {
        self.condition_arity
    }
}
