//! Nyashランタイムモジュール
//!
//! プラグインシステムとBox管理の中核

pub mod box_registry;
pub mod core_box_ids; // Phase 87: CoreBoxId/CoreMethodId 型安全enum
pub mod core_method_aliases; // Phase 29ab: Core method alias SSOT
pub mod core_services; // Phase 91: CoreServices trait 定義
pub mod deprecations;
pub mod gc;
pub mod gc_controller;
pub mod gc_mode;
pub mod gc_trace;
mod gc_trigger_policy;
pub mod global_hooks;
pub mod leak_tracker;
pub mod mirbuilder_emit;
pub mod nyash_runtime;
pub mod observe; // Lightweight observability flags (OOB etc.)
pub mod plugin_config;
pub mod plugin_ffi_common;
pub mod plugin_host; // Phase 91: PluginHost skeleton
pub mod plugin_loader_unified;
pub mod plugin_loader_v2;
pub mod provider_lock;
pub mod provider_verify;
pub mod ring0; // Phase 88: Ring0Context - OS API 抽象化レイヤー
pub mod runtime_profile; // Phase 109: RuntimeProfile enum (Default/NoFs)
pub mod scheduler;
pub mod semantics;
pub mod unified_registry; // Deprecation warnings with warn-once guards
pub mod extern_registry; // ExternCall (env.*) 登録・診断用レジストリ
pub mod host_api; // C ABI: plugins -> host 逆呼び出しAPI（TLSでVMに橋渡し）
pub mod host_handles; // C ABI(TLV) 向け HostHandle レジストリ（ユーザー/内蔵Box受け渡し）
mod host_handles_policy;
pub mod modules_registry;
pub mod type_box_abi; // Phase 12: Nyash ABI (vtable) 雛形
pub mod type_meta;
pub mod type_registry;
pub mod weak_handles; // Phase 285LLVM-1: WeakRef Handle レジストリ（bit 63 = 1） // Phase 12: TypeId→TypeBox 解決（雛形） // env.modules minimal registry

#[cfg(test)]
mod tests;

pub use box_registry::{get_global_registry, BoxFactoryRegistry, BoxProvider};
pub use core_box_ids::{CoreBoxCategory, CoreBoxId, CoreMethodId}; // Phase 87: 型安全enum
pub use plugin_config::PluginConfig;
pub use plugin_host::CoreInitError; // Phase 92: CoreServices 初期化エラー
pub use plugin_loader_unified::{
    get_global_plugin_host, init_global_plugin_host, MethodHandle, PluginBoxType, PluginHost,
    PluginLibraryHandle,
};
pub use plugin_loader_v2::{get_global_loader_v2, init_global_loader_v2, PluginLoaderV2};
pub use ring0::{get_global_ring0, init_global_ring0, Ring0Context}; // Phase 88: Ring0 公開 API
pub use runtime_profile::RuntimeProfile; // Phase 109: RuntimeProfile enum
pub mod cache_versions;
pub use gc::{BarrierKind, GcHooks};
pub use nyash_runtime::{NyashRuntime, NyashRuntimeBuilder};
pub use scheduler::{Scheduler, SingleThreadScheduler};
pub use unified_registry::{
    get_global_unified_registry, init_global_unified_registry, register_user_defined_factory,
};

// Phase 95: CoreServices 用 global accessor
use std::sync::{Arc, OnceLock};
static GLOBAL_CORE_PLUGIN_HOST: OnceLock<Arc<plugin_host::PluginHost>> = OnceLock::new();

pub fn init_core_plugin_host(host: plugin_host::PluginHost) {
    GLOBAL_CORE_PLUGIN_HOST
        .set(Arc::new(host))
        .expect("[Phase 95] CorePluginHost already initialized");
}

pub fn get_core_plugin_host() -> Arc<plugin_host::PluginHost> {
    GLOBAL_CORE_PLUGIN_HOST
        .get()
        .expect("[Phase 95] CorePluginHost not initialized")
        .clone()
}

/// Phase 98: Safe accessor that returns None if not initialized
pub fn try_get_core_plugin_host() -> Option<Arc<plugin_host::PluginHost>> {
    GLOBAL_CORE_PLUGIN_HOST.get().cloned()
}

/// Phase 98: Helper macro to print using ConsoleService if available, otherwise eprintln
/// Phase 103: Updated to handle Option<Arc<dyn ConsoleService>>
#[macro_export]
macro_rules! console_println {
    ($($arg:tt)*) => {
        if let Some(host) = $crate::runtime::try_get_core_plugin_host() {
            if let Some(ref console) = host.core.console {
                console.println(&format!($($arg)*));
            } else {
                eprintln!($($arg)*);
            }
        } else {
            eprintln!($($arg)*);
        }
    };
}

/// Runtime 初期化（Phase 112: Ring0Registry-aware initialization）
///
/// Phase 94: フォールバック削除 - 常に実際の Box を使用
/// Phase 95: global に登録して get_core_plugin_host() でアクセス可能に
/// Phase 109: RuntimeProfile に基づく条件付き初期化
/// Phase 112: Ring0Registry による profile-aware Ring0Context 構築
///
/// # Responsibility Separation (Phase 112)
///
/// - **initialize_runtime**: 環境変数から profile を読む（唯一の env reader）、Ring0Registry.build() 呼び出し
/// - **Ring0Registry**: profile に応じた Ring0Context 実装選択
/// - **PluginHost**: profile を引数として受け取り、provider 初期化を実行（initialization hub）
///
/// # Profile behavior
///
/// - **Default**: FileBox provider 必須（Fail-Fast）、全 core services 有効、StdFs 使用
/// - **NoFs**: FileBox provider optional（NoFsFileIo stub）、core services のみ有効、NoFsApi 使用
///
/// # Ring0 initialization flow (Phase 112)
///
/// 1. RuntimeProfile::from_env() で profile 読み込み（env 読み込み唯一の場所）
/// 2. Ring0Registry::build(profile) で Ring0Context 構築
/// 3. init_global_ring0() で GLOBAL_RING0 に登録
/// 4. PluginHost 初期化時に get_global_ring0() で取得
pub fn initialize_runtime(ring0: std::sync::Arc<Ring0Context>) -> Result<(), CoreInitError> {
    use crate::box_factory::builtin::BuiltinBoxFactory;
    use crate::box_factory::UnifiedBoxRegistry;

    // Phase 109/112: Read RuntimeProfile from environment (this layer only)
    let profile = RuntimeProfile::from_env();

    let mut registry = UnifiedBoxRegistry::with_env_policy();

    // Phase 94: BuiltinBoxFactory を登録して core_required Boxes を提供
    registry.register(std::sync::Arc::new(BuiltinBoxFactory::new()));

    // Phase 109: PluginHost acts as initialization hub (handles FileBox provider)
    let plugin_host = plugin_host::PluginHost::with_core_from_registry_optional(
        ring0,
        &registry,
        plugin_host::CoreServicesConfig::all_enabled(),
        &profile,
    )?;
    plugin_host.ensure_core_initialized();

    // Phase 95: global に登録
    init_core_plugin_host(plugin_host);

    Ok(())
}
