/*!
 * Boxed runtime ABI plans for native enum values.
 *
 * Canonical MIR keeps using VariantMake / VariantTag / VariantProject. This
 * metadata describes when a sum value needs boxed runtime transport across
 * function or container boundaries, so backends do not infer enum layout from
 * enum names or ad-hoc payload strings.
 */

use crate::mir::{
    BasicBlockId, MirEnumDecl, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::{BTreeMap, HashMap};

pub const BOXED_SUM_ABI_VERSION_V1: &str = "boxed_runtime_v1";
pub const BOXED_SUM_ABI_VERSION_V2: &str = "boxed_runtime_v2";
pub const BOXED_SUM_TAG_STORAGE_I64: &str = "i64";
pub const BOXED_SUM_RUNTIME_TYPE_ID_BASE: u32 = 700_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxedSumPayloadStorage {
    None,
    I64,
    Handle,
}

impl BoxedSumPayloadStorage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::I64 => "i64",
            Self::Handle => "handle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxedSumAbiVariantPlan {
    pub name: String,
    pub tag: u32,
    pub payload_storage: BoxedSumPayloadStorage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxedSumAbiPlanV1 {
    pub plan_id: u32,
    pub shape_key: String,
    pub enum_name: String,
    pub runtime_type_id: u32,
    pub runtime_box_name: String,
    pub tag_storage: &'static str,
    pub variants: Vec<BoxedSumAbiVariantPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxedSumSitePlan {
    pub plan_id: u32,
    pub payload_storage: Option<BoxedSumPayloadStorage>,
}

pub fn refresh_module_boxed_sum_abi_plans(module: &mut MirModule) {
    module.metadata.boxed_sum_abi_plans = build_boxed_sum_abi_plans(module);
}

pub fn build_boxed_sum_abi_plans(module: &MirModule) -> Vec<BoxedSumAbiPlanV1> {
    let mut plans = std::collections::BTreeMap::<String, BoxedSumAbiPlanV1>::new();
    for (enum_name, decl) in &module.metadata.enum_decls {
        if let Some(plan) = build_enum_plan(enum_name, decl) {
            plans.insert(plan.shape_key.clone(), plan);
        }
    }
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                if let Some(plan) = build_site_plan(module, inst) {
                    plans.insert(plan.shape_key.clone(), plan);
                }
            }
        }
    }
    plans
        .into_values()
        .enumerate()
        .map(|(index, mut plan)| {
            let plan_id = index as u32;
            plan.plan_id = plan_id;
            plan.runtime_type_id = BOXED_SUM_RUNTIME_TYPE_ID_BASE + plan_id;
            plan
        })
        .collect()
}

pub fn find_boxed_sum_site_plan<'a>(
    plans: &'a [BoxedSumAbiPlanV1],
    enum_name: &str,
    tag: u32,
    payload_type: Option<&MirType>,
) -> Option<&'a BoxedSumAbiPlanV1> {
    let storage = payload_type
        .map(payload_storage_from_mir_type)
        .unwrap_or(Some(BoxedSumPayloadStorage::None))?;
    find_boxed_sum_site_plan_for_storage(plans, enum_name, tag, storage)
}

pub fn find_boxed_sum_site_plan_for_storage<'a>(
    plans: &'a [BoxedSumAbiPlanV1],
    enum_name: &str,
    tag: u32,
    storage: BoxedSumPayloadStorage,
) -> Option<&'a BoxedSumAbiPlanV1> {
    let mut matches = plans.iter().filter(|plan| {
        plan.enum_name == enum_name
            && plan
                .variants
                .get(tag as usize)
                .is_some_and(|variant| variant.tag == tag && variant.payload_storage == storage)
    });
    let first = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(first)
}

pub fn build_function_boxed_sum_site_plan_map(
    function: &MirFunction,
    plans: &[BoxedSumAbiPlanV1],
) -> BTreeMap<(BasicBlockId, usize), BoxedSumSitePlan> {
    let mut site_plans = BTreeMap::new();
    let mut value_plans: HashMap<ValueId, BoxedSumSitePlan> = HashMap::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in &block_ids {
        let Some(block) = function.blocks.get(block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            match inst {
                MirInstruction::VariantMake {
                    dst,
                    enum_name,
                    tag,
                    payload,
                    payload_type,
                    ..
                } => {
                    if let Some(site) = site_plan_for_variant_make(
                        plans,
                        enum_name,
                        *tag,
                        payload.is_some(),
                        payload_type.as_ref(),
                    ) {
                        site_plans.insert((*block_id, instruction_index), site.clone());
                        value_plans.insert(*dst, site);
                    }
                }
                MirInstruction::Copy { dst, src } => {
                    if let Some(site) = value_plans.get(src).cloned() {
                        value_plans.insert(*dst, site);
                    }
                }
                MirInstruction::VariantProject {
                    enum_name,
                    tag,
                    payload_type,
                    ..
                } => {
                    if let Some(site) =
                        site_plan_for_variant_site(plans, enum_name, *tag, payload_type.as_ref())
                    {
                        site_plans.insert((*block_id, instruction_index), site);
                    }
                }
                MirInstruction::VariantTag {
                    value, enum_name, ..
                } => {
                    if let Some(site) = value_plans.get(value).filter(|site| {
                        plans
                            .get(site.plan_id as usize)
                            .is_some_and(|plan| plan.enum_name == *enum_name)
                    }) {
                        site_plans.insert(
                            (*block_id, instruction_index),
                            BoxedSumSitePlan {
                                plan_id: site.plan_id,
                                payload_storage: None,
                            },
                        );
                    }
                }
                _ => {}
            }
        }
    }

    site_plans
}

fn site_plan_for_variant_site(
    plans: &[BoxedSumAbiPlanV1],
    enum_name: &str,
    tag: u32,
    payload_type: Option<&MirType>,
) -> Option<BoxedSumSitePlan> {
    let plan = find_boxed_sum_site_plan(plans, enum_name, tag, payload_type)?;
    let payload_storage = plan
        .variants
        .get(tag as usize)
        .map(|variant| variant.payload_storage.clone());
    Some(BoxedSumSitePlan {
        plan_id: plan.plan_id,
        payload_storage,
    })
}

fn site_plan_for_variant_make(
    plans: &[BoxedSumAbiPlanV1],
    enum_name: &str,
    tag: u32,
    has_payload: bool,
    payload_type: Option<&MirType>,
) -> Option<BoxedSumSitePlan> {
    let plan = if has_payload {
        find_boxed_sum_site_plan(plans, enum_name, tag, Some(payload_type?))?
    } else {
        let storage = BoxedSumPayloadStorage::None;
        if let Some(payload_type) = payload_type {
            plans
                .iter()
                .find(|plan| {
                    plan.enum_name == enum_name
                        && plan.variants.get(tag as usize).is_some_and(|variant| {
                            variant.tag == tag && variant.payload_storage == storage
                        })
                        && plan.variants.iter().any(|variant| {
                            variant.payload_storage
                                == payload_storage_from_mir_type(payload_type)
                                    .unwrap_or(BoxedSumPayloadStorage::None)
                        })
                })
                .or_else(|| find_boxed_sum_site_plan_for_storage(plans, enum_name, tag, storage))?
        } else {
            find_boxed_sum_site_plan_for_storage(plans, enum_name, tag, storage)?
        }
    };
    let payload_storage = plan
        .variants
        .get(tag as usize)
        .map(|variant| variant.payload_storage.clone());
    Some(BoxedSumSitePlan {
        plan_id: plan.plan_id,
        payload_storage,
    })
}

fn build_enum_plan(enum_name: &str, decl: &MirEnumDecl) -> Option<BoxedSumAbiPlanV1> {
    if decl.variants.is_empty() {
        return None;
    }
    let variants = decl
        .variants
        .iter()
        .enumerate()
        .map(|(tag, variant)| BoxedSumAbiVariantPlan {
            name: variant.name.clone(),
            tag: tag as u32,
            payload_storage: variant
                .payload_type_name
                .as_deref()
                .and_then(payload_storage_from_decl_type)
                .unwrap_or_else(|| {
                    if variant.payload_type_name.is_some() {
                        BoxedSumPayloadStorage::Handle
                    } else {
                        BoxedSumPayloadStorage::None
                    }
                }),
        })
        .collect::<Vec<_>>();
    let shape_key = shape_key(enum_name, &variants);
    Some(BoxedSumAbiPlanV1 {
        plan_id: 0,
        shape_key,
        enum_name: enum_name.to_string(),
        runtime_type_id: 0,
        runtime_box_name: runtime_variant_box_name(enum_name),
        tag_storage: BOXED_SUM_TAG_STORAGE_I64,
        variants,
    })
}

fn build_site_plan(module: &MirModule, inst: &MirInstruction) -> Option<BoxedSumAbiPlanV1> {
    let (enum_name, tag, payload_type) = match inst {
        MirInstruction::VariantMake {
            enum_name,
            tag,
            payload,
            payload_type,
            ..
        } => {
            return build_variant_make_site_plan(
                module,
                enum_name,
                *tag,
                payload.is_some(),
                payload_type.as_ref(),
            );
        }
        MirInstruction::VariantProject {
            enum_name,
            tag,
            payload_type,
            ..
        } => (enum_name, *tag, payload_type.as_ref()),
        _ => return None,
    };
    let storage = payload_storage_from_mir_type(payload_type?)?;
    build_site_plan_with_storage(module, enum_name, tag, storage)
}

fn build_variant_make_site_plan(
    module: &MirModule,
    enum_name: &str,
    tag: u32,
    has_payload: bool,
    payload_type: Option<&MirType>,
) -> Option<BoxedSumAbiPlanV1> {
    let decl = module.metadata.enum_decls.get(enum_name)?;
    if !has_payload {
        if let Some(payload_type) = payload_type {
            if let Some(plan) = build_single_param_instantiated_plan(enum_name, decl, payload_type)
            {
                let variant = plan.variants.get(tag as usize)?;
                if variant.tag == tag && variant.payload_storage == BoxedSumPayloadStorage::None {
                    return Some(plan);
                }
            }
        }
        return build_site_plan_with_storage(module, enum_name, tag, BoxedSumPayloadStorage::None);
    }

    let storage = payload_storage_from_mir_type(payload_type?)?;
    build_site_plan_with_storage(module, enum_name, tag, storage)
}

fn build_single_param_instantiated_plan(
    enum_name: &str,
    decl: &MirEnumDecl,
    payload_type: &MirType,
) -> Option<BoxedSumAbiPlanV1> {
    let [param] = decl.type_parameters.as_slice() else {
        return None;
    };
    let storage = payload_storage_from_mir_type(payload_type)?;
    let mut plan = build_enum_plan(enum_name, decl)?;
    let mut used_hint = false;
    for (index, variant_decl) in decl.variants.iter().enumerate() {
        if variant_decl.payload_type_name.as_deref() == Some(param.as_str()) {
            let variant = plan.variants.get_mut(index)?;
            variant.payload_storage = storage.clone();
            used_hint = true;
        }
    }
    if !used_hint {
        return None;
    }
    plan.shape_key = shape_key(enum_name, &plan.variants);
    Some(plan)
}

fn build_site_plan_with_storage(
    module: &MirModule,
    enum_name: &str,
    tag: u32,
    storage: BoxedSumPayloadStorage,
) -> Option<BoxedSumAbiPlanV1> {
    let decl = module.metadata.enum_decls.get(enum_name)?;
    let mut plan = build_enum_plan(enum_name, decl)?;
    let variant = plan.variants.get_mut(tag as usize)?;
    if variant.tag != tag {
        return None;
    }
    variant.payload_storage = storage;
    plan.shape_key = shape_key(enum_name, &plan.variants);
    Some(plan)
}

fn payload_storage_from_decl_type(raw: &str) -> Option<BoxedSumPayloadStorage> {
    match raw {
        "i64" | "Integer" | "bool" | "Bool" => Some(BoxedSumPayloadStorage::I64),
        "f64" | "Float" => None,
        _ if looks_like_generic_type_param(raw) => None,
        _ => Some(BoxedSumPayloadStorage::Handle),
    }
}

fn payload_storage_from_mir_type(ty: &MirType) -> Option<BoxedSumPayloadStorage> {
    match ty {
        MirType::Integer | MirType::Bool => Some(BoxedSumPayloadStorage::I64),
        MirType::Float => None,
        MirType::Void => Some(BoxedSumPayloadStorage::None),
        MirType::String
        | MirType::Box(_)
        | MirType::Array(_)
        | MirType::Future(_)
        | MirType::WeakRef => Some(BoxedSumPayloadStorage::Handle),
        MirType::Unknown => None,
    }
}

fn shape_key(enum_name: &str, variants: &[BoxedSumAbiVariantPlan]) -> String {
    let payloads = variants
        .iter()
        .map(|variant| format!("{}:{}", variant.tag, variant.payload_storage.as_str()))
        .collect::<Vec<_>>()
        .join(",");
    format!("{}|{}", enum_name, payloads)
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

pub fn runtime_variant_box_name(enum_name: &str) -> String {
    format!("__NyVariant_{}", enum_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{EffectMask, FunctionSignature, MirEnumDecl, MirEnumVariantDecl, MirModule};

    #[test]
    fn build_boxed_sum_abi_plans_derives_payloadless_enum_rows() {
        let mut module = MirModule::new("boxed_sum_abi_probe".to_string());
        module.metadata.enum_decls.insert(
            "ProbeKind".to_string(),
            MirEnumDecl {
                type_parameters: vec![],
                variants: vec![
                    MirEnumVariantDecl {
                        name: "Alpha".to_string(),
                        payload_type_name: None,
                    },
                    MirEnumVariantDecl {
                        name: "Beta".to_string(),
                        payload_type_name: None,
                    },
                ],
            },
        );

        let plans = build_boxed_sum_abi_plans(&module);
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].plan_id, 0);
        assert_eq!(plans[0].shape_key, "ProbeKind|0:none,1:none");
        assert_eq!(plans[0].runtime_type_id, BOXED_SUM_RUNTIME_TYPE_ID_BASE);
        assert_eq!(plans[0].enum_name, "ProbeKind");
        assert_eq!(plans[0].runtime_box_name, "__NyVariant_ProbeKind");
        assert_eq!(plans[0].tag_storage, BOXED_SUM_TAG_STORAGE_I64);
        assert_eq!(plans[0].variants[1].name, "Beta");
        assert_eq!(plans[0].variants[1].tag, 1);
        assert_eq!(
            plans[0].variants[1].payload_storage,
            BoxedSumPayloadStorage::None
        );
    }

    #[test]
    fn build_boxed_sum_abi_plans_marks_payload_variants_as_handle_storage() {
        let mut module = MirModule::new("boxed_sum_abi_payload_probe".to_string());
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

        let plans = build_boxed_sum_abi_plans(&module);
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].enum_name, "Option");
        assert_eq!(
            plans[0].variants[0].payload_storage,
            BoxedSumPayloadStorage::None
        );
        assert_eq!(
            plans[0].variants[1].payload_storage,
            BoxedSumPayloadStorage::Handle
        );
    }

    #[test]
    fn build_boxed_sum_abi_plans_adds_concrete_i64_shape_from_sites() {
        use crate::mir::{BasicBlock, BasicBlockId, MirFunction};

        let mut module = MirModule::new("boxed_sum_abi_i64_probe".to_string());
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
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::VariantMake {
            dst: crate::mir::ValueId::new(2),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(crate::mir::ValueId::new(1)),
            payload_type: Some(MirType::Integer),
        });
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "probe".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.blocks.insert(BasicBlockId::new(0), block);
        module.functions.insert("probe".to_string(), function);

        let plans = build_boxed_sum_abi_plans(&module);
        assert_eq!(plans.len(), 2);
        assert!(plans
            .iter()
            .any(|plan| plan.shape_key == "Option|0:none,1:handle"));
        assert!(plans
            .iter()
            .any(|plan| plan.shape_key == "Option|0:none,1:i64"));
    }

    #[test]
    fn function_site_plan_map_annotates_local_variant_tag_source() {
        use crate::mir::{BasicBlock, BasicBlockId, MirFunction};

        let mut module = MirModule::new("boxed_sum_tag_site_probe".to_string());
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
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::VariantMake {
            dst: crate::mir::ValueId::new(2),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(crate::mir::ValueId::new(1)),
            payload_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::VariantTag {
            dst: crate::mir::ValueId::new(3),
            value: crate::mir::ValueId::new(2),
            enum_name: "Option".to_string(),
        });
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "probe".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.blocks.insert(BasicBlockId::new(0), block);
        module.functions.insert("probe".to_string(), function);

        let plans = build_boxed_sum_abi_plans(&module);
        let function = module.functions.get("probe").unwrap();
        let site_map = build_function_boxed_sum_site_plan_map(function, &plans);
        let tag_site = site_map.get(&(BasicBlockId::new(0), 1)).unwrap();
        let plan = plans.get(tag_site.plan_id as usize).unwrap();

        assert_eq!(plan.shape_key, "Option|0:none,1:i64");
        assert_eq!(tag_site.payload_storage, None);
    }

    #[test]
    fn variant_make_none_uses_payload_type_as_instantiation_hint() {
        use crate::mir::{BasicBlock, BasicBlockId, MirFunction};

        let mut module = MirModule::new("boxed_sum_none_i64_probe".to_string());
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
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::VariantMake {
            dst: crate::mir::ValueId::new(2),
            enum_name: "Option".to_string(),
            variant: "None".to_string(),
            tag: 0,
            payload: None,
            payload_type: Some(MirType::Integer),
        });
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "probe".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.blocks.insert(BasicBlockId::new(0), block);
        module.functions.insert("probe".to_string(), function);

        let plans = build_boxed_sum_abi_plans(&module);
        let function = module.functions.get("probe").unwrap();
        let site_map = build_function_boxed_sum_site_plan_map(function, &plans);
        let make_site = site_map.get(&(BasicBlockId::new(0), 0)).unwrap();
        let plan = plans.get(make_site.plan_id as usize).unwrap();

        assert_eq!(plan.shape_key, "Option|0:none,1:i64");
        assert_eq!(
            make_site.payload_storage,
            Some(BoxedSumPayloadStorage::None)
        );
    }

    #[test]
    fn variant_make_some_payload_zero_is_still_payload_present() {
        use crate::mir::{BasicBlock, BasicBlockId, MirFunction};

        let mut module = MirModule::new("boxed_sum_some_zero_payload_probe".to_string());
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
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::VariantMake {
            dst: crate::mir::ValueId::new(2),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(crate::mir::ValueId::new(0)),
            payload_type: Some(MirType::Integer),
        });
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "probe".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.blocks.insert(BasicBlockId::new(0), block);
        module.functions.insert("probe".to_string(), function);

        let plans = build_boxed_sum_abi_plans(&module);
        let function = module.functions.get("probe").unwrap();
        let site_map = build_function_boxed_sum_site_plan_map(function, &plans);
        let make_site = site_map.get(&(BasicBlockId::new(0), 0)).unwrap();
        let plan = plans.get(make_site.plan_id as usize).unwrap();

        assert_eq!(plan.shape_key, "Option|0:none,1:i64");
        assert_eq!(make_site.payload_storage, Some(BoxedSumPayloadStorage::I64));
    }
}
