//! Caller-zero logical output product for the S6C `ScanWithInit` cohort.
//!
//! The product retains the verified Facts/Recipe/Join closure and owns only
//! the fixed logical rows emitted from that product.  Consumers borrow a
//! private façade; raw Recipe, JoinSig, and constituent products never cross
//! this boundary.

use super::s6c_scan_with_init::VerifiedS6CScanWithInitRecipeProductV2;
use super::s6c_scan_with_init_joinir::{
    with_s6c_scan_with_init_logical_join_input, S6CLogicalCallInputRefV1, S6CLogicalCallRoleV1,
};
use super::s6c_scan_with_init_joinir_output_rows::{
    issue_s6c_logical_output_rows, S6CLogicalCallSlotV1, S6CLogicalOutputRejectV1,
    S6CLogicalOutputRowsV1,
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
            let calls = S6CLogicalCallPairsRefV1 {
                length: S6CLogicalCallWithSourceRefV1 {
                    row: *self
                        .rows
                        .calls()
                        .first()
                        .expect("verified S6C Length call row"),
                    source: input.length(),
                },
                substring: S6CLogicalCallWithSourceRefV1 {
                    row: *self
                        .rows
                        .calls()
                        .get(1)
                        .expect("verified S6C Substring call row"),
                    source: input.substring(),
                },
            };
            callback(S6CScanWithInitLogicalOutputRefV1 {
                rows: &self.rows,
                calls,
                transfer: input.logical_transfer(),
            })
        })
        .expect("verified S6C logical output parity")
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScanWithInitLogicalOutputRefV1<'rows, 'product> {
    rows: &'rows S6CLogicalOutputRowsV1,
    calls: S6CLogicalCallPairsRefV1<'product>,
    transfer: &'product super::join_sig::LoopJoinLogicalTransferViewV2<'product>,
}

impl<'rows, 'product> S6CScanWithInitLogicalOutputRefV1<'rows, 'product> {
    pub(crate) const fn rows(self) -> &'rows S6CLogicalOutputRowsV1 {
        self.rows
    }

    pub(crate) const fn calls(self) -> S6CLogicalCallPairsRefV1<'product> {
        self.calls
    }

    pub(crate) const fn logical_transfer(
        self,
    ) -> &'product super::join_sig::LoopJoinLogicalTransferViewV2<'product> {
        self.transfer
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CLogicalCallPairsRefV1<'product> {
    length: S6CLogicalCallWithSourceRefV1<'product>,
    substring: S6CLogicalCallWithSourceRefV1<'product>,
}

impl<'product> S6CLogicalCallPairsRefV1<'product> {
    pub(crate) const fn len(self) -> usize {
        2
    }

    pub(crate) const fn length(self) -> S6CLogicalCallWithSourceRefV1<'product> {
        self.length
    }

    pub(crate) const fn substring(self) -> S6CLogicalCallWithSourceRefV1<'product> {
        self.substring
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CLogicalCallWithSourceRefV1<'product> {
    row: S6CLogicalCallSlotV1,
    source: S6CLogicalCallInputRefV1<'product>,
}

impl<'product> S6CLogicalCallWithSourceRefV1<'product> {
    pub(crate) const fn role(self) -> S6CLogicalCallRoleV1 {
        self.row.role()
    }

    pub(crate) const fn row(self) -> S6CLogicalCallSlotV1 {
        self.row
    }

    pub(crate) const fn source(self) -> S6CLogicalCallInputRefV1<'product> {
        self.source
    }
}

pub(crate) fn issue_s6c_scan_with_init_logical_output_v1(
    product: VerifiedS6CScanWithInitRecipeProductV2,
) -> Result<VerifiedS6CScanWithInitLogicalOutputV1, S6CLogicalOutputRejectV1> {
    let rows = with_s6c_scan_with_init_logical_join_input(&product, issue_s6c_logical_output_rows)
        .map_err(|_| S6CLogicalOutputRejectV1::Control("logical input"))??;
    Ok(VerifiedS6CScanWithInitLogicalOutputV1 { product, rows })
}
