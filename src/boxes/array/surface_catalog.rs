use super::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayMethodId {
    Length,
    Get,
    Set,
    Push,
    Pop,
    Clear,
    Contains,
    Slice,
    Remove,
    Insert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArraySurfaceEffect {
    Read,
    WriteHeap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArraySurfaceReturn {
    Value,
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayExposureState {
    pub runtime_impl: bool,
    pub vm_dispatch: bool,
    pub std_sugar: bool,
    pub smoke_pinned: bool,
}

impl ArrayExposureState {
    pub const STABLE: Self = Self {
        runtime_impl: true,
        vm_dispatch: true,
        std_sugar: true,
        smoke_pinned: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayMethodSpec {
    pub id: ArrayMethodId,
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub arity: u8,
    pub slot: u16,
    pub effect: ArraySurfaceEffect,
    pub returns: ArraySurfaceReturn,
    pub exposure: ArrayExposureState,
}

impl ArrayMethodSpec {
    pub fn matches_name(&self, name: &str) -> bool {
        self.canonical == name || self.aliases.iter().any(|alias| *alias == name)
    }

    pub fn matches_signature(&self, name: &str, arity: usize) -> bool {
        self.arity as usize == arity && self.matches_name(name)
    }
}

pub const ARRAY_SURFACE_METHODS: &[ArrayMethodSpec] = &[
    ArrayMethodSpec {
        id: ArrayMethodId::Length,
        canonical: "length",
        aliases: &["size", "len"],
        arity: 0,
        slot: 102,
        effect: ArraySurfaceEffect::Read,
        returns: ArraySurfaceReturn::Value,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Get,
        canonical: "get",
        aliases: &[],
        arity: 1,
        slot: 100,
        effect: ArraySurfaceEffect::Read,
        returns: ArraySurfaceReturn::Value,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Set,
        canonical: "set",
        aliases: &[],
        arity: 2,
        slot: 101,
        effect: ArraySurfaceEffect::WriteHeap,
        returns: ArraySurfaceReturn::Void,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Push,
        canonical: "push",
        aliases: &[],
        arity: 1,
        slot: 103,
        effect: ArraySurfaceEffect::WriteHeap,
        returns: ArraySurfaceReturn::Void,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Pop,
        canonical: "pop",
        aliases: &[],
        arity: 0,
        slot: 104,
        effect: ArraySurfaceEffect::WriteHeap,
        returns: ArraySurfaceReturn::Value,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Clear,
        canonical: "clear",
        aliases: &[],
        arity: 0,
        slot: 105,
        effect: ArraySurfaceEffect::WriteHeap,
        returns: ArraySurfaceReturn::Void,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Contains,
        canonical: "contains",
        aliases: &[],
        arity: 1,
        slot: 106,
        effect: ArraySurfaceEffect::Read,
        returns: ArraySurfaceReturn::Value,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Slice,
        canonical: "slice",
        aliases: &[],
        arity: 2,
        slot: 111,
        effect: ArraySurfaceEffect::Read,
        returns: ArraySurfaceReturn::Value,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Remove,
        canonical: "remove",
        aliases: &[],
        arity: 1,
        slot: 112,
        effect: ArraySurfaceEffect::WriteHeap,
        returns: ArraySurfaceReturn::Value,
        exposure: ArrayExposureState::STABLE,
    },
    ArrayMethodSpec {
        id: ArrayMethodId::Insert,
        canonical: "insert",
        aliases: &[],
        arity: 2,
        slot: 113,
        effect: ArraySurfaceEffect::WriteHeap,
        returns: ArraySurfaceReturn::Void,
        exposure: ArrayExposureState::STABLE,
    },
];

impl ArrayMethodId {
    pub fn spec(self) -> &'static ArrayMethodSpec {
        ARRAY_SURFACE_METHODS
            .iter()
            .find(|spec| spec.id == self)
            .expect("ArrayMethodSpec missing for ArrayMethodId")
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

    pub fn effect(self) -> ArraySurfaceEffect {
        self.spec().effect
    }

    pub fn returns(self) -> ArraySurfaceReturn {
        self.spec().returns
    }

    pub fn from_name(name: &str) -> Option<Self> {
        ARRAY_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_name(name))
            .map(|spec| spec.id)
    }

    pub fn from_name_and_arity(name: &str, arity: usize) -> Option<Self> {
        ARRAY_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_signature(name, arity))
            .map(|spec| spec.id)
    }

    pub fn from_slot(slot: u16) -> Option<Self> {
        ARRAY_SURFACE_METHODS
            .iter()
            .find(|spec| spec.slot == slot)
            .map(|spec| spec.id)
    }
}

pub enum ArraySurfaceInvokeResult {
    Value(Box<dyn NyashBox>),
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArraySurfaceInvokeError {
    pub method: ArrayMethodId,
    pub expected: usize,
    pub actual: usize,
}

impl fmt::Display for ArraySurfaceInvokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ArrayBox.{} expects {} args, got {}",
            self.method.canonical_name(),
            self.expected,
            self.actual
        )
    }
}

impl ArrayBox {
    pub fn invoke_surface(
        &self,
        method: ArrayMethodId,
        args: Vec<Box<dyn NyashBox>>,
    ) -> Result<ArraySurfaceInvokeResult, ArraySurfaceInvokeError> {
        let expected = method.arity();
        let actual = args.len();
        if actual != expected {
            return Err(ArraySurfaceInvokeError {
                method,
                expected,
                actual,
            });
        }

        let mut args = args.into_iter();
        let result = match method {
            ArrayMethodId::Length => ArraySurfaceInvokeResult::Value(self.length()),
            ArrayMethodId::Get => {
                let index = args.next().expect("validated ArrayBox.get index");
                ArraySurfaceInvokeResult::Value(self.get(index))
            }
            ArrayMethodId::Set => {
                let index = args.next().expect("validated ArrayBox.set index");
                let value = args.next().expect("validated ArrayBox.set value");
                let _ = self.set(index, value);
                ArraySurfaceInvokeResult::Void
            }
            ArrayMethodId::Push => {
                let value = args.next().expect("validated ArrayBox.push value");
                let _ = self.push(value);
                ArraySurfaceInvokeResult::Void
            }
            ArrayMethodId::Pop => ArraySurfaceInvokeResult::Value(self.pop()),
            ArrayMethodId::Clear => {
                let _ = self.clear();
                ArraySurfaceInvokeResult::Void
            }
            ArrayMethodId::Contains => {
                let value = args.next().expect("validated ArrayBox.contains value");
                ArraySurfaceInvokeResult::Value(self.contains(value))
            }
            ArrayMethodId::Slice => {
                let start = args.next().expect("validated ArrayBox.slice start");
                let end = args.next().expect("validated ArrayBox.slice end");
                ArraySurfaceInvokeResult::Value(self.slice(start, end))
            }
            ArrayMethodId::Remove => {
                let index = args.next().expect("validated ArrayBox.remove index");
                ArraySurfaceInvokeResult::Value(self.remove(index))
            }
            ArrayMethodId::Insert => {
                let index = args.next().expect("validated ArrayBox.insert index");
                let value = args.next().expect("validated ArrayBox.insert value");
                let _ = self.insert(index, value);
                ArraySurfaceInvokeResult::Void
            }
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_surface_catalog_normalizes_length_aliases() {
        assert_eq!(
            ArrayMethodId::from_name("length"),
            Some(ArrayMethodId::Length)
        );
        assert_eq!(
            ArrayMethodId::from_name("size"),
            Some(ArrayMethodId::Length)
        );
        assert_eq!(ArrayMethodId::from_name("len"), Some(ArrayMethodId::Length));
        assert_eq!(
            ArrayMethodId::from_name_and_arity("size", 0),
            Some(ArrayMethodId::Length)
        );
        assert_eq!(ArrayMethodId::Length.slot(), 102);
    }

    #[test]
    fn array_surface_catalog_keeps_stable_slots() {
        assert_eq!(ArrayMethodId::from_slot(100), Some(ArrayMethodId::Get));
        assert_eq!(ArrayMethodId::from_slot(101), Some(ArrayMethodId::Set));
        assert_eq!(ArrayMethodId::from_slot(102), Some(ArrayMethodId::Length));
        assert_eq!(ArrayMethodId::from_slot(103), Some(ArrayMethodId::Push));
        assert_eq!(ArrayMethodId::from_slot(104), Some(ArrayMethodId::Pop));
        assert_eq!(ArrayMethodId::from_slot(105), Some(ArrayMethodId::Clear));
        assert_eq!(ArrayMethodId::from_slot(106), Some(ArrayMethodId::Contains));
        assert_eq!(ArrayMethodId::from_slot(111), Some(ArrayMethodId::Slice));
        assert_eq!(ArrayMethodId::from_slot(112), Some(ArrayMethodId::Remove));
        assert_eq!(ArrayMethodId::from_slot(113), Some(ArrayMethodId::Insert));
    }
}
