//! MapBox existing-key result publication.
//!
//! This box mirrors the small Array<T> publication slice conservatively:
//! - publish only for homogeneous receiver-local MapBox value facts
//! - require a tracked literal key so missing-key reads stay Unknown
//! - invalidate facts on mixed, unknown, or destructive writes

use crate::mir::builder::MirBuilder;
use crate::mir::{Callee, MirType, ValueId};

pub(in crate::mir::builder) fn observe_map_write_call(
    builder: &mut MirBuilder,
    callee: &Callee,
    args_with_optional_receiver: &[ValueId],
) {
    let Callee::Method {
        box_name,
        method,
        receiver: Some(receiver),
        ..
    } = callee
    else {
        return;
    };
    if box_name != "MapBox" {
        return;
    }

    let user_args = user_args(*receiver, args_with_optional_receiver);
    let Some(method_id) = crate::boxes::MapMethodId::from_name_and_arity(method, user_args.len())
    else {
        return;
    };

    match method_id {
        crate::boxes::MapMethodId::Set => {
            let key_arg = user_args.first().copied();
            let value_arg = user_args.get(1).copied();
            let key_literal = key_arg.and_then(|value| string_literal(builder, value));
            let next_type = value_arg.and_then(|value| concrete_value_type(builder, value));
            merge_map_value_fact(builder, *receiver, key_literal, next_type);
        }
        crate::boxes::MapMethodId::Delete | crate::boxes::MapMethodId::Clear => {
            clear_map_facts(builder, *receiver);
        }
        _ => {}
    }
}

pub(in crate::mir::builder) fn annotate_map_get_result(
    builder: &mut MirBuilder,
    dst: ValueId,
    callee: &Callee,
    args_with_optional_receiver: &[ValueId],
) {
    let Callee::Method {
        box_name,
        method,
        receiver: Some(receiver),
        ..
    } = callee
    else {
        return;
    };
    if box_name != "MapBox" {
        return;
    }

    let user_args = user_args(*receiver, args_with_optional_receiver);
    let Some(method_id) = crate::boxes::MapMethodId::from_name_and_arity(method, user_args.len())
    else {
        return;
    };
    if method_id != crate::boxes::MapMethodId::Get {
        return;
    }

    let Some(key_arg) = user_args.first().copied() else {
        return;
    };
    let Some(key_literal) = string_literal(builder, key_arg) else {
        return;
    };
    let Some(receiver_value_type) = builder.type_ctx.map_value_types.get(receiver) else {
        return;
    };
    if !is_publishable_value_type(receiver_value_type) {
        return;
    }
    let Some(value_type) = builder
        .type_ctx
        .map_literal_value_types
        .get(&(*receiver, key_literal))
        .cloned()
    else {
        return;
    };
    if !is_publishable_value_type(&value_type) {
        return;
    }

    builder.type_ctx.value_types.insert(dst, value_type.clone());
    builder
        .comp_ctx
        .type_registry
        .record_type(dst, value_type.clone());
    if let MirType::Box(box_name) = value_type {
        builder
            .type_ctx
            .value_origin_newbox
            .insert(dst, box_name.clone());
        builder
            .comp_ctx
            .type_registry
            .record_origin(dst, box_name, "map_value");
    }
}

fn user_args(receiver: ValueId, args: &[ValueId]) -> &[ValueId] {
    if args.first() == Some(&receiver) {
        &args[1..]
    } else {
        args
    }
}

fn string_literal(builder: &MirBuilder, value: ValueId) -> Option<String> {
    builder.type_ctx.string_literals.get(&value).cloned()
}

fn concrete_value_type(builder: &MirBuilder, value: ValueId) -> Option<MirType> {
    if let Some(ty) = builder.type_ctx.value_types.get(&value) {
        if is_publishable_value_type(ty) {
            return Some(ty.clone());
        }
        return None;
    }
    builder
        .type_ctx
        .value_origin_newbox
        .get(&value)
        .map(|box_name| MirType::Box(box_name.clone()))
}

fn merge_map_value_fact(
    builder: &mut MirBuilder,
    receiver: ValueId,
    literal_key: Option<String>,
    next_type: Option<MirType>,
) {
    let Some(literal_key) = literal_key else {
        mark_map_value_unknown(builder, receiver);
        return;
    };
    let Some(next_type) = next_type else {
        mark_map_value_unknown(builder, receiver);
        return;
    };

    if !receiver_supports_local_map_facts(builder, receiver) {
        return;
    }

    match builder.type_ctx.map_value_types.get(&receiver).cloned() {
        Some(existing) if existing == next_type => {}
        Some(existing) if is_publishable_value_type(&existing) => {
            mark_map_value_unknown(builder, receiver);
            return;
        }
        Some(_) => return,
        None => {
            builder
                .type_ctx
                .map_value_types
                .insert(receiver, next_type.clone());
        }
    }

    builder
        .type_ctx
        .map_literal_value_types
        .insert((receiver, literal_key), next_type);
}

fn receiver_supports_local_map_facts(builder: &MirBuilder, receiver: ValueId) -> bool {
    match builder.type_ctx.value_types.get(&receiver) {
        Some(MirType::Box(box_name)) if box_name == "MapBox" => true,
        Some(_) => false,
        None => builder
            .type_ctx
            .value_origin_newbox
            .get(&receiver)
            .map(|name| name == "MapBox")
            .unwrap_or(false),
    }
}

fn mark_map_value_unknown(builder: &mut MirBuilder, receiver: ValueId) {
    builder
        .type_ctx
        .map_value_types
        .insert(receiver, MirType::Unknown);
    builder
        .type_ctx
        .map_literal_value_types
        .retain(|(value_id, _), _| *value_id != receiver);
}

fn clear_map_facts(builder: &mut MirBuilder, receiver: ValueId) {
    builder.type_ctx.map_value_types.remove(&receiver);
    builder
        .type_ctx
        .map_literal_value_types
        .retain(|(value_id, _), _| *value_id != receiver);
}

fn is_publishable_value_type(ty: &MirType) -> bool {
    !matches!(ty, MirType::Unknown | MirType::Void)
}
