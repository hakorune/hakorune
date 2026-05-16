#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericSignedness {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
    Pointer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericResolvedWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericTarget {
    pointer_width: NumericResolvedWidth,
}

impl NumericTarget {
    #[cfg(target_pointer_width = "32")]
    pub(crate) const HOST: Self = Self {
        pointer_width: NumericResolvedWidth::Bits32,
    };

    #[cfg(target_pointer_width = "64")]
    pub(crate) const HOST: Self = Self {
        pointer_width: NumericResolvedWidth::Bits64,
    };

    pub(crate) const fn host() -> Self {
        Self::HOST
    }

    #[allow(dead_code)] // 294x-15 explicit target layout entry; cross-target producers land later.
    pub(crate) const fn from_pointer_width(pointer_width: NumericResolvedWidth) -> Option<Self> {
        match pointer_width {
            NumericResolvedWidth::Bits32 | NumericResolvedWidth::Bits64 => {
                Some(Self { pointer_width })
            }
            NumericResolvedWidth::Bits8 | NumericResolvedWidth::Bits16 => None,
        }
    }

    pub(crate) const fn pointer_width(self) -> NumericResolvedWidth {
        self.pointer_width
    }
}

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Hakorune numeric substrate requires an explicit 32-bit or 64-bit pointer target");

impl NumericWidth {
    pub(crate) const fn resolve_for_target(self, target: NumericTarget) -> NumericResolvedWidth {
        match self {
            Self::Bits8 => NumericResolvedWidth::Bits8,
            Self::Bits16 => NumericResolvedWidth::Bits16,
            Self::Bits32 => NumericResolvedWidth::Bits32,
            Self::Bits64 => NumericResolvedWidth::Bits64,
            Self::Pointer => target.pointer_width(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericTypeName {
    pub signedness: NumericSignedness,
    pub width: NumericWidth,
}

impl NumericTypeName {
    pub(crate) const fn kind_for_target(self, target: NumericTarget) -> NumericKind {
        NumericKind {
            signedness: self.signedness,
            width: self.width.resolve_for_target(target),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericKind {
    pub signedness: NumericSignedness,
    pub width: NumericResolvedWidth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExactNumericRange {
    pub min: i128,
    pub max: i128,
}

impl NumericResolvedWidth {
    pub(crate) const fn bits(self) -> u32 {
        match self {
            Self::Bits8 => 8,
            Self::Bits16 => 16,
            Self::Bits32 => 32,
            Self::Bits64 => 64,
        }
    }
}

impl NumericKind {
    pub(crate) fn value_range(self) -> ExactNumericRange {
        let bits = self.width.bits();
        match self.signedness {
            NumericSignedness::Signed => {
                let magnitude = 1_i128 << (bits - 1);
                ExactNumericRange {
                    min: -magnitude,
                    max: magnitude - 1,
                }
            }
            NumericSignedness::Unsigned => ExactNumericRange {
                min: 0,
                max: (1_i128 << bits) - 1,
            },
        }
    }
}

pub(crate) fn classify_numeric_type_name(name: &str) -> Option<NumericTypeName> {
    let (signedness, width) = match name {
        "i8" => (NumericSignedness::Signed, NumericWidth::Bits8),
        "i16" => (NumericSignedness::Signed, NumericWidth::Bits16),
        "i32" => (NumericSignedness::Signed, NumericWidth::Bits32),
        "i64" => (NumericSignedness::Signed, NumericWidth::Bits64),
        "isize" => (NumericSignedness::Signed, NumericWidth::Pointer),
        "u8" => (NumericSignedness::Unsigned, NumericWidth::Bits8),
        "u16" => (NumericSignedness::Unsigned, NumericWidth::Bits16),
        "u32" => (NumericSignedness::Unsigned, NumericWidth::Bits32),
        "u64" => (NumericSignedness::Unsigned, NumericWidth::Bits64),
        "usize" => (NumericSignedness::Unsigned, NumericWidth::Pointer),
        _ => return None,
    };
    Some(NumericTypeName { signedness, width })
}

pub(crate) fn classify_numeric_kind_for_target(
    name: &str,
    target: NumericTarget,
) -> Option<NumericKind> {
    classify_numeric_type_name(name).map(|type_name| type_name.kind_for_target(target))
}
