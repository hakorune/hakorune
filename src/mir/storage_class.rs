/*!
 * MIR storage-class inventory.
 *
 * This module keeps a no-behavior-change inventory of how current MIR values
 * are represented for the primitive / user-box fast-path plan. It does not
 * change lowering or backend behavior.
 */

use super::{MirFunction, MirModule, MirType, ValueId};

/// Logical storage class for a MIR value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageClass {
    InlineI64,
    InlineBool,
    InlineF64,
    BorrowedText,
    BoxRef,
    Opaque,
}

impl StorageClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineI64 => "inline_i64",
            Self::InlineBool => "inline_bool",
            Self::InlineF64 => "inline_f64",
            Self::BorrowedText => "borrowed_text",
            Self::BoxRef => "box_ref",
            Self::Opaque => "opaque",
        }
    }
}

impl std::fmt::Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Refresh every function's storage-class inventory from the current MIR value types.
pub fn refresh_module_storage_class_facts(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_storage_class_facts(function);
    }
}

/// Refresh a single function's storage-class inventory from value types.
pub fn refresh_function_storage_class_facts(function: &mut MirFunction) {
    function.metadata.value_storage_classes.clear();

    let entries: Vec<(ValueId, StorageClass)> = function
        .metadata
        .value_types
        .iter()
        .map(|(value, ty)| (*value, infer_storage_class(ty)))
        .collect();

    for (value, class) in entries {
        function.metadata.value_storage_classes.insert(value, class);
    }

    for block in function.blocks.values() {
        for inst in &block.instructions {
            if let super::MirInstruction::FieldGet {
                dst,
                declared_type: Some(declared_type),
                ..
            } = inst
            {
                let inferred = infer_storage_class(declared_type);
                let current = function.metadata.value_storage_classes.get(dst).copied();
                if current.is_none() || current == Some(StorageClass::Opaque) {
                    function
                        .metadata
                        .value_storage_classes
                        .insert(*dst, inferred);
                }
            }
        }
    }
}

fn infer_storage_class(ty: &MirType) -> StorageClass {
    match ty {
        MirType::Integer => StorageClass::InlineI64,
        MirType::Bool => StorageClass::InlineBool,
        MirType::Float => StorageClass::InlineF64,
        MirType::String => StorageClass::BorrowedText,
        MirType::Box(name) if is_primitive_box_name(name) => match name.as_str() {
            "IntegerBox" => StorageClass::InlineI64,
            "BoolBox" => StorageClass::InlineBool,
            "FloatBox" => StorageClass::InlineF64,
            "StringBox" => StorageClass::BorrowedText,
            _ => StorageClass::Opaque,
        },
        MirType::Box(_) => StorageClass::BoxRef,
        MirType::Future(_) | MirType::Array(_) | MirType::WeakRef | MirType::Void => {
            StorageClass::Opaque
        }
        MirType::Unknown => StorageClass::Opaque,
    }
}

fn is_primitive_box_name(name: &str) -> bool {
    matches!(name, "IntegerBox" | "BoolBox" | "StringBox" | "FloatBox")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, ValueId};

    #[test]
    fn infer_storage_class_maps_known_primitive_types() {
        assert_eq!(
            infer_storage_class(&MirType::Integer),
            StorageClass::InlineI64
        );
        assert_eq!(
            infer_storage_class(&MirType::Bool),
            StorageClass::InlineBool
        );
        assert_eq!(
            infer_storage_class(&MirType::String),
            StorageClass::BorrowedText
        );
        assert_eq!(
            infer_storage_class(&MirType::Float),
            StorageClass::InlineF64
        );
        assert_eq!(
            infer_storage_class(&MirType::Box("StringBox".to_string())),
            StorageClass::BorrowedText
        );
        assert_eq!(
            infer_storage_class(&MirType::Box("IntegerBox".to_string())),
            StorageClass::InlineI64
        );
        assert_eq!(
            infer_storage_class(&MirType::Box("FloatBox".to_string())),
            StorageClass::InlineF64
        );
        assert_eq!(
            infer_storage_class(&MirType::Box("MyUserBox".to_string())),
            StorageClass::BoxRef
        );
    }

    #[test]
    fn refresh_function_collects_storage_classes_from_value_types() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .metadata
            .value_types
            .insert(ValueId::new(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId::new(2), MirType::Box("MyUserBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId::new(3), MirType::Float);

        refresh_function_storage_class_facts(&mut function);

        assert_eq!(
            function
                .metadata
                .value_storage_classes
                .get(&ValueId::new(1)),
            Some(&StorageClass::InlineI64)
        );
        assert_eq!(
            function
                .metadata
                .value_storage_classes
                .get(&ValueId::new(2)),
            Some(&StorageClass::BoxRef)
        );
        assert_eq!(
            function
                .metadata
                .value_storage_classes
                .get(&ValueId::new(3)),
            Some(&StorageClass::InlineF64)
        );
    }

    #[test]
    fn refresh_function_collects_storage_classes_from_field_get_declared_type() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(super::super::MirInstruction::FieldGet {
                dst: ValueId::new(1),
                base: ValueId::new(0),
                field: "x".to_string(),
                declared_type: Some(MirType::Box("IntegerBox".to_string())),
            });

        refresh_function_storage_class_facts(&mut function);

        assert_eq!(
            function
                .metadata
                .value_storage_classes
                .get(&ValueId::new(1)),
            Some(&StorageClass::InlineI64)
        );
    }

    #[test]
    fn refresh_function_collects_float_storage_class_from_field_get_declared_type() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(super::super::MirInstruction::FieldGet {
                dst: ValueId::new(1),
                base: ValueId::new(0),
                field: "x".to_string(),
                declared_type: Some(MirType::Box("FloatBox".to_string())),
            });

        refresh_function_storage_class_facts(&mut function);

        assert_eq!(
            function
                .metadata
                .value_storage_classes
                .get(&ValueId::new(1)),
            Some(&StorageClass::InlineF64)
        );
    }
}
