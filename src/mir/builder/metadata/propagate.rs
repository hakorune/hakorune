//! MetadataPropagationBox — MIR のメタデータ（型/起源）の伝播
//! 仕様不変・小粒。各所のコピペを置換するための薄い関数郡。
//!
//! 🎯 箱理論: TypeRegistryBox 統合対応
//! NYASH_USE_TYPE_REGISTRY=1 で TypeRegistry 経由に切り替え（段階的移行）

use crate::mir::builder::observe::types as type_trace;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};

/// src から dst へ builder 内メタデータ（value_types / value_origin_newbox）を伝播する。
/// 🎯 TypeRegistry 経由モード対応（NYASH_USE_TYPE_REGISTRY=1）
#[inline]
pub fn propagate(builder: &mut MirBuilder, src: ValueId, dst: ValueId) {
    let use_registry = crate::config::env::builder_use_type_registry();

    if use_registry {
        // 🎯 新: TypeRegistry 経由（トレース可能）
        builder.comp_ctx.type_registry.propagate(src, dst);
    } else {
        // 従来: 直接アクセス（後方互換性）
        if let Some(t) = builder.type_ctx.value_types.get(&src).cloned() {
            builder.type_ctx.value_types.insert(dst, t);
        }
        if let Some(cls) = builder.type_ctx.value_origin_newbox.get(&src).cloned() {
            builder.type_ctx.value_origin_newbox.insert(dst, cls);
        }
    }
    if let Some(text) = builder.type_ctx.string_literals.get(&src).cloned() {
        builder.type_ctx.string_literals.insert(dst, text);
    }
    if let Some(map_value_type) = builder.type_ctx.map_value_types.get(&src).cloned() {
        builder.type_ctx.map_value_types.insert(dst, map_value_type);
    }
    let literal_facts: Vec<(String, MirType)> = builder
        .type_ctx
        .map_literal_value_types
        .iter()
        .filter(|((value_id, _), _)| *value_id == src)
        .map(|((_, key), ty)| (key.clone(), ty.clone()))
        .collect();
    for (key, ty) in literal_facts {
        builder
            .type_ctx
            .map_literal_value_types
            .insert((dst, key), ty);
    }
    type_trace::propagate("meta", src, dst);
}

/// dst に型注釈を明示的に設定し、必要ならば起源情報を消去/維持する。
/// 🎯 TypeRegistry 経由モード対応（NYASH_USE_TYPE_REGISTRY=1）
#[inline]
#[allow(dead_code)]
pub fn propagate_with_override(builder: &mut MirBuilder, dst: ValueId, ty: MirType) {
    let use_registry = crate::config::env::builder_use_type_registry();

    // clone once for dual paths + trace
    let ty_clone = ty.clone();
    if use_registry {
        // 🎯 新: TypeRegistry 経由
        builder.comp_ctx.type_registry.record_type(dst, ty);
    } else {
        // 従来: 直接アクセス
        builder.type_ctx.value_types.insert(dst, ty);
    }
    type_trace::ty("override", dst, &ty_clone);
}
