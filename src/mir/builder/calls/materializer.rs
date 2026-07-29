/*!
 * CallMaterializerBox - Call前処理・準備専用箱
 *
 * 箱理論の実践:
 * - 箱にする: Call発行前の前処理を1箱に集約
 * - 境界を作る: 未解決Global callの追加解決・receiver実体化を分離
 * - 状態最小: MirBuilderを引数として受け取る（所有しない）
 *
 * 責務:
 * - try_global_additional_resolvers_with_authority: 未解決Global callの追加解決
 * - materialize_receiver_in_callee: Receiverの実体化（pinning）
 * - Call発行前の準備処理全般
 */

use crate::mir::builder::callable_declaration_catalog::BareStaticRecoveryDecisionV1;
use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::{EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::mir::definitions::call_unified::Callee;

/// Call前処理・準備専用箱
///
/// 箱理論:
/// - 単一責務: Call発行前の前処理のみ
/// - 状態レス: MirBuilderを引数で受け取る設計
/// - サポート役: 本体のCall発行をサポートする役割
pub struct CallMaterializerBox;

/// Exclusive authority for direct global presence during migration.
///
/// Invocation headers and the legacy module-map observation are different
/// policies. The enum prevents an explicit header from being paired with a
/// contradictory legacy-presence flag.
#[derive(Clone, Copy)]
pub(in crate::mir::builder) enum GlobalPresenceAuthorityV1<'a> {
    InvocationHeader(&'a dyn FunctionSignatureLookupV1),
    LegacyCompatibility { present: bool },
}

impl CallMaterializerBox {
    /// Resolve direct global presence with one exclusive authority mode.
    pub(in crate::mir::builder) fn try_global_additional_resolvers_with_authority(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        name: &str,
        args: &[ValueId],
        authority: GlobalPresenceAuthorityV1<'_>,
    ) -> Result<Option<()>, String> {
        // 0) Dev-only safety: treat condition_fn as always-true predicate when missing
        if name == "condition_fn" {
            let dstv = dst.unwrap_or_else(|| builder.next_value_id());
            // Emit integer constant via ConstantEmissionBox
            let one = crate::mir::builder::emission::constant::emit_integer(builder, 1)?;
            if dst.is_none() {
                // If a destination was not provided, copy into the allocated dstv
                builder.emit_instruction(MirInstruction::Copy {
                    dst: dstv,
                    src: one,
                })?;
                crate::mir::builder::metadata::propagate::propagate(builder, one, dstv);
            } else {
                // If caller provided dst, ensure the computed value lands there
                builder.emit_instruction(MirInstruction::Copy {
                    dst: dstv,
                    src: one,
                })?;
                crate::mir::builder::metadata::propagate::propagate(builder, one, dstv);
            }
            return Ok(Some(()));
        }

        // 1) Direct module function resolver: call by name if present
        let direct_presence = match authority {
            GlobalPresenceAuthorityV1::InvocationHeader(headers) => headers.contains_symbol(name),
            GlobalPresenceAuthorityV1::LegacyCompatibility { present } => present,
        };
        if direct_presence {
            let dstv = dst.unwrap_or_else(|| builder.next_value_id());
            let name_const =
                crate::mir::builder::name_const::make_name_const_result(builder, name)?;
            builder.emit_instruction(MirInstruction::Call {
                dst: Some(dstv),
                func: name_const,
                callee: Some(Callee::Global(name.to_string())),
                args: args.to_vec(),
                effects: EffectMask::IO,
            })?;
            match authority {
                GlobalPresenceAuthorityV1::InvocationHeader(headers) => {
                    super::annotation::annotate_call_result_from_func_name_with_lookup(
                        builder,
                        dstv,
                        name,
                        Some(headers),
                    );
                }
                GlobalPresenceAuthorityV1::LegacyCompatibility { .. } => {
                    builder.annotate_call_result_from_func_name(dstv, name);
                }
            }
            return Ok(Some(()));
        }

        // 2) Unique static-method resolver: name+arity → canonical key → MIR symbol.
        let recovery = {
            let catalog = builder
                .comp_ctx
                .callable_declaration_catalog()
                .map_err(|error| error.to_string())?;
            BareStaticRecoveryDecisionV1::decide(catalog, name, args.len())
                .map_err(|error| error.to_string())?
        };
        if let BareStaticRecoveryDecisionV1::Unique(key) = recovery {
            let func_name = key.mir_symbol_projection();
            // Emit the resolved global call directly.
            let dstv = dst.unwrap_or_else(|| builder.next_value_id());
            let name_const =
                crate::mir::builder::name_const::make_name_const_result(builder, &func_name)?;
            builder.emit_instruction(MirInstruction::Call {
                dst: Some(dstv),
                func: name_const,
                callee: Some(Callee::Global(func_name.clone())),
                args: args.to_vec(),
                effects: EffectMask::IO,
            })?;
            match authority {
                GlobalPresenceAuthorityV1::InvocationHeader(headers) => {
                    super::annotation::annotate_call_result_from_func_name_with_lookup(
                        builder,
                        dstv,
                        &func_name,
                        Some(headers),
                    );
                }
                GlobalPresenceAuthorityV1::LegacyCompatibility { .. } => {
                    builder.annotate_call_result_from_func_name(dstv, func_name);
                }
            }
            return Ok(Some(()));
        }

        Ok(None)
    }

    /// Ensure receiver is materialized in Callee::Method
    ///
    /// Receiver実体化の目的:
    /// - receiverをスロットにpinningして、start_new_blockで伝播可能に
    /// - SSA不変条件の保持（receiverが常に定義済みであることを保証）
    /// - デバッグトレース出力（NYASH_BUILDER_TRACE_RECV=1）
    pub fn materialize_receiver_in_callee(
        _builder: &mut MirBuilder,
        callee: Callee,
    ) -> Result<Callee, String> {
        // Phase 25.1j+:
        // Receiver 実体化（pinning + LocalSSA）は ReceiverMaterializationBox
        // （crate::mir::builder::receiver）側に一本化したよ。
        // ここでは Callee 構造は変更せず、そのまま返す。
        //
        // NYASH_BUILDER_TRACE_RECV は新しい receiver.rs 側で扱う。
        Ok(callee)
    }
}
