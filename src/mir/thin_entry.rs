/*!
 * Thin-entry inventory for lifecycle/value routes.
 *
 * This module keeps no-behavior-change facts about where canonical MIR already
 * exposes a candidate for "public entry vs thin internal entry" selection.
 * It does not add a new semantic call dialect; it annotates the current MIR so
 * later pass + manifest work can bind physical entries without guessing from
 * backend helper names.
 */

use super::{BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId};
use crate::mir::definitions::call_unified::{Callee, CalleeBoxKind, TypeCertainty};
use crate::mir::function::ModuleMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinEntrySurface {
    UserBoxMethod,
    UserBoxFieldGet,
    UserBoxFieldSet,
    VariantMake,
    VariantTag,
    VariantProject,
}

impl std::fmt::Display for ThinEntrySurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserBoxMethod => f.write_str("user_box_method"),
            Self::UserBoxFieldGet => f.write_str("user_box_field_get"),
            Self::UserBoxFieldSet => f.write_str("user_box_field_set"),
            Self::VariantMake => f.write_str("variant_make"),
            Self::VariantTag => f.write_str("variant_tag"),
            Self::VariantProject => f.write_str("variant_project"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinEntryPreferredEntry {
    PublicEntry,
    ThinInternalEntry,
}

impl std::fmt::Display for ThinEntryPreferredEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublicEntry => f.write_str("public_entry"),
            Self::ThinInternalEntry => f.write_str("thin_internal_entry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinEntryCurrentCarrier {
    PublicRuntime,
    BackendTyped,
    CompatBox,
}

impl std::fmt::Display for ThinEntryCurrentCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublicRuntime => f.write_str("public_runtime"),
            Self::BackendTyped => f.write_str("backend_typed"),
            Self::CompatBox => f.write_str("compat_box"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinEntryValueClass {
    Unknown,
    InlineI64,
    InlineBool,
    InlineF64,
    BorrowedText,
    Handle,
    AggLocal,
}

impl std::fmt::Display for ThinEntryValueClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_str("?"),
            Self::InlineI64 => f.write_str("inline_i64"),
            Self::InlineBool => f.write_str("inline_bool"),
            Self::InlineF64 => f.write_str("inline_f64"),
            Self::BorrowedText => f.write_str("borrowed_text"),
            Self::Handle => f.write_str("handle"),
            Self::AggLocal => f.write_str("agg_local"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThinEntryCandidate {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub value: Option<ValueId>,
    pub surface: ThinEntrySurface,
    pub subject: String,
    pub preferred_entry: ThinEntryPreferredEntry,
    pub current_carrier: ThinEntryCurrentCarrier,
    pub value_class: ThinEntryValueClass,
    pub reason: String,
}

impl ThinEntryCandidate {
    pub fn summary(&self) -> String {
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        format!(
            "bb{}#{} {} {} preferred={} current={} value_class={}{} reason={}",
            self.block.as_u32(),
            self.instruction_index,
            self.surface,
            self.subject,
            self.preferred_entry,
            self.current_carrier,
            self.value_class,
            value_suffix,
            self.reason
        )
    }
}

pub fn refresh_module_thin_entry_candidates(module: &mut MirModule) {
    let metadata = &module.metadata;
    for function in module.functions.values_mut() {
        refresh_function_thin_entry_candidates(function, metadata);
    }
}

pub fn refresh_function_thin_entry_candidates(
    function: &mut MirFunction,
    module_metadata: &ModuleMetadata,
) {
    function.metadata.thin_entry_candidates.clear();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(candidate) =
                infer_candidate(function, module_metadata, block_id, instruction_index, inst)
            {
                function.metadata.thin_entry_candidates.push(candidate);
            }
        }
    }
}

fn infer_candidate(
    function: &MirFunction,
    module_metadata: &ModuleMetadata,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<ThinEntryCandidate> {
    match inst {
        MirInstruction::FieldGet {
            dst,
            base,
            field,
            declared_type,
        } => {
            let box_name = known_user_box_name(function, module_metadata, *base)?;
            let field_decl = known_non_weak_field_decl(module_metadata, box_name, field)?;
            let fallback_declared_type =
                type_name_hint_to_mir(field_decl.declared_type_name.as_deref());
            let value_class = value_class_from_declared_type(
                declared_type
                    .as_ref()
                    .or(fallback_declared_type.as_ref()),
            );
            Some(ThinEntryCandidate {
                block,
                instruction_index,
                value: Some(*dst),
                surface: ThinEntrySurface::UserBoxFieldGet,
                subject: format!("{}.{}", box_name, field),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: field_current_carrier(value_class),
                value_class,
                reason: if field_current_carrier(value_class)
                    == ThinEntryCurrentCarrier::BackendTyped
                {
                    "known non-weak primitive field already reaches a backend-typed helper lane; manifest should own the thin/public entry split".to_string()
                } else {
                    "known non-weak user-box field can choose a thin internal entry below canonical field.get".to_string()
                },
            })
        }
        MirInstruction::FieldSet {
            base,
            field,
            declared_type,
            ..
        } => {
            let box_name = known_user_box_name(function, module_metadata, *base)?;
            let field_decl = known_non_weak_field_decl(module_metadata, box_name, field)?;
            let fallback_declared_type =
                type_name_hint_to_mir(field_decl.declared_type_name.as_deref());
            let value_class = value_class_from_declared_type(
                declared_type
                    .as_ref()
                    .or(fallback_declared_type.as_ref()),
            );
            Some(ThinEntryCandidate {
                block,
                instruction_index,
                value: None,
                surface: ThinEntrySurface::UserBoxFieldSet,
                subject: format!("{}.{}", box_name, field),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: field_current_carrier(value_class),
                value_class,
                reason: if field_current_carrier(value_class)
                    == ThinEntryCurrentCarrier::BackendTyped
                {
                    "known non-weak primitive field write already reaches a backend-typed helper lane; manifest should own the thin/public entry split".to_string()
                } else {
                    "known non-weak user-box field write can choose a thin internal entry below canonical field.set".to_string()
                },
            })
        }
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(_),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::UserDefined,
                }),
            ..
        } if !is_synthetic_payload_box(box_name) && !is_runtime_sum_box(box_name) => {
            Some(ThinEntryCandidate {
                block,
                instruction_index,
                value: *dst,
                surface: ThinEntrySurface::UserBoxMethod,
                subject: format!("{}.{}", box_name, method),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::PublicRuntime,
                value_class: ThinEntryValueClass::Unknown,
                reason: "known user-defined receiver already has canonical Call; pass + manifest can bind a thin internal entry without adding a second semantic call dialect".to_string(),
            })
        }
        MirInstruction::VariantMake {
            dst,
            enum_name,
            variant,
            ..
        } if enum_variant_exists(module_metadata, enum_name, variant) => Some(ThinEntryCandidate {
            block,
            instruction_index,
            value: Some(*dst),
            surface: ThinEntrySurface::VariantMake,
            subject: format!("{}::{}", enum_name, variant),
            preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            current_carrier: ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::AggLocal,
            reason: "variant.make is semantically local aggregate-first; current __NyVariant_* boxing is compat/runtime fallback rather than the preferred physical entry".to_string(),
        }),
        MirInstruction::VariantTag {
            dst,
            enum_name,
            ..
        } if module_metadata.enum_decls.contains_key(enum_name) => {
            Some(ThinEntryCandidate {
                block,
                instruction_index,
                value: Some(*dst),
                surface: ThinEntrySurface::VariantTag,
                subject: enum_name.clone(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::CompatBox,
                value_class: ThinEntryValueClass::InlineI64,
                reason: "variant.tag can stay on a thin internal tag lane; current __NyVariant_* carriers remain compat fallback for VM/runtime and current LLVM lowering".to_string(),
            })
        }
        MirInstruction::VariantProject {
            dst,
            enum_name,
            variant,
            payload_type,
            ..
        } if enum_variant_exists(module_metadata, enum_name, variant) => Some(ThinEntryCandidate {
            block,
            instruction_index,
            value: Some(*dst),
            surface: ThinEntrySurface::VariantProject,
            subject: format!("{}::{}", enum_name, variant),
            preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            current_carrier: ThinEntryCurrentCarrier::CompatBox,
            value_class: value_class_from_declared_type(payload_type.as_ref()),
            reason: "variant.project can stay on a thin internal payload route; current __NyVariant_* carriers remain compat fallback for VM/runtime and current LLVM lowering".to_string(),
        }),
        _ => None,
    }
}

fn known_user_box_name<'a>(
    function: &'a MirFunction,
    module_metadata: &ModuleMetadata,
    base: ValueId,
) -> Option<&'a str> {
    let MirType::Box(box_name) = function.metadata.value_types.get(&base)? else {
        return None;
    };
    if is_synthetic_payload_box(box_name) || is_runtime_sum_box(box_name) {
        return None;
    }
    if module_metadata.user_box_decls.contains_key(box_name)
        || module_metadata.user_box_field_decls.contains_key(box_name)
    {
        Some(box_name.as_str())
    } else {
        None
    }
}

fn known_non_weak_field_decl<'a>(
    module_metadata: &'a ModuleMetadata,
    box_name: &str,
    field: &str,
) -> Option<&'a super::UserBoxFieldDecl> {
    let field_decl = module_metadata
        .user_box_field_decls
        .get(box_name)?
        .iter()
        .find(|decl| decl.name == field)?;
    if field_decl.is_weak {
        return None;
    }
    Some(field_decl)
}

fn enum_variant_exists(module_metadata: &ModuleMetadata, enum_name: &str, variant: &str) -> bool {
    module_metadata
        .enum_decls
        .get(enum_name)
        .map(|decl| decl.variants.iter().any(|item| item.name == variant))
        .unwrap_or(false)
}

fn field_current_carrier(value_class: ThinEntryValueClass) -> ThinEntryCurrentCarrier {
    match value_class {
        ThinEntryValueClass::InlineI64
        | ThinEntryValueClass::InlineBool
        | ThinEntryValueClass::InlineF64 => ThinEntryCurrentCarrier::BackendTyped,
        _ => ThinEntryCurrentCarrier::PublicRuntime,
    }
}

fn value_class_from_declared_type(ty: Option<&MirType>) -> ThinEntryValueClass {
    match ty {
        Some(MirType::Integer) => ThinEntryValueClass::InlineI64,
        Some(MirType::Bool) => ThinEntryValueClass::InlineBool,
        Some(MirType::Float) => ThinEntryValueClass::InlineF64,
        Some(MirType::String) => ThinEntryValueClass::BorrowedText,
        Some(MirType::Box(box_name)) => match type_name_hint_to_mir(Some(box_name.as_str())) {
            Some(MirType::Integer) => ThinEntryValueClass::InlineI64,
            Some(MirType::Bool) => ThinEntryValueClass::InlineBool,
            Some(MirType::Float) => ThinEntryValueClass::InlineF64,
            Some(MirType::String) => ThinEntryValueClass::BorrowedText,
            _ => ThinEntryValueClass::Handle,
        },
        Some(MirType::Void)
        | Some(MirType::Array(_))
        | Some(MirType::Future(_))
        | Some(MirType::WeakRef) => ThinEntryValueClass::Handle,
        Some(MirType::Unknown) | None => ThinEntryValueClass::Unknown,
    }
}

fn type_name_hint_to_mir(raw: Option<&str>) -> Option<MirType> {
    let raw = raw?;
    let lower = raw.to_ascii_lowercase();
    match lower.as_str() {
        "integer" | "int" | "i64" | "integerbox" => Some(MirType::Integer),
        "float" | "f64" | "floatbox" => Some(MirType::Float),
        "bool" | "boolean" | "boolbox" => Some(MirType::Bool),
        "string" | "str" | "stringbox" => Some(MirType::String),
        "void" | "null" | "voidbox" | "nullbox" => Some(MirType::Void),
        _ if looks_like_generic_type_param(raw) => None,
        _ => Some(MirType::Box(raw.to_string())),
    }
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

fn is_synthetic_payload_box(name: &str) -> bool {
    name.starts_with("__NyVariantPayload_")
}

fn is_runtime_sum_box(name: &str) -> bool {
    name.starts_with("__NyVariant_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        EffectMask, FunctionSignature, MirEnumDecl, MirEnumVariantDecl, UserBoxFieldDecl,
    };

    #[test]
    fn refresh_function_collects_thin_entry_candidates_for_known_user_box_and_sum_routes() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let base = ValueId::new(1);
        let field_dst = ValueId::new(2);
        let sum_dst = ValueId::new(3);
        let tag_dst = ValueId::new(4);
        let project_dst = ValueId::new(5);
        function
            .metadata
            .value_types
            .insert(base, MirType::Box("Point".to_string()));
        let entry = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        entry.add_instruction(MirInstruction::FieldGet {
            dst: field_dst,
            base,
            field: "x".to_string(),
            declared_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::VariantMake {
            dst: sum_dst,
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(field_dst),
            payload_type: Some(MirType::Integer),
        });
        entry.add_instruction(MirInstruction::VariantTag {
            dst: tag_dst,
            value: sum_dst,
            enum_name: "Option".to_string(),
        });
        entry.add_instruction(MirInstruction::VariantProject {
            dst: project_dst,
            value: sum_dst,
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload_type: Some(MirType::Integer),
        });

        let mut module = MirModule::new("test".to_string());
        module.functions.insert("test".to_string(), function);
        module
            .metadata
            .user_box_decls
            .insert("Point".to_string(), vec!["x".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Point".to_string(),
            vec![UserBoxFieldDecl {
                name: "x".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: false,
            }],
        );
        module.metadata.enum_decls.insert(
            "Option".to_string(),
            MirEnumDecl {
                type_parameters: vec!["T".to_string()],
                variants: vec![
                    MirEnumVariantDecl {
                        name: "None".to_string(),
                        payload_type_name: None,
                    },
                    MirEnumVariantDecl {
                        name: "Some".to_string(),
                        payload_type_name: Some("T".to_string()),
                    },
                ],
            },
        );

        refresh_module_thin_entry_candidates(&mut module);
        let function = module.get_function("test").expect("function exists");
        let candidates = &function.metadata.thin_entry_candidates;

        assert_eq!(candidates.len(), 4);
        assert!(candidates.iter().any(|candidate| {
            candidate.surface == ThinEntrySurface::UserBoxFieldGet
                && candidate.preferred_entry == ThinEntryPreferredEntry::ThinInternalEntry
                && candidate.current_carrier == ThinEntryCurrentCarrier::BackendTyped
                && candidate.subject == "Point.x"
        }));
        assert!(candidates.iter().any(|candidate| {
            candidate.surface == ThinEntrySurface::VariantMake
                && candidate.current_carrier == ThinEntryCurrentCarrier::CompatBox
                && candidate.value_class == ThinEntryValueClass::AggLocal
                && candidate.subject == "Option::Some"
        }));
        assert!(candidates.iter().any(|candidate| {
            candidate.surface == ThinEntrySurface::VariantTag
                && candidate.current_carrier == ThinEntryCurrentCarrier::CompatBox
                && candidate.value_class == ThinEntryValueClass::InlineI64
                && candidate.subject == "Option"
        }));
        assert!(candidates.iter().any(|candidate| {
            candidate.surface == ThinEntrySurface::VariantProject
                && candidate.value_class == ThinEntryValueClass::InlineI64
                && candidate.subject == "Option::Some"
        }));
    }

    #[test]
    fn refresh_function_normalizes_primitive_box_declared_types() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let base = ValueId::new(1);
        let field_dst = ValueId::new(2);
        function
            .metadata
            .value_types
            .insert(base, MirType::Box("Point".to_string()));
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(MirInstruction::FieldGet {
                dst: field_dst,
                base,
                field: "x".to_string(),
                declared_type: Some(MirType::Box("IntegerBox".to_string())),
            });

        let mut module = MirModule::new("test".to_string());
        module.functions.insert("test".to_string(), function);
        module
            .metadata
            .user_box_decls
            .insert("Point".to_string(), vec!["x".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Point".to_string(),
            vec![UserBoxFieldDecl {
                name: "x".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: false,
            }],
        );

        refresh_module_thin_entry_candidates(&mut module);
        let function = module.get_function("test").expect("function exists");
        let candidates = &function.metadata.thin_entry_candidates;

        assert_eq!(candidates.len(), 1);
        assert!(candidates.iter().any(|candidate| {
            candidate.surface == ThinEntrySurface::UserBoxFieldGet
                && candidate.current_carrier == ThinEntryCurrentCarrier::BackendTyped
                && candidate.value_class == ThinEntryValueClass::InlineI64
                && candidate.subject == "Point.x"
        }));
    }

    #[test]
    fn refresh_function_skips_hidden_enum_payload_boxes() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let base = ValueId::new(1);
        function.metadata.value_types.insert(
            base,
            MirType::Box("__NyVariantPayload_Token_Ident".to_string()),
        );
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(MirInstruction::FieldGet {
                dst: ValueId::new(2),
                base,
                field: "name".to_string(),
                declared_type: Some(MirType::String),
            });

        let mut module = MirModule::new("test".to_string());
        module.functions.insert("test".to_string(), function);
        module.metadata.user_box_decls.insert(
            "__NyVariantPayload_Token_Ident".to_string(),
            vec!["name".to_string()],
        );
        module.metadata.user_box_field_decls.insert(
            "__NyVariantPayload_Token_Ident".to_string(),
            vec![UserBoxFieldDecl {
                name: "name".to_string(),
                declared_type_name: Some("String".to_string()),
                is_weak: false,
            }],
        );

        refresh_module_thin_entry_candidates(&mut module);
        let function = module.get_function("test").expect("function exists");
        assert!(function.metadata.thin_entry_candidates.is_empty());
    }
}
