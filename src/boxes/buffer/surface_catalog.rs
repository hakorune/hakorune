//! BufferBox visible surface catalog.
//!
//! This catalog names the user-visible BufferBox surface. It does not make
//! Buffer slots executable and does not change the VM handler dispatch owner.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferMethodId {
    Write,
    Read,
    ReadAll,
    Clear,
    Length,
    Append,
    Slice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferSurfaceEffect {
    Read,
    WriteHeap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferSurfaceReturn {
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferExposureState {
    pub runtime_impl: bool,
    pub vm_dispatch: bool,
    pub std_sugar: bool,
    pub smoke_pinned: bool,
}

impl BufferExposureState {
    pub const CURRENT_HANDLER: Self = Self {
        runtime_impl: true,
        vm_dispatch: true,
        std_sugar: false,
        smoke_pinned: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferMethodSpec {
    pub id: BufferMethodId,
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub arity: u8,
    pub slot: u16,
    pub effect: BufferSurfaceEffect,
    pub returns: BufferSurfaceReturn,
    pub exposure: BufferExposureState,
}

impl BufferMethodSpec {
    pub fn matches_name(&self, name: &str) -> bool {
        self.canonical == name || self.aliases.iter().any(|alias| *alias == name)
    }

    pub fn matches_signature(&self, name: &str, arity: usize) -> bool {
        self.arity as usize == arity && self.matches_name(name)
    }
}

pub const BUFFER_SURFACE_METHODS: &[BufferMethodSpec] = &[
    BufferMethodSpec {
        id: BufferMethodId::Write,
        canonical: "write",
        aliases: &[],
        arity: 1,
        slot: 500,
        effect: BufferSurfaceEffect::WriteHeap,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
    BufferMethodSpec {
        id: BufferMethodId::Read,
        canonical: "read",
        aliases: &[],
        arity: 1,
        slot: 501,
        effect: BufferSurfaceEffect::WriteHeap,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
    BufferMethodSpec {
        id: BufferMethodId::ReadAll,
        canonical: "readAll",
        aliases: &[],
        arity: 0,
        slot: 502,
        effect: BufferSurfaceEffect::Read,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
    BufferMethodSpec {
        id: BufferMethodId::Clear,
        canonical: "clear",
        aliases: &[],
        arity: 0,
        slot: 503,
        effect: BufferSurfaceEffect::WriteHeap,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
    BufferMethodSpec {
        id: BufferMethodId::Length,
        canonical: "length",
        aliases: &["len", "size"],
        arity: 0,
        slot: 504,
        effect: BufferSurfaceEffect::Read,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
    BufferMethodSpec {
        id: BufferMethodId::Append,
        canonical: "append",
        aliases: &[],
        arity: 1,
        slot: 505,
        effect: BufferSurfaceEffect::WriteHeap,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
    BufferMethodSpec {
        id: BufferMethodId::Slice,
        canonical: "slice",
        aliases: &[],
        arity: 2,
        slot: 506,
        effect: BufferSurfaceEffect::Read,
        returns: BufferSurfaceReturn::Value,
        exposure: BufferExposureState::CURRENT_HANDLER,
    },
];

impl BufferMethodId {
    pub fn spec(self) -> &'static BufferMethodSpec {
        BUFFER_SURFACE_METHODS
            .iter()
            .find(|spec| spec.id == self)
            .expect("BufferMethodSpec missing for BufferMethodId")
    }

    pub fn canonical_name(self) -> &'static str {
        self.spec().canonical
    }

    pub fn aliases(self) -> &'static [&'static str] {
        self.spec().aliases
    }

    pub fn arity(self) -> usize {
        self.spec().arity as usize
    }

    pub fn slot(self) -> u16 {
        self.spec().slot
    }

    pub fn effect(self) -> BufferSurfaceEffect {
        self.spec().effect
    }

    pub fn returns(self) -> BufferSurfaceReturn {
        self.spec().returns
    }

    pub fn from_name(name: &str) -> Option<Self> {
        BUFFER_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_name(name))
            .map(|spec| spec.id)
    }

    pub fn from_name_and_arity(name: &str, arity: usize) -> Option<Self> {
        BUFFER_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_signature(name, arity))
            .map(|spec| spec.id)
    }

    pub fn from_slot(slot: u16) -> Option<Self> {
        BUFFER_SURFACE_METHODS
            .iter()
            .find(|spec| spec.slot == slot)
            .map(|spec| spec.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_surface_catalog_names_visible_methods() {
        let names: Vec<_> = BUFFER_SURFACE_METHODS
            .iter()
            .map(|spec| (spec.canonical, spec.arity))
            .collect();

        assert_eq!(
            names,
            vec![
                ("write", 1),
                ("read", 1),
                ("readAll", 0),
                ("clear", 0),
                ("length", 0),
                ("append", 1),
                ("slice", 2),
            ]
        );
    }

    #[test]
    fn buffer_surface_catalog_resolves_aliases_and_slots() {
        assert_eq!(
            BufferMethodId::from_name("len"),
            Some(BufferMethodId::Length)
        );
        assert_eq!(
            BufferMethodId::from_name_and_arity("size", 0),
            Some(BufferMethodId::Length)
        );
        assert_eq!(BufferMethodId::from_name_and_arity("slice", 1), None);
        assert_eq!(BufferMethodId::from_slot(506), Some(BufferMethodId::Slice));
    }
}
