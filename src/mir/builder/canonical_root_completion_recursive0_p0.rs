//! RECURSIVE0 focused fixtures for branded capability witnesses.

use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use super::module_invocation_owner_chain::InvocationBranded;
use super::module_lowering_shell::ModuleLoweringShellV1;
use crate::mir::MirModule;

fn shell(brand: ModuleInvocationBrandV1) -> InvocationBranded<ModuleLoweringShellV1> {
    InvocationBranded::from_test(
        brand,
        ModuleLoweringShellV1::from_empty_module(MirModule::new("recursive0".into())).unwrap(),
    )
}

#[test]
fn recursive_install_returns_exact_brand_and_family_once() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let mut shell = shell(brand);
    let receipt = shell
        .install_callable_batch_capability(ModuleInvocationFamilyV1::BindingSsaRecursive)
        .unwrap()
        .unwrap();
    assert_eq!(receipt.brand(), brand);
    assert_eq!(
        receipt.family(),
        ModuleInvocationFamilyV1::BindingSsaRecursive
    );
    let duplicate = shell
        .install_callable_batch_capability(ModuleInvocationFamilyV1::BindingSsaRecursive)
        .unwrap_err();
    assert!(duplicate.contains("capability_preexisting"));
}

#[test]
fn acyclic_install_returns_branded_absence_witness() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let mut shell = shell(brand);
    let absence = shell
        .install_callable_batch_capability(ModuleInvocationFamilyV1::BindingSsaAcyclic)
        .unwrap()
        .unwrap_err();
    assert_eq!(absence.brand(), brand);
    assert_eq!(
        absence.family(),
        ModuleInvocationFamilyV1::BindingSsaAcyclic
    );
}

#[test]
fn acyclic_route_rejects_a_preexisting_recursive_marker() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let mut shell = shell(brand);
    shell
        .install_callable_batch_capability(ModuleInvocationFamilyV1::BindingSsaRecursive)
        .unwrap()
        .unwrap();
    let error = shell
        .install_callable_batch_capability(ModuleInvocationFamilyV1::BindingSsaAcyclic)
        .unwrap_err();
    assert!(error.contains("capability_unexpected"));
}
