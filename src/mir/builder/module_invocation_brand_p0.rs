//! CUT0-I0-ROOT0-BRAND0 fixtures for the real physical owner chain.

use super::module_invocation_brand0::ActiveModuleInvocationV1;
use super::module_invocation_identity::{
    ModuleInvocationFamilyV1, TestInvocationPreflightFactoryV1,
};
use super::module_invocation_route_matrix::InvocationRootFamilyV1;
use super::module_invocation_session::{
    BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerV1,
};
use super::MirBuilder;

fn token(
    factory: &mut TestInvocationPreflightFactoryV1,
) -> super::module_invocation_identity::ModuleInvocationTokenV1 {
    factory.mint(InvocationRootFamilyV1::Raw).unwrap()
}

#[test]
fn one_token_brands_actual_session_shell_collector_and_ledger() {
    let current = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &current,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let active =
        ActiveModuleInvocationV1::open(token(&mut factory), &current, config, "brand0".into())
            .unwrap();
    let brand = active.brand();
    assert_eq!(active.session().brand(), brand);
    assert_eq!(active.physical().brand(), brand);
    assert_eq!(active.physical().shell().brand(), brand);
    assert_eq!(active.physical().collector().brand(), brand);

    let ledger = RawExpansionReceiptLedgerV1::new_for_token(
        &active.into_parts().0,
        RawCallableMainCompatibilityDispositionV1::NotSelected,
    );
    assert_eq!(ledger.brand(), brand);
}

#[test]
fn foreign_tokens_cannot_be_confused_with_the_active_owner() {
    let current = MirBuilder::new();
    let config =
        BuilderInvocationConfigV1::snapshot_with_policy(&current, BuilderCoreSeedPolicyV1::Fresh);
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let first = ActiveModuleInvocationV1::open(
        token(&mut factory),
        &current,
        config.clone(),
        "first".into(),
    )
    .unwrap();
    let second =
        ActiveModuleInvocationV1::open(token(&mut factory), &current, config, "second".into())
            .unwrap();
    assert_ne!(first.brand(), second.brand());
    assert_ne!(first.physical().brand(), second.physical().brand());
}

#[test]
fn prepared_commit_keeps_the_invocation_brand() {
    let current = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &current,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let source = token(&mut factory);
    let brand = source.brand();
    let session = ModuleBuilderInvocationSessionV1::open_for_token(&source, &current, config);
    let prepared = session.prepare_external_commit().unwrap();
    assert_eq!(prepared.brand(), brand);
}

#[test]
fn dropping_an_active_owner_does_not_mutate_the_live_builder() {
    let current = MirBuilder::new();
    let before = current.core_ctx.peek_next_value().as_u32();
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &current,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut factory = TestInvocationPreflightFactoryV1::new();
    let active =
        ActiveModuleInvocationV1::open(token(&mut factory), &current, config, "drop".into())
            .unwrap();
    drop(active);
    assert_eq!(current.core_ctx.peek_next_value().as_u32(), before);
}

#[allow(dead_code)]
fn _family_is_route_typed(family: ModuleInvocationFamilyV1) -> ModuleInvocationFamilyV1 {
    family
}
