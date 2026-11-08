//! One transaction owns provisional PHI tokens and rollback failures.
use super::{define_provisional_phi, patch_phi_inputs, rollback_provisional_phi};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PhiToken {
    block: BasicBlockId,
    dst: ValueId,
}

impl PhiToken {
    pub(in crate::mir::builder) const fn dst(self) -> ValueId {
        self.dst
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PhiRollbackFailureV1 {
    token: PhiToken,
    error: String,
}

#[cfg(test)]
impl PhiRollbackFailureV1 {
    pub(in crate::mir::builder) fn block(&self) -> BasicBlockId {
        self.token.block
    }

    pub(in crate::mir::builder) fn dst(&self) -> ValueId {
        self.token.dst
    }

    pub(in crate::mir::builder) fn error(&self) -> &str {
        &self.error
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PhiTxnAbortErrorV1 {
    tag: String,
    pending_count: usize,
    primary: String,
    cleanup_failures: Box<[PhiRollbackFailureV1]>,
}

#[cfg(test)]
impl PhiTxnAbortErrorV1 {
    pub(in crate::mir::builder) fn primary(&self) -> &str {
        &self.primary
    }

    pub(in crate::mir::builder) fn pending_count(&self) -> usize {
        self.pending_count
    }

    pub(in crate::mir::builder) fn cleanup_failures(&self) -> &[PhiRollbackFailureV1] {
        &self.cleanup_failures
    }
}

impl std::fmt::Display for PhiTxnAbortErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cleanup = self
            .cleanup_failures
            .iter()
            .map(|failure| {
                format!(
                    "bb={:?}:dst=%{}:<{}>",
                    failure.token.block, failure.token.dst.0, failure.error
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        write!(
            f,
            "[freeze:contract][phi_lifecycle/txn_abort] tag={} pending_count={} source_error=<{}> cleanup_failure_count={} cleanup_failures=[{}]",
            self.tag,
            self.pending_count,
            self.primary,
            self.cleanup_failures.len(),
            cleanup
        )
    }
}

impl std::error::Error for PhiTxnAbortErrorV1 {}

/// Transaction wrapper for provisional PHI lifecycle operations.
///
/// This is a structural guard over the existing low-level lifecycle helpers.
/// It does not change release routing or accepted source shapes; it only
/// centralizes "define provisional, patch or rollback, then commit" ordering.
#[derive(Debug)]
pub(in crate::mir::builder) struct PhiTxn {
    tag: String,
    pending: Vec<PhiToken>,
}

impl PhiTxn {
    /// Read-only census of tokens issued and still owned by this transaction.
    pub(in crate::mir::builder) fn pending_tokens_v1(&self) -> impl Iterator<Item = (BasicBlockId, PhiToken)> + '_ {
        self.pending.iter().copied().map(|token| (token.block, token))
    }

    pub(in crate::mir::builder) fn begin(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            pending: Vec::new(),
        }
    }

    pub(in crate::mir::builder) fn define_provisional_phi(
        &mut self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
        dst: ValueId,
        tag: &str,
    ) -> Result<PhiToken, String> {
        define_provisional_phi(builder, block, dst, tag)?;
        let token = PhiToken { block, dst };
        self.pending.push(token);
        Ok(token)
    }

    /// Reject foreign or already completed tokens before touching MIR.
    pub(in crate::mir::builder) fn patch_phi_inputs(
        &mut self,
        builder: &mut MirBuilder,
        token: PhiToken,
        inputs: Vec<(BasicBlockId, ValueId)>,
        tag: &str,
    ) -> Result<(), String> {
        if !self.pending.contains(&token) {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/patch_not_pending] tag={} bb={:?} dst=%{}",
                tag, token.block, token.dst.0,
            ));
        }
        patch_phi_inputs(builder, token.block, token.dst, inputs, tag)?;
        self.pending
            .retain(|pending| pending.block != token.block || pending.dst != token.dst);
        Ok(())
    }

    /// Roll back one still-pending provisional PHI owned by this transaction.
    ///
    /// A patched token is no longer pending and cannot be rolled back through
    /// this entry. Failed rollback keeps the token pending so the enclosing
    /// abort path can retain and retry every cleanup obligation.
    pub(in crate::mir::builder) fn rollback_pending_phi(
        &mut self,
        builder: &mut MirBuilder,
        token: PhiToken,
        tag: &str,
    ) -> Result<(), String> {
        if !self.pending.contains(&token) {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/rollback_not_pending] tag={} bb={:?} dst=%{}",
                tag, token.block, token.dst.0
            ));
        }
        match rollback_provisional_phi(builder, token.block, token.dst, tag)? {
            true => {
                self.pending.retain(|pending| *pending != token);
                Ok(())
            }
            false => Err(format!(
                "[freeze:contract][phi_lifecycle/rollback_missing_phi] tag={} bb={:?} dst=%{}",
                tag, token.block, token.dst.0
            )),
        }
    }

    pub(in crate::mir::builder) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> Result<(), PhiTxnAbortErrorV1> {
        if self.pending.is_empty() {
            return Ok(());
        }

        let err = format!(
            "[freeze:contract][phi_lifecycle/provisional_left_unpatched] tag={} pending_count={} pending={}",
            self.tag,
            self.pending.len(),
            self.pending
                .iter()
                .map(|token| format!("bb={:?}:dst=%{}", token.block, token.dst.0))
                .collect::<Vec<_>>()
                .join(",")
        );
        Err(self.abort_on_err(builder, err))
    }

    pub(in crate::mir::builder) fn abort_on_err(
        self,
        builder: &mut MirBuilder,
        err: String,
    ) -> PhiTxnAbortErrorV1 {
        let mut cleanup_failures = Vec::new();
        for token in &self.pending {
            let rollback = rollback_provisional_phi(
                builder,
                token.block,
                token.dst,
                &format!("{}:abort", self.tag),
            );
            match rollback {
                Ok(true) => {}
                Ok(false) => cleanup_failures.push(PhiRollbackFailureV1 {
                    token: *token,
                    error: "provisional PHI was not found during rollback".to_string(),
                }),
                Err(error) => cleanup_failures.push(PhiRollbackFailureV1 {
                    token: *token,
                    error,
                }),
            }
        }
        PhiTxnAbortErrorV1 {
            tag: self.tag,
            pending_count: self.pending.len(),
            primary: err,
            cleanup_failures: cleanup_failures.into_boxed_slice(),
        }
    }
}

