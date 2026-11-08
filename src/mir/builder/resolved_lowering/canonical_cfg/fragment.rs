//! Fragment outgoing seals and CFG incoming seals have distinct meanings.
//! Both are owned by the same function CFG; fragment preflight remains atomic.
use super::CanonicalCfgSessionV1;
use crate::mir::builder::control_flow::edgecfg::api::{emit::emit_frag, Frag};
use crate::mir::{BasicBlockId, MirFunction};
use std::collections::BTreeSet;

impl CanonicalCfgSessionV1 {
    fn collect_from_blocks(frag: &Frag) -> BTreeSet<BasicBlockId> {
        frag.wires
            .iter()
            .map(|row| row.from)
            .chain(frag.branches.iter().map(|row| row.from))
            .collect()
    }
    fn assert_open(&self, block: BasicBlockId, context: &str) -> Result<(), String> {
        if self.fragment_outgoing.contains(&block) {
            return Err(format!(
                "[freeze:contract] Cannot emit to sealed block {:?}: {}",
                block, context
            ));
        }
        Ok(())
    }
    fn seal(&mut self, block: BasicBlockId) {
        self.fragment_outgoing.insert(block);
    }
    #[cfg(test)]
    pub(in crate::mir::builder) fn is_sealed_for_test(&self, block: BasicBlockId) -> bool {
        self.fragment_outgoing.contains(&block)
    }
    pub(in crate::mir::builder) fn emit_and_seal(
        &mut self,
        function: &mut MirFunction,
        frag: &Frag,
    ) -> Result<(), String> {
        let sources = Self::collect_from_blocks(frag);
        for source in &sources {
            self.assert_open(*source, "emit_and_seal")?;
        }
        emit_frag(function, frag, self)?;
        for source in sources {
            self.seal(source);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session_is_empty() {
        let session = CanonicalCfgSessionV1::new();
        assert!(session.fragment_outgoing.is_empty());
    }

    #[test]
    fn test_reset_clears_sealed() {
        let mut session = CanonicalCfgSessionV1::new();
        let bb = BasicBlockId(42);
        session.seal(bb);
        assert!(!session.fragment_outgoing.is_empty());

        session = CanonicalCfgSessionV1::default();
        assert!(session.fragment_outgoing.is_empty());
    }

    #[test]
    fn test_assert_open_passes_for_unregistered() {
        let session = CanonicalCfgSessionV1::new();
        let bb = BasicBlockId(42);

        // 未登録 = Open
        let result = session.assert_open(bb, "test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_double_emit_returns_err() {
        // 同じ from を 2 回 emit_and_seal したら Err
        // （panic 依存ではない - Result で fail-fast）
        let mut session = CanonicalCfgSessionV1::new();
        let bb = BasicBlockId(42);

        // 最初の seal
        session.seal(bb);

        // 2 回目は Err
        let result = session.assert_open(bb, "test");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("[freeze:contract]"),
            "Error should contain '[freeze:contract]', got: {}",
            err
        );
    }

    #[test]
    fn test_collect_from_blocks_dedup() {
        use crate::mir::builder::control_flow::edgecfg::api::branch_stub::BranchStub;
        use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
        use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
        use crate::mir::edge_args::JumpArgsLayout;
        use crate::mir::EdgeArgs;
        use crate::mir::ValueId;
        use std::collections::BTreeMap;

        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);

        // 同じ from (bb0) が wires と branches に複数回出現
        let frag = Frag {
            entry: bb0,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![EdgeStub::new(
                bb0,
                ExitKind::Normal,
                Some(bb1),
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
            branches: vec![BranchStub::new(
                bb0, // 同じ from
                ValueId(100),
                bb1,
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
                bb2,
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
        };

        let from_blocks = CanonicalCfgSessionV1::collect_from_blocks(&frag);

        // BTreeSet で dedup されるので 1 つだけ
        assert_eq!(from_blocks.len(), 1);
        assert!(from_blocks.contains(&bb0));
    }
}
