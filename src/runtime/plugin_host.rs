//! Phase 91: PluginHost/CoreServices skeleton
//!
//! Ring1-Core（Box 実装層）の構造定義。
//! Phase 92 で UnifiedBoxRegistry と接続予定。

use crate::box_factory::UnifiedBoxRegistry;
use crate::runtime::CoreBoxId;
use crate::runtime::RuntimeProfile;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Phase 103: CoreServices Optional化設定
///
/// 環境変数で各CoreServiceの有効化を制御
/// - NYASH_CORE_DISABLE_STRING=1 → StringService無効
/// - NYASH_CORE_DISABLE_INTEGER=1 → IntegerService無効
/// - (etc.)
#[derive(Debug, Clone)]
pub struct CoreServicesConfig {
    pub string_enabled: bool,
    pub integer_enabled: bool,
    pub bool_enabled: bool,
    pub array_enabled: bool,
    pub map_enabled: bool,
    pub console_enabled: bool,
}

impl CoreServicesConfig {
    /// すべてのサービスを有効化（デフォルト）
    pub fn all_enabled() -> Self {
        Self {
            string_enabled: true,
            integer_enabled: true,
            bool_enabled: true,
            array_enabled: true,
            map_enabled: true,
            console_enabled: true,
        }
    }

    /// ConsoleOnly（メモリ最小化）
    pub fn minimal() -> Self {
        Self {
            string_enabled: false,
            integer_enabled: false,
            bool_enabled: false,
            array_enabled: false,
            map_enabled: false,
            console_enabled: true, // Console is mandatory
        }
    }
}

/// Plugin の基本情報
#[derive(Debug, Clone)]
pub struct PluginDescriptor {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>, // "json", "http", "cli" など
}

/// Nyash Plugin の trait
///
/// Phase 109: PluginRegistry skeleton removed (was Phase 92 placeholder)
pub trait NyashPlugin: Send + Sync {
    fn descriptor(&self) -> PluginDescriptor;
    // Note: register() method signature will be redesigned when actual plugin system is implemented
}

use super::core_services::CoreServices;
use super::ring0::Ring0Context;

/// CoreServices 初期化エラー
#[derive(Debug, Clone)]
pub enum CoreInitError {
    MissingService {
        box_id: CoreBoxId,
        /// Phase 109: hint now includes profile context
        hint: String,
    },
    RegistryEmpty,
    InvalidServiceType {
        box_id: CoreBoxId,
        expected: &'static str,
        found: String,
    },
}

impl std::fmt::Display for CoreInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoreInitError::MissingService { box_id, hint } => {
                write!(f, "Missing core service {:?}: {}", box_id, hint)
            }
            CoreInitError::RegistryEmpty => {
                write!(f, "UnifiedBoxRegistry is empty")
            }
            CoreInitError::InvalidServiceType {
                box_id,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Invalid service type for {:?}: expected {}, found {}",
                    box_id, expected, found
                )
            }
        }
    }
}

impl std::error::Error for CoreInitError {}

/// PluginHost: Ring0Context と CoreServices の橋渡し
#[derive(Debug)]
pub struct PluginHost {
    pub ring0: Arc<Ring0Context>,
    pub core: CoreServices,
    pub optional: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl PluginHost {
    fn register_default_provider_if_absent(
        ring0: &Ring0Context,
        is_registered: bool,
        register: impl FnOnce() -> Result<(), String>,
        registered_msg: &str,
        already_registered_msg: &str,
    ) {
        if is_registered {
            return;
        }
        match register() {
            Ok(()) => ring0.log.debug(registered_msg),
            Err(_) => ring0.log.debug(already_registered_msg),
        }
    }

    /// Phase 103/109: Optional CoreServices initialization with RuntimeProfile support
    ///
    /// Allows selective initialization based on CoreServicesConfig and RuntimeProfile.
    /// ConsoleBox is mandatory for user-facing output.
    ///
    /// Phase 109 additions:
    /// - `profile` parameter controls FileBox provider requirements
    /// - Default profile: FileBox provider is required (Fail-Fast)
    /// - NoFs profile: FileBox provider is optional (disabled)
    pub fn with_core_from_registry_optional(
        ring0: Arc<Ring0Context>,
        registry: &UnifiedBoxRegistry,
        config: CoreServicesConfig,
        profile: &RuntimeProfile,
    ) -> Result<Self, CoreInitError> {
        use crate::runtime::core_services::*;
        use crate::runtime::provider_lock;

        // Phase 109: Profile-aware required check
        for box_id in CoreBoxId::iter() {
            if box_id.is_required_in(profile) && !registry.has_type(box_id.name()) {
                return Err(CoreInitError::MissingService {
                    box_id,
                    hint: format!(
                        "Core Box {} is required in {} profile but not found in registry",
                        box_id.name(),
                        profile.name()
                    ),
                });
            }
        }

        // Phase 109: FileBox provider initialization hub (Responsibility: PluginHost)
        // This is the single source of truth for FileBox provider initialization
        match profile {
            RuntimeProfile::Default => {
                // Phase 109: Default profile requires FileBox provider
                if provider_lock::get_filebox_provider().is_none() {
                    // Phase 107: Auto-register Ring0FsFileIo as default provider
                    use crate::providers::ring1::file::ring0_fs_fileio::Ring0FsFileIo;
                    let provider = Arc::new(Ring0FsFileIo::new(ring0.clone()));

                    match provider_lock::set_filebox_provider(provider) {
                        Ok(()) => {
                            ring0.log.debug(
                                "[Phase 109] Ring0FsFileIo registered as default FileBox provider",
                            );
                        }
                        Err(_) => {
                            // Plugin provider already registered - this is OK (plugin priority)
                            ring0.log.debug("[Phase 109] Plugin FileBox provider already registered (plugin priority)");
                        }
                    }
                }

                // Phase 109: Verify FileBox provider exists (Fail-Fast)
                if CoreBoxId::File.is_required_in(profile) {
                    if provider_lock::get_filebox_provider().is_none() {
                        return Err(CoreInitError::MissingService {
                            box_id: CoreBoxId::File,
                            hint: "FileBox provider not registered (required for Default profile)"
                                .to_string(),
                        });
                    }
                }
            }
            RuntimeProfile::NoFs => {
                // Phase 109: NoFs profile uses NoFsFileIo stub
                if provider_lock::get_filebox_provider().is_none() {
                    use crate::providers::ring1::file::nofs_fileio::NoFsFileIo;
                    let provider = Arc::new(NoFsFileIo);

                    let _ = provider_lock::set_filebox_provider(provider);
                    ring0
                        .log
                        .debug("[Phase 109] NoFsFileIo registered for NoFs profile");
                }
            }
        }

        Self::register_default_provider_if_absent(
            ring0.as_ref(),
            provider_lock::get_pathbox_provider().is_some(),
            || {
                use crate::providers::ring1::path::Ring1PathService;
                provider_lock::set_pathbox_provider(Arc::new(Ring1PathService::new()))
            },
            "[Phase 29y] Ring1PathService registered as default PathBox provider",
            "[Phase 29y] PathBox provider already registered (plugin priority)",
        );

        let mut core = CoreServices {
            string: None,
            integer: None,
            bool: None,
            array: None,
            map: None,
            console: None,
        };

        // StringService: Optional
        if config.string_enabled {
            if !registry.has_type("StringBox") {
                return Err(CoreInitError::MissingService {
                    box_id: CoreBoxId::String,
                    hint: "StringBox enabled but not found in registry".to_string(),
                });
            }
            core.string = Some(Arc::new(StringBoxAdapter::new()));
        }

        // IntegerService: Optional
        if config.integer_enabled {
            if !registry.has_type("IntegerBox") {
                return Err(CoreInitError::MissingService {
                    box_id: CoreBoxId::Integer,
                    hint: "IntegerBox enabled but not found in registry".to_string(),
                });
            }
            core.integer = Some(Arc::new(IntegerBoxAdapter::new()));
        }

        // BoolService: Optional
        if config.bool_enabled {
            if !registry.has_type("BoolBox") {
                return Err(CoreInitError::MissingService {
                    box_id: CoreBoxId::Bool,
                    hint: "BoolBox enabled but not found in registry".to_string(),
                });
            }
            core.bool = Some(Arc::new(BoolBoxAdapter::new()));
        }

        // ArrayService: Optional
        if config.array_enabled {
            if !registry.has_type("ArrayBox") {
                return Err(CoreInitError::MissingService {
                    box_id: CoreBoxId::Array,
                    hint: "ArrayBox enabled but not found in registry".to_string(),
                });
            }
            Self::register_default_provider_if_absent(
                ring0.as_ref(),
                provider_lock::get_arraybox_provider().is_some(),
                || {
                    use crate::providers::ring1::array::Ring1ArrayService;
                    provider_lock::set_arraybox_provider(Arc::new(Ring1ArrayService::new()))
                },
                "[Phase 29y] Ring1ArrayService registered as default ArrayBox provider",
                "[Phase 29y] ArrayBox provider already registered (plugin priority)",
            );
            core.array = Some(
                provider_lock::new_arraybox_provider_instance().map_err(|hint| {
                    CoreInitError::MissingService {
                        box_id: CoreBoxId::Array,
                        hint,
                    }
                })?,
            );
        }

        // MapService: Optional
        if config.map_enabled {
            if !registry.has_type("MapBox") {
                return Err(CoreInitError::MissingService {
                    box_id: CoreBoxId::Map,
                    hint: "MapBox enabled but not found in registry".to_string(),
                });
            }
            Self::register_default_provider_if_absent(
                ring0.as_ref(),
                provider_lock::get_mapbox_provider().is_some(),
                || {
                    use crate::providers::ring1::map::Ring1MapService;
                    provider_lock::set_mapbox_provider(Arc::new(Ring1MapService::new()))
                },
                "[Phase 29y] Ring1MapService registered as default MapBox provider",
                "[Phase 29y] MapBox provider already registered (plugin priority)",
            );
            core.map = Some(
                provider_lock::new_mapbox_provider_instance().map_err(|hint| {
                    CoreInitError::MissingService {
                        box_id: CoreBoxId::Map,
                        hint,
                    }
                })?,
            );
        }

        // ConsoleService: MANDATORY (Graceful Degradation principle)
        if config.console_enabled {
            if !registry.has_type("ConsoleBox") {
                return Err(CoreInitError::MissingService {
                    box_id: CoreBoxId::Console,
                    hint: "ConsoleBox is mandatory but not found in registry".to_string(),
                });
            }
            Self::register_default_provider_if_absent(
                ring0.as_ref(),
                provider_lock::get_consolebox_provider().is_some(),
                || {
                    use crate::providers::ring1::console::Ring1ConsoleService;
                    provider_lock::set_consolebox_provider(Arc::new(Ring1ConsoleService::new()))
                },
                "[Phase 29y] Ring1ConsoleService registered as default ConsoleBox provider",
                "[Phase 29y] ConsoleBox provider already registered (plugin priority)",
            );
            core.console = Some(provider_lock::new_consolebox_provider_instance().map_err(
                |hint| CoreInitError::MissingService {
                    box_id: CoreBoxId::Console,
                    hint,
                },
            )?);
        } else {
            return Err(CoreInitError::MissingService {
                box_id: CoreBoxId::Console,
                hint: "Phase 103: ConsoleBox is mandatory for user-facing output".to_string(),
            });
        }

        Ok(PluginHost {
            ring0,
            core,
            optional: HashMap::new(),
        })
    }

    /// Phase 101/109: Backward compatibility - all services required with Default profile
    ///
    /// Maintains existing behavior: all CoreServices must be present.
    /// Used by default_ring0 and other initialization paths expecting all services.
    ///
    /// Phase 109: Now uses Default profile (FileBox required)
    pub fn with_core_from_registry(
        ring0: Arc<Ring0Context>,
        registry: &UnifiedBoxRegistry,
    ) -> Result<Self, CoreInitError> {
        // Use all_enabled() and Default profile for backward compatibility
        Self::with_core_from_registry_optional(
            ring0,
            registry,
            CoreServicesConfig::all_enabled(),
            &RuntimeProfile::Default,
        )
    }

    /// core_required が全て揃っているか検証
    pub fn ensure_core_initialized(&self) {
        self.core.ensure_initialized();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_descriptor() {
        let desc = PluginDescriptor {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["json".to_string()],
        };
        assert_eq!(desc.name, "test_plugin");
        assert_eq!(desc.version, "1.0.0");
        assert_eq!(desc.capabilities.len(), 1);
    }

    #[test]
    fn test_core_services_all_fields() {
        // Phase 94: 実際の registry を使用してテスト
        use crate::box_factory::builtin::BuiltinBoxFactory;
        use crate::runtime::ring0::default_ring0;

        let ring0 = Arc::new(default_ring0());
        let mut registry = UnifiedBoxRegistry::new();
        registry.register(Arc::new(BuiltinBoxFactory::new()));

        // Phase 107: FileBox provider auto-registration (no manual setup needed)
        // with_core_from_registry will call init_default_filebox_provider internally

        let plugin_host = PluginHost::with_core_from_registry(ring0, &registry)
            .expect("CoreServices should be initialized with builtin boxes");

        plugin_host.ensure_core_initialized();
        // panic しないことを確認
    }

    #[test]
    fn test_core_services_coverage() {
        // Phase 94: 実際の registry を使用して全フィールドが存在することを確認
        use crate::box_factory::builtin::BuiltinBoxFactory;
        use crate::runtime::ring0::default_ring0;

        let ring0 = Arc::new(default_ring0());
        let mut registry = UnifiedBoxRegistry::new();
        registry.register(Arc::new(BuiltinBoxFactory::new()));

        // Phase 107: FileBox provider auto-registration (no manual setup needed)
        // with_core_from_registry will call init_default_filebox_provider internally

        let plugin_host = PluginHost::with_core_from_registry(ring0, &registry)
            .expect("CoreServices should be initialized");

        // Phase 87 core_required (6個) と一致することを確認
        // String, Integer, Bool, Array, Map, Console
        let _string = &plugin_host.core.string;
        let _integer = &plugin_host.core.integer;
        let _bool = &plugin_host.core.bool;
        let _array = &plugin_host.core.array;
        let _map = &plugin_host.core.map;
        let _console = &plugin_host.core.console;

        // 全フィールドが存在することを確認
    }

    struct DummyPlugin;
    impl NyashPlugin for DummyPlugin {
        fn descriptor(&self) -> PluginDescriptor {
            PluginDescriptor {
                name: "dummy".to_string(),
                version: "0.1.0".to_string(),
                capabilities: vec![],
            }
        }
    }

    #[test]
    fn test_plugin_trait_implementation() {
        let plugin = DummyPlugin;
        let desc = plugin.descriptor();
        assert_eq!(desc.name, "dummy");
        assert_eq!(desc.version, "0.1.0");
    }

    #[test]
    fn test_core_init_error_display() {
        let error = CoreInitError::MissingService {
            box_id: CoreBoxId::String,
            hint: "StringBox not found".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("String"));
        assert!(display.contains("not found"));
    }

    #[test]
    fn test_with_core_from_registry_missing_box() {
        // Phase 95.5: registry が空の場合はエラーを返すことを確認
        use crate::runtime::ring0::default_ring0;
        let ring0 = Arc::new(default_ring0());
        let registry = UnifiedBoxRegistry::new();

        let result = PluginHost::with_core_from_registry(ring0.clone(), &registry);
        assert!(result.is_err());

        // Phase 95.5 + Phase 106: エラーメッセージチェック
        // FileBox provider が未登録の場合は、CoreBoxId::File のエラーが優先される
        if let Err(e) = result {
            let msg = format!("{}", e);
            ring0.log.debug(&format!("Error message: {}", msg)); // デバッグ出力
            assert!(
                msg.contains("not found in registry")
                    || msg.contains("creation failed")
                    || msg.contains("Unknown Box type")
                    || msg.contains("FileBox provider not registered"),
                "Error message should contain expected error patterns, got: {}",
                msg
            );
        }
    }

    #[test]
    fn test_with_core_from_registry_filebox_auto_registered() {
        // Phase 107/108: with_core_from_registry() は Ring0FsFileIo を自動登録するため、
        // FileBox は常に利用可能になる

        use crate::box_factory::builtin::BuiltinBoxFactory;
        use crate::runtime::ring0::default_ring0;

        let ring0 = Arc::new(default_ring0());
        let mut registry = UnifiedBoxRegistry::new();
        registry.register(Arc::new(BuiltinBoxFactory::new()));

        // Phase 107: with_core_from_registry() は Ring0FsFileIo を自動登録
        let result = PluginHost::with_core_from_registry(ring0, &registry);

        // Phase 107/108: FileBox provider は自動登録されるため、成功するはず
        assert!(
            result.is_ok(),
            "Expected success with auto-registered FileBox provider"
        );

        // Phase 108: 登録された provider は read/write 両対応
        use crate::runtime::provider_lock;
        if let Some(provider) = provider_lock::get_filebox_provider() {
            let caps = provider.caps();
            assert!(caps.read, "FileBox provider should support read");
            assert!(
                caps.write,
                "FileBox provider should support write (Phase 108)"
            );
        } else {
            panic!("FileBox provider should be registered after with_core_from_registry");
        }
    }

    #[test]
    fn test_with_core_from_registry_nofs_filebox_optional() {
        // Phase 109: NoFs profile では FileBox provider なしで OK
        use crate::box_factory::builtin::BuiltinBoxFactory;
        use crate::runtime::ring0::default_ring0;

        let ring0 = Arc::new(default_ring0());
        let mut registry = UnifiedBoxRegistry::new();
        registry.register(Arc::new(BuiltinBoxFactory::new()));

        // Phase 109: NoFs profile で初期化
        let profile = RuntimeProfile::NoFs;
        let result = PluginHost::with_core_from_registry_optional(
            ring0,
            &registry,
            CoreServicesConfig::all_enabled(),
            &profile,
        );

        // Phase 109: FileBox は optional なので、provider なしで成功するはず
        assert!(
            result.is_ok(),
            "Expected success with NoFs profile (FileBox optional)"
        );
    }
}

#[cfg(test)]
mod optional_core_tests {
    use super::*;

    #[test]
    fn test_core_services_config_all_enabled() {
        let config = CoreServicesConfig::all_enabled();
        assert!(config.string_enabled, "string should be enabled");
        assert!(config.integer_enabled, "integer should be enabled");
        assert!(config.bool_enabled, "bool should be enabled");
        assert!(config.array_enabled, "array should be enabled");
        assert!(config.map_enabled, "map should be enabled");
        assert!(config.console_enabled, "console should be enabled");
    }

    #[test]
    fn test_core_services_config_minimal() {
        let config = CoreServicesConfig::minimal();
        assert!(!config.string_enabled, "string should be disabled");
        assert!(!config.integer_enabled, "integer should be disabled");
        assert!(!config.bool_enabled, "bool should be disabled");
        assert!(!config.array_enabled, "array should be disabled");
        assert!(!config.map_enabled, "map should be disabled");
        assert!(config.console_enabled, "console must remain enabled");
    }
}
