/*!
 * MIR-owned raw layout vocabulary for allocator-grade substrate rows.
 *
 * This is intentionally not source syntax. It gives later `.hako` / parser
 * rows a small, auditable target for fixed-layout metadata instead of reusing
 * semantic `box` fields or backend-local layout guesses.
 */

use crate::mir::numeric_substrate::{
    classify_numeric_type_name, NumericSignedness, NumericTypeName, NumericWidth,
};

pub const RAW_LAYOUT_REPR_C_V0: &str = "repr_c_v0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawLayoutRepr {
    C,
}

impl RawLayoutRepr {
    pub fn as_str(self) -> &'static str {
        match self {
            RawLayoutRepr::C => RAW_LAYOUT_REPR_C_V0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawLayoutScalarStorage {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

impl RawLayoutScalarStorage {
    pub fn size_bytes(self) -> u32 {
        match self {
            RawLayoutScalarStorage::I8 | RawLayoutScalarStorage::U8 => 1,
            RawLayoutScalarStorage::I16 | RawLayoutScalarStorage::U16 => 2,
            RawLayoutScalarStorage::I32 | RawLayoutScalarStorage::U32 => 4,
            RawLayoutScalarStorage::I64 | RawLayoutScalarStorage::U64 => 8,
        }
    }

    pub fn align_bytes(self) -> u32 {
        self.size_bytes()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawLayoutFieldDecl<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawLayoutFieldPlan {
    pub name: String,
    pub declared_type_name: String,
    pub storage: RawLayoutScalarStorage,
    pub offset_bytes: u32,
    pub size_bytes: u32,
    pub align_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawLayoutPlan {
    pub name: String,
    pub repr: RawLayoutRepr,
    pub layout_kind: String,
    pub size_bytes: u32,
    pub align_bytes: u32,
    pub fields: Vec<RawLayoutFieldPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawLayoutError {
    EmptyLayoutName,
    EmptyFieldSet {
        layout_name: String,
    },
    EmptyFieldName {
        layout_name: String,
    },
    DuplicateField {
        layout_name: String,
        field: String,
    },
    UnsupportedFieldType {
        layout_name: String,
        field: String,
        type_name: String,
        reason: &'static str,
    },
    SizeOverflow {
        layout_name: String,
    },
}

impl std::fmt::Display for RawLayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawLayoutError::EmptyLayoutName => write!(f, "[raw-layout] empty layout name"),
            RawLayoutError::EmptyFieldSet { layout_name } => {
                write!(f, "[raw-layout] empty field set: {layout_name}")
            }
            RawLayoutError::EmptyFieldName { layout_name } => {
                write!(f, "[raw-layout] empty field name: {layout_name}")
            }
            RawLayoutError::DuplicateField { layout_name, field } => {
                write!(f, "[raw-layout] duplicate field: {layout_name}.{field}")
            }
            RawLayoutError::UnsupportedFieldType {
                layout_name,
                field,
                type_name,
                reason,
            } => write!(
                f,
                "[raw-layout] unsupported field type: {layout_name}.{field}: {type_name} ({reason})"
            ),
            RawLayoutError::SizeOverflow { layout_name } => {
                write!(f, "[raw-layout] layout size overflow: {layout_name}")
            }
        }
    }
}

impl std::error::Error for RawLayoutError {}

pub(crate) fn scalar_storage_for_raw_layout_type(
    type_name: &str,
) -> Result<RawLayoutScalarStorage, &'static str> {
    match classify_numeric_type_name(type_name) {
        Some(NumericTypeName {
            signedness: NumericSignedness::Signed,
            width: NumericWidth::Bits8,
        }) => Ok(RawLayoutScalarStorage::I8),
        Some(NumericTypeName {
            signedness: NumericSignedness::Signed,
            width: NumericWidth::Bits16,
        }) => Ok(RawLayoutScalarStorage::I16),
        Some(NumericTypeName {
            signedness: NumericSignedness::Signed,
            width: NumericWidth::Bits32,
        }) => Ok(RawLayoutScalarStorage::I32),
        Some(NumericTypeName {
            signedness: NumericSignedness::Signed,
            width: NumericWidth::Bits64,
        }) => Ok(RawLayoutScalarStorage::I64),
        Some(NumericTypeName {
            signedness: NumericSignedness::Unsigned,
            width: NumericWidth::Bits8,
        }) => Ok(RawLayoutScalarStorage::U8),
        Some(NumericTypeName {
            signedness: NumericSignedness::Unsigned,
            width: NumericWidth::Bits16,
        }) => Ok(RawLayoutScalarStorage::U16),
        Some(NumericTypeName {
            signedness: NumericSignedness::Unsigned,
            width: NumericWidth::Bits32,
        }) => Ok(RawLayoutScalarStorage::U32),
        Some(NumericTypeName {
            signedness: NumericSignedness::Unsigned,
            width: NumericWidth::Bits64,
        }) => Ok(RawLayoutScalarStorage::U64),
        Some(NumericTypeName {
            width: NumericWidth::Pointer,
            ..
        }) => Err("pointer-sized fields require a target ABI row"),
        None => Err("only fixed-width numeric fields are live"),
    }
}

pub(crate) fn build_repr_c_v0_raw_layout(
    layout_name: &str,
    fields: &[RawLayoutFieldDecl<'_>],
) -> Result<RawLayoutPlan, RawLayoutError> {
    if layout_name.trim().is_empty() {
        return Err(RawLayoutError::EmptyLayoutName);
    }
    if fields.is_empty() {
        return Err(RawLayoutError::EmptyFieldSet {
            layout_name: layout_name.to_string(),
        });
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut offset = 0_u32;
    let mut max_align = 1_u32;
    let mut planned = Vec::with_capacity(fields.len());

    for field in fields {
        if field.name.trim().is_empty() {
            return Err(RawLayoutError::EmptyFieldName {
                layout_name: layout_name.to_string(),
            });
        }
        if !seen.insert(field.name.to_string()) {
            return Err(RawLayoutError::DuplicateField {
                layout_name: layout_name.to_string(),
                field: field.name.to_string(),
            });
        }

        let storage = scalar_storage_for_raw_layout_type(field.type_name).map_err(|reason| {
            RawLayoutError::UnsupportedFieldType {
                layout_name: layout_name.to_string(),
                field: field.name.to_string(),
                type_name: field.type_name.to_string(),
                reason,
            }
        })?;
        let align = storage.align_bytes();
        let size = storage.size_bytes();
        offset = align_up_u32(offset, align).ok_or_else(|| RawLayoutError::SizeOverflow {
            layout_name: layout_name.to_string(),
        })?;
        let field_offset = offset;
        offset = offset
            .checked_add(size)
            .ok_or_else(|| RawLayoutError::SizeOverflow {
                layout_name: layout_name.to_string(),
            })?;
        max_align = max_align.max(align);

        planned.push(RawLayoutFieldPlan {
            name: field.name.to_string(),
            declared_type_name: field.type_name.to_string(),
            storage,
            offset_bytes: field_offset,
            size_bytes: size,
            align_bytes: align,
        });
    }

    let size_bytes =
        align_up_u32(offset, max_align).ok_or_else(|| RawLayoutError::SizeOverflow {
            layout_name: layout_name.to_string(),
        })?;

    Ok(RawLayoutPlan {
        name: layout_name.to_string(),
        repr: RawLayoutRepr::C,
        layout_kind: RAW_LAYOUT_REPR_C_V0.to_string(),
        size_bytes,
        align_bytes: max_align,
        fields: planned,
    })
}

fn align_up_u32(value: u32, align: u32) -> Option<u32> {
    debug_assert!(align.is_power_of_two());
    let mask = align.checked_sub(1)?;
    value.checked_add(mask).map(|v| v & !mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_repr_c_v0_offsets_for_fixed_width_numeric_fields() {
        let plan = build_repr_c_v0_raw_layout(
            "MiPage",
            &[
                RawLayoutFieldDecl {
                    name: "used",
                    type_name: "u32",
                },
                RawLayoutFieldDecl {
                    name: "block_size",
                    type_name: "u64",
                },
                RawLayoutFieldDecl {
                    name: "flags",
                    type_name: "u8",
                },
            ],
        )
        .expect("layout");

        assert_eq!(plan.layout_kind, RAW_LAYOUT_REPR_C_V0);
        assert_eq!(plan.repr.as_str(), RAW_LAYOUT_REPR_C_V0);
        assert_eq!(plan.align_bytes, 8);
        assert_eq!(plan.size_bytes, 24);
        assert_eq!(plan.fields[0].offset_bytes, 0);
        assert_eq!(plan.fields[1].offset_bytes, 8);
        assert_eq!(plan.fields[2].offset_bytes, 16);
        assert_eq!(plan.fields[1].storage, RawLayoutScalarStorage::U64);
    }

    #[test]
    fn rejects_pointer_sized_fields_until_target_abi_row_lands() {
        let err = build_repr_c_v0_raw_layout(
            "NeedsTarget",
            &[RawLayoutFieldDecl {
                name: "size",
                type_name: "usize",
            }],
        )
        .expect_err("usize is not live for raw layout");

        assert!(matches!(
            err,
            RawLayoutError::UnsupportedFieldType { reason, .. }
                if reason == "pointer-sized fields require a target ABI row"
        ));
    }

    #[test]
    fn rejects_box_style_or_unknown_field_types() {
        let err = build_repr_c_v0_raw_layout(
            "NotBox",
            &[RawLayoutFieldDecl {
                name: "payload",
                type_name: "StringBox",
            }],
        )
        .expect_err("box style fields are not raw layout fields");

        assert!(matches!(
            err,
            RawLayoutError::UnsupportedFieldType { reason, .. }
                if reason == "only fixed-width numeric fields are live"
        ));
    }

    #[test]
    fn rejects_duplicate_fields() {
        let err = build_repr_c_v0_raw_layout(
            "Dup",
            &[
                RawLayoutFieldDecl {
                    name: "x",
                    type_name: "i32",
                },
                RawLayoutFieldDecl {
                    name: "x",
                    type_name: "i32",
                },
            ],
        )
        .expect_err("duplicate field");

        assert!(matches!(err, RawLayoutError::DuplicateField { field, .. } if field == "x"));
    }
}
