//! Disconnected acceptance-matrix proof for the sole associated-item match.

use std::convert::Infallible;

use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind, IfMode};

use super::dispatch::{
    lower_verified_parts_associated_item, PartsAssociatedBlockModeV1,
    PartsAssociatedLoweringHooksV1,
};
use super::{
    sealed, PartsAssociatedRecipeItemV1, PartsAssociatedSourceErrorV1, PartsAssociatedSourceV1,
    VerifiedPartsAssociatedItemV1,
};

struct FakeSource;

impl sealed::Sealed for FakeSource {}

impl PartsAssociatedSourceV1 for FakeSource {
    type PortHandle = &'static str;
    type BlockInput = &'static str;
    type StmtInput = &'static str;
    type ConditionInput = &'static str;
    type BodyInput = &'static str;
    type WrappedJoinInput = &'static str;
    type LoopInput = &'static str;

    fn block_len(&self, _block: &Self::BlockInput) -> Result<usize, PartsAssociatedSourceErrorV1> {
        unreachable!("recording source publishes no blocks")
    }

    fn item(
        &self,
        _block: &Self::BlockInput,
        _index: usize,
    ) -> Result<
        VerifiedPartsAssociatedItemV1<
            Self::PortHandle,
            Self::StmtInput,
            Self::ConditionInput,
            Self::BodyInput,
            Self::BlockInput,
            Self::WrappedJoinInput,
            Self::LoopInput,
        >,
        PartsAssociatedSourceErrorV1,
    > {
        unreachable!("recording source publishes no items")
    }
}

#[derive(Default)]
struct RecordingHooks {
    events: Vec<&'static str>,
}

impl PartsAssociatedLoweringHooksV1<FakeSource> for RecordingHooks {
    type Output = &'static str;

    fn lower_opaque_stmt(
        &mut self,
        _port: &'static str,
        _source: &'static str,
    ) -> Result<Self::Output, String> {
        self.events.push("stmt");
        Ok("stmt")
    }

    fn lower_opaque_exit(
        &mut self,
        _port: &'static str,
        _source: &'static str,
        _kind: ExitKind,
    ) -> Result<Self::Output, String> {
        self.events.push("exit");
        Ok("exit")
    }

    fn lower_explicit_if(
        &mut self,
        _port: &'static str,
        _source: &'static str,
        _condition: &'static str,
        _then_body: &'static str,
        _else_body: Option<&'static str>,
        _contract: IfContractKind,
        _then_block: &'static str,
        _else_block: Option<&'static str>,
    ) -> Result<Self::Output, String> {
        self.events.push("if");
        Ok("if")
    }

    fn lower_stmt_wrapped_join_if(
        &mut self,
        _port: &'static str,
        _bridge: &'static str,
    ) -> Result<Self::Output, String> {
        self.events.push("wrapped");
        Ok("wrapped")
    }

    fn lower_raw_loop_v0(
        &mut self,
        _port: &'static str,
        _loop_input: &'static str,
    ) -> Result<Self::Output, String> {
        self.events.push("loop");
        Ok("loop")
    }
}

type FakeItem = PartsAssociatedRecipeItemV1<
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
>;

fn verified(
    item: FakeItem,
) -> VerifiedPartsAssociatedItemV1<
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
> {
    VerifiedPartsAssociatedItemV1 { port: "port", item }
}

fn dispatch(mode: PartsAssociatedBlockModeV1, item: FakeItem) -> Result<&'static str, String> {
    lower_verified_parts_associated_item::<FakeSource, _>(
        mode,
        verified(item),
        &mut RecordingHooks::default(),
        "recording",
    )
}

#[test]
fn sole_dispatcher_accepts_the_existing_block_mode_matrix() {
    for mode in [
        PartsAssociatedBlockModeV1::ExitOnly,
        PartsAssociatedBlockModeV1::ExitAllowed,
        PartsAssociatedBlockModeV1::StmtOnly,
        PartsAssociatedBlockModeV1::NoExit,
    ] {
        assert_eq!(
            dispatch(
                mode,
                PartsAssociatedRecipeItemV1::OpaqueStmt { source: "s" }
            )
            .expect("statement is admitted everywhere"),
            "stmt"
        );
    }

    for mode in [
        PartsAssociatedBlockModeV1::ExitOnly,
        PartsAssociatedBlockModeV1::ExitAllowed,
    ] {
        assert_eq!(
            dispatch(
                mode,
                PartsAssociatedRecipeItemV1::OpaqueExit {
                    source: "s",
                    kind: ExitKind::Return,
                },
            )
            .expect("exit-bearing block accepts Exit"),
            "exit"
        );
        for contract in [
            IfContractKind::ExitOnly {
                mode: IfMode::ExitIf,
            },
            IfContractKind::ExitOnly {
                mode: IfMode::ExitAll,
            },
            IfContractKind::ExitAllowed {
                mode: IfMode::ThenOnlyExit,
            },
            IfContractKind::ExitAllowed {
                mode: IfMode::ElseOnlyExit,
            },
        ] {
            assert_eq!(
                dispatch(mode, explicit_if(contract)).expect("exit If admitted"),
                "if"
            );
        }
        assert_eq!(
            dispatch(
                mode,
                PartsAssociatedRecipeItemV1::RawLoopV0 { loop_input: "loop" },
            )
            .expect("raw nested Loop remains admitted"),
            "loop"
        );
    }

    assert_eq!(
        dispatch(
            PartsAssociatedBlockModeV1::NoExit,
            explicit_if(IfContractKind::Join),
        )
        .expect("NoExit accepts Join If"),
        "if"
    );
    assert_eq!(
        dispatch(
            PartsAssociatedBlockModeV1::NoExit,
            PartsAssociatedRecipeItemV1::RawLoopV0 { loop_input: "loop" },
        )
        .expect("NoExit preserves raw nested Loop"),
        "loop"
    );
    assert_eq!(
        dispatch(
            PartsAssociatedBlockModeV1::ExitAllowed,
            PartsAssociatedRecipeItemV1::StmtWrappedJoinIf { bridge: "join" },
        )
        .expect("located ExitAllowed accepts its sealed Join bridge"),
        "wrapped"
    );
}

#[test]
fn invalid_if_contract_modes_reject_without_invoking_hooks() {
    for mode in [
        PartsAssociatedBlockModeV1::ExitOnly,
        PartsAssociatedBlockModeV1::ExitAllowed,
    ] {
        for contract in [
            IfContractKind::ExitOnly {
                mode: IfMode::ThenOnlyExit,
            },
            IfContractKind::ExitOnly {
                mode: IfMode::ElseOnlyExit,
            },
            IfContractKind::ExitAllowed {
                mode: IfMode::ExitIf,
            },
            IfContractKind::ExitAllowed {
                mode: IfMode::ExitAll,
            },
        ] {
            let mut hooks = RecordingHooks::default();
            let error = lower_verified_parts_associated_item::<FakeSource, _>(
                mode,
                verified(explicit_if(contract)),
                &mut hooks,
                "invalid-if-contract",
            )
            .expect_err("invalid If contract/mode pair must reject before hooks");
            assert!(error.contains("dispatch_saw_unsupported_item"));
            assert!(hooks.events.is_empty());
        }
    }
}

#[test]
fn sole_dispatcher_rejects_cross_mode_items_without_invoking_hooks() {
    for (mode, item) in [
        (
            PartsAssociatedBlockModeV1::StmtOnly,
            PartsAssociatedRecipeItemV1::OpaqueExit {
                source: "s",
                kind: ExitKind::Return,
            },
        ),
        (
            PartsAssociatedBlockModeV1::NoExit,
            explicit_if(IfContractKind::ExitOnly {
                mode: IfMode::ExitIf,
            }),
        ),
        (
            PartsAssociatedBlockModeV1::ExitAllowed,
            explicit_if(IfContractKind::Join),
        ),
        (
            PartsAssociatedBlockModeV1::ExitAllowed,
            explicit_if(IfContractKind::ExitAllowed {
                mode: IfMode::ExitAll,
            }),
        ),
        (
            PartsAssociatedBlockModeV1::ExitOnly,
            explicit_if(IfContractKind::ExitAllowed {
                mode: IfMode::ExitAll,
            }),
        ),
        (
            PartsAssociatedBlockModeV1::ExitOnly,
            explicit_if(IfContractKind::ExitAllowed {
                mode: IfMode::ExitIf,
            }),
        ),
        (
            PartsAssociatedBlockModeV1::ExitAllowed,
            explicit_if(IfContractKind::ExitAllowed {
                mode: IfMode::ExitIf,
            }),
        ),
        (
            PartsAssociatedBlockModeV1::NoExit,
            PartsAssociatedRecipeItemV1::StmtWrappedJoinIf { bridge: "join" },
        ),
    ] {
        let mut hooks = RecordingHooks::default();
        let error = lower_verified_parts_associated_item::<FakeSource, _>(
            mode,
            verified(item),
            &mut hooks,
            "reject",
        )
        .expect_err("cross-mode item must reject");
        let expected = if mode == PartsAssociatedBlockModeV1::StmtOnly {
            "stmt_only_block_contains_non_stmt_item"
        } else {
            "dispatch_saw_unsupported_item"
        };
        assert!(error.contains(expected), "unexpected error: {error}");
        assert!(hooks.events.is_empty());
    }
}

fn explicit_if(contract: IfContractKind) -> FakeItem {
    PartsAssociatedRecipeItemV1::ExplicitIfV2 {
        source: "if",
        condition: "cond",
        then_body: "then-body",
        else_body: Some("else-body"),
        contract,
        then_block: "then-block",
        else_block: Some("else-block"),
    }
}

#[allow(dead_code)]
fn _infallible_is_not_a_located_loop_input(_: Infallible) {}
