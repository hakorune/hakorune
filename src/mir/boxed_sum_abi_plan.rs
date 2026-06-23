/*!
 * Boxed runtime ABI plans for native enum values.
 *
 * Canonical MIR keeps using VariantMake / VariantTag / VariantProject. This
 * metadata describes when a sum value needs boxed runtime transport across
 * function or container boundaries, so backends do not infer enum layout from
 * enum names or ad-hoc payload strings.
 */

use crate::mir::{MirEnumDecl, MirModule};

pub const BOXED_SUM_ABI_VERSION_V1: &str = "boxed_runtime_v1";
pub const BOXED_SUM_TAG_STORAGE_I64: &str = "i64";
pub const BOXED_SUM_RUNTIME_TYPE_ID_BASE: u32 = 700_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxedSumPayloadStorage {
    None,
    Handle,
}

impl BoxedSumPayloadStorage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
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
    pub enum_name: String,
    pub runtime_type_id: u32,
    pub runtime_box_name: String,
    pub tag_storage: &'static str,
    pub variants: Vec<BoxedSumAbiVariantPlan>,
}

pub fn refresh_module_boxed_sum_abi_plans(module: &mut MirModule) {
    module.metadata.boxed_sum_abi_plans = build_boxed_sum_abi_plans(module);
}

pub fn build_boxed_sum_abi_plans(module: &MirModule) -> Vec<BoxedSumAbiPlanV1> {
    module
        .metadata
        .enum_decls
        .iter()
        .filter_map(|(enum_name, decl)| build_unit_enum_plan(enum_name, decl))
        .enumerate()
        .map(|(index, mut plan)| {
            let plan_id = index as u32;
            plan.plan_id = plan_id;
            plan.runtime_type_id = BOXED_SUM_RUNTIME_TYPE_ID_BASE + plan_id;
            plan
        })
        .collect()
}

fn build_unit_enum_plan(enum_name: &str, decl: &MirEnumDecl) -> Option<BoxedSumAbiPlanV1> {
    if decl
        .variants
        .iter()
        .any(|variant| variant.payload_type_name.is_some())
    {
        return None;
    }

    Some(BoxedSumAbiPlanV1 {
        plan_id: 0,
        enum_name: enum_name.to_string(),
        runtime_type_id: 0,
        runtime_box_name: runtime_variant_box_name(enum_name),
        tag_storage: BOXED_SUM_TAG_STORAGE_I64,
        variants: decl
            .variants
            .iter()
            .enumerate()
            .map(|(tag, variant)| BoxedSumAbiVariantPlan {
                name: variant.name.clone(),
                tag: tag as u32,
                payload_storage: BoxedSumPayloadStorage::None,
            })
            .collect(),
    })
}

pub fn runtime_variant_box_name(enum_name: &str) -> String {
    format!("__NyVariant_{}", enum_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{MirEnumDecl, MirEnumVariantDecl, MirModule};

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
    fn build_boxed_sum_abi_plans_does_not_guess_payload_storage() {
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

        assert!(build_boxed_sum_abi_plans(&module).is_empty());
    }
}
