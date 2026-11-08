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
        let complete = !inputs.is_empty();
        patch_phi_inputs(builder, token.block, token.dst, inputs, tag)?;
        if complete { self.pending
            .retain(|pending| pending.block != token.block || pending.dst != token.dst); }
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

    /// Finish an existing plan scope using the same transaction as its writer.
    /// Source transactions are borrowed by their owner instead of calling this.
    pub(in crate::mir::builder) fn finish_plan_scope(
        self, builder: &mut MirBuilder, result: Result<(), String>,
        labels: &[(BasicBlockId, ValueId, &str)], strict_diagnostics: bool,
    ) -> Result<(), String> {
        if self.pending.is_empty() {
            return match result {
                Ok(()) => self.commit(builder).map_err(|error| error.to_string()),
                Err(error) => Err(self.abort_on_err(builder, error).primary),
            };
        }
        let first = self.pending[0];
        let label = |token: PhiToken| labels.iter()
            .find(|(block, value, _)| *block == token.block && *value == token.dst)
            .map(|(_, _, label)| *label).unwrap_or("<unknown>");
        let function = builder.function_state.current_function.as_ref()
            .map(|function| function.signature.name.clone()).unwrap_or_default();
        let failed = result.is_err();
        let primary = result.err().unwrap_or_else(|| "none".to_owned());
        let error = self.abort_on_err(builder, primary);
        if !strict_diagnostics && failed && error.cleanup_failures.is_empty() {
            return Err(error.primary);
        }
        let reason = if !error.cleanup_failures.is_empty() { "rollback_failed" }
            else if failed { "err_before_patch" } else { "ok_after_patch" };
        let original = error.cleanup_failures.first().map(|failure| format!(
            "dst=%{} bb={:?} tag={} rollback_err={}", failure.token.dst.0,
            failure.token.block, label(failure.token), failure.error))
            .unwrap_or_else(|| error.primary.clone());
        let original = if original.starts_with("[freeze:")
            && !original.contains(['\n', '\r']) { original }
            else { let short: String = original.chars().take(80).collect();
                if original.chars().count() > 80 { format!("{short}...") } else { short } };
        let rollback_count = error.pending_count - error.cleanup_failures.len();
        let additional = if !error.cleanup_failures.is_empty() { format!(" cleanup={error}") }
            else { String::new() };
        Err(format!("[freeze:contract][phi_lifecycle/provisional_left_unpatched] fn={} phi_dst=%{} phi_bb={:?} tag={} orig_err=<{}> reason={} rollback_count={}{}",
            function, first.dst.0, first.block, label(first), original, reason, rollback_count, additional))
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
