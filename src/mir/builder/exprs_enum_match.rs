//! Retained enum source preflight; raw enum MIR emission is retired.
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::enum_variant_arguments_are_statically_nullish_v1;
use crate::mir::MirType;

pub(in crate::mir::builder) struct PreparedRawEnumVariantHeaderV1 {
    tag: u32,
    declared_payload_type: Option<MirType>,
    _seal: PreparedRawEnumVariantHeaderSealV1,
}

struct PreparedRawEnumVariantHeaderSealV1;

pub(in crate::mir::builder) fn prepare_raw_enum_variant_header_v1(
    builder: &super::MirBuilder,
    enum_name: &str,
    variant_name: &str,
    arguments: &[ASTNode],
) -> Result<Option<PreparedRawEnumVariantHeaderV1>, String> {
    let Some(resolved) = builder
        .comp_ctx
        .resolve_enum_variant(enum_name, variant_name)
    else {
        return Ok(None);
    };
    if resolved.decl.requires_compat_payload_box() {
        return Err(format!(
            "[freeze:contract][mir_builder/enum_ctor] {}::{} record/tuple payload lowering is outside direct-MIR MVP",
            enum_name, variant_name
        ));
    }
    let expected_arity = resolved.decl.payload_arity();
    if arguments.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][mir_builder/enum_ctor] {}::{} expects {} arg(s), got {}",
            enum_name,
            variant_name,
            expected_arity,
            arguments.len()
        ));
    }
    if crate::semantics::option_contract::requires_non_nullish_payload(enum_name, variant_name)
        && enum_variant_arguments_are_statically_nullish_v1(arguments)
    {
        return Err(crate::semantics::option_contract::nullish_payload_error(
            "mir_builder/enum_ctor",
        ));
    }
    Ok(Some(PreparedRawEnumVariantHeaderV1 {
        tag: resolved.tag,
        declared_payload_type: payload_mir_type(resolved.decl.payload_type_name.as_deref()),
        _seal: PreparedRawEnumVariantHeaderSealV1,
    }))
}

fn payload_mir_type(raw: Option<&str>) -> Option<MirType> {
    let raw = raw?;
    if looks_like_generic_type_param(raw) {
        return None;
    }
    Some(super::MirBuilder::parse_type_name_to_mir(raw))
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}
