/*!
 * CallMaterializerBox - Call前処理・準備専用箱
 *
 * 箱理論の実践:
 * - 箱にする: Call発行前の前処理を1箱に集約
 * - 境界を作る: receiver実体化を分離
 * - 状態最小: MirBuilderを引数として受け取る（所有しない）
 *
 * 責務:
 * - receiver materialization（pinning）
 * - Call発行前の準備処理全般
 */

use crate::mir::builder::MirBuilder;
use crate::mir::definitions::call_unified::Callee;

/// Call前処理・準備専用箱
///
/// 箱理論:
/// - 単一責務: Call発行前の前処理のみ
/// - 状態レス: MirBuilderを引数で受け取る設計
/// - サポート役: 本体のCall発行をサポートする役割
pub struct CallMaterializerBox;

impl CallMaterializerBox {
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
