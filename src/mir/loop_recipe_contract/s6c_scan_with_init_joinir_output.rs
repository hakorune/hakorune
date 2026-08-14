//! Caller-zero logical output product for the S6C `ScanWithInit` cohort.
//!
//! The product retains the verified Facts/Recipe/Join closure and owns only
//! the fixed logical rows emitted from that product.  Consumers borrow a
//! private façade; raw Recipe, JoinSig, and constituent products never cross
//! this boundary.

use super::s6c_scan_with_init::VerifiedS6CScanWithInitRecipeProductV2;
use super::s6c_scan_with_init_joinir::{
    with_s6c_scan_with_init_logical_join_input, S6CScanWithInitLogicalJoinInputRefV1,
};
use super::s6c_scan_with_init_joinir_output_rows::{
    issue_s6c_logical_output_rows, S6CLogicalOutputRejectV1, S6CLogicalOutputRowsV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedS6CScanWithInitLogicalOutputV1 {
    product: VerifiedS6CScanWithInitRecipeProductV2,
    rows: S6CLogicalOutputRowsV1,
}

impl VerifiedS6CScanWithInitLogicalOutputV1 {
    pub(crate) fn with_output<R>(
        &self,
        callback: impl for<'rows, 'product> FnOnce(
            S6CScanWithInitLogicalOutputRefV1<'rows, 'product>,
        ) -> R,
    ) -> R {
        with_s6c_scan_with_init_logical_join_input(&self.product, |input| {
            callback(S6CScanWithInitLogicalOutputRefV1 {
                rows: &self.rows,
                input,
            })
        })
        .expect("verified S6C logical output parity")
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScanWithInitLogicalOutputRefV1<'rows, 'product> {
    rows: &'rows S6CLogicalOutputRowsV1,
    input: S6CScanWithInitLogicalJoinInputRefV1<'product>,
}

impl<'rows, 'product> S6CScanWithInitLogicalOutputRefV1<'rows, 'product> {
    pub(crate) const fn rows(self) -> &'rows S6CLogicalOutputRowsV1 {
        self.rows
    }

    pub(crate) const fn logical_transfer(
        self,
    ) -> &'product super::join_sig::LoopJoinLogicalTransferViewV2<'product> {
        self.input.logical_transfer()
    }

    pub(crate) const fn input(self) -> S6CScanWithInitLogicalJoinInputRefV1<'product> {
        self.input
    }
}

pub(crate) fn issue_s6c_scan_with_init_logical_output_v1(
    product: VerifiedS6CScanWithInitRecipeProductV2,
) -> Result<VerifiedS6CScanWithInitLogicalOutputV1, S6CLogicalOutputRejectV1> {
    let rows = with_s6c_scan_with_init_logical_join_input(&product, issue_s6c_logical_output_rows)
        .map_err(|_| S6CLogicalOutputRejectV1::Control("logical input"))??;
    Ok(VerifiedS6CScanWithInitLogicalOutputV1 { product, rows })
}
