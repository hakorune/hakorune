//! ArrayBox element-result publication.
//!
//! This box owns only receiver-local `Array<T>` facts. It does not infer
//! global generics and it keeps mixed or unknown element arrays as `Unknown`.

use crate::mir::builder::MirBuilder;
use crate::mir::{Callee, MirType, ValueId};

pub(in crate::mir::builder) fn record_array_literal_elements(
    builder: &mut MirBuilder,
    receiver: ValueId,
    element_types: &[Option<MirType>],
) {
    if element_types.is_empty() {
        return;
    }
    let Some(element_type) = homogeneous_element_type(element_types) else {
        set_array_type(builder, receiver, MirType::Unknown);
        return;
    };
    set_array_type(builder, receiver, element_type);
}

pub(in crate::mir::builder) fn observe_array_write_call(
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
    if box_name != "ArrayBox" {
        return;
    }

    let user_args = user_args(*receiver, args_with_optional_receiver);
    let Some(method_id) =
        crate::boxes::array::ArrayMethodId::from_name_and_arity(method, user_args.len())
    else {
        return;
    };
    let value_arg = match method_id {
        crate::boxes::array::ArrayMethodId::Push => user_args.first().copied(),
        crate::boxes::array::ArrayMethodId::Set | crate::boxes::array::ArrayMethodId::Insert => {
            user_args.get(1).copied()
        }
        _ => None,
    };

    let Some(value_arg) = value_arg else {
        return;
    };
    let next_type = concrete_value_type(builder, value_arg);
    merge_array_element_type(builder, *receiver, next_type);
}

pub(in crate::mir::builder) fn annotate_array_element_result(
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
    if box_name != "ArrayBox" {
        return;
    }

    let user_args = user_args(*receiver, args_with_optional_receiver);
    let Some(method_id) =
        crate::boxes::array::ArrayMethodId::from_name_and_arity(method, user_args.len())
    else {
        return;
    };
    if !matches!(
        method_id,
        crate::boxes::array::ArrayMethodId::Get
            | crate::boxes::array::ArrayMethodId::Pop
            | crate::boxes::array::ArrayMethodId::Remove
    ) {
        return;
    }

    let Some(element_type) = array_element_type(builder, *receiver) else {
        return;
    };
    if is_publishable_element_type(&element_type) {
        builder
            .type_ctx
            .value_types
            .insert(dst, element_type.clone());
        builder
            .comp_ctx
            .type_registry
            .record_type(dst, element_type.clone());
        if let MirType::Box(box_name) = element_type {
            builder
                .type_ctx
                .value_origin_newbox
                .insert(dst, box_name.clone());
            builder
                .comp_ctx
                .type_registry
                .record_origin(dst, box_name, "array_element");
        }
    }
}

fn user_args(receiver: ValueId, args: &[ValueId]) -> &[ValueId] {
    if args.first() == Some(&receiver) {
        &args[1..]
    } else {
        args
    }
}

fn homogeneous_element_type(element_types: &[Option<MirType>]) -> Option<MirType> {
    let first = element_types.first()?.as_ref()?;
    if !is_publishable_element_type(first) {
        return None;
    }
    if element_types
        .iter()
        .all(|candidate| candidate.as_ref() == Some(first))
    {
        Some(first.clone())
    } else {
        None
    }
}

fn concrete_value_type(builder: &MirBuilder, value: ValueId) -> Option<MirType> {
    if let Some(ty) = builder.type_ctx.value_types.get(&value) {
        if is_publishable_element_type(ty) {
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

fn merge_array_element_type(
    builder: &mut MirBuilder,
    receiver: ValueId,
    next_type: Option<MirType>,
) {
    let Some(next_type) = next_type else {
        set_array_type(builder, receiver, MirType::Unknown);
        return;
    };

    match builder.type_ctx.value_types.get(&receiver).cloned() {
        Some(MirType::Array(existing)) if *existing == next_type => {}
        Some(MirType::Array(existing)) if is_publishable_element_type(&existing) => {
            set_array_type(builder, receiver, MirType::Unknown);
        }
        Some(MirType::Array(_)) => {}
        Some(MirType::Box(box_name)) if box_name == "ArrayBox" => {
            set_array_type(builder, receiver, next_type);
        }
        Some(_) => {}
        None if builder
            .type_ctx
            .value_origin_newbox
            .get(&receiver)
            .map(|name| name == "ArrayBox")
            .unwrap_or(false) =>
        {
            set_array_type(builder, receiver, next_type);
        }
        None => {}
    }
}

fn array_element_type(builder: &MirBuilder, receiver: ValueId) -> Option<MirType> {
    match builder.type_ctx.value_types.get(&receiver) {
        Some(MirType::Array(element_type)) => Some((**element_type).clone()),
        _ => None,
    }
}

fn set_array_type(builder: &mut MirBuilder, receiver: ValueId, element_type: MirType) {
    let array_type = MirType::Array(Box::new(element_type));
    builder
        .type_ctx
        .value_types
        .insert(receiver, array_type.clone());
    builder
        .comp_ctx
        .type_registry
        .record_type(receiver, array_type);
}

fn is_publishable_element_type(ty: &MirType) -> bool {
    !matches!(ty, MirType::Unknown | MirType::Void)
}
