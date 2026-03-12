//! TypeRegistryBox - 型情報管理の一元化
//!
//! 🎯 責務: 全ての ValueId の型情報・起源情報を一元管理
//! 🔧 境界: 型情報へのアクセスはこの箱経由のみ
//! 🔍 見える化: trace_origin() でデータフロー追跡可能
//! 🔄 戻せる: NYASH_TYPE_REGISTRY_TRACE=1 で詳細ログ

use crate::mir::{MirType, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::HashMap;

/// 型情報追跡エントリ（デバッグ用）
#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub vid: ValueId,
    pub source: String, // "newbox:MapBox", "param:args", "propagate:from_%123"
    #[allow(dead_code)]
    pub timestamp: usize,
}

/// 型情報レジストリ - 全ての ValueId の型・起源を管理
#[derive(Debug, Clone, Default)]
pub struct TypeRegistry {
    /// NewBox起源（new MapBox() → "MapBox"）
    origins: HashMap<ValueId, String>,

    /// 型注釈（パラメータ、推論など）
    types: HashMap<ValueId, MirType>,

    /// デバッグ用追跡ログ
    trace_log: Vec<TraceEntry>,

    /// トレース有効化フラグ（環境変数キャッシュ）
    trace_enabled: bool,
}

impl TypeRegistry {
    /// 新しいレジストリを作成
    pub fn new() -> Self {
        let trace_enabled = crate::config::env::builder_type_registry_trace();

        Self {
            origins: HashMap::new(),
            types: HashMap::new(),
            trace_log: Vec::new(),
            trace_enabled,
        }
    }

    // ============================================================
    // 📝 記録系メソッド（明示的な型情報設定）
    // ============================================================

    /// NewBox起源を記録
    #[allow(dead_code)]
    pub fn record_newbox(&mut self, vid: ValueId, class: String) {
        self.origins.insert(vid, class.clone());

        if self.trace_enabled {
            let entry = TraceEntry {
                vid,
                source: format!("newbox:{}", class),
                timestamp: self.trace_log.len(),
            };
            self.trace_log.push(entry.clone());
            get_global_ring0()
                .log
                .debug(&format!("[type-registry] {} {:?}", entry.source, vid));
        }
    }

    /// パラメータ型を記録
    #[allow(dead_code)]
    pub fn record_param(&mut self, vid: ValueId, param_name: &str, param_type: Option<MirType>) {
        if let Some(ty) = param_type.clone() {
            self.types.insert(vid, ty.clone());

            if self.trace_enabled {
                let entry = TraceEntry {
                    vid,
                    source: format!("param:{}:{:?}", param_name, ty),
                    timestamp: self.trace_log.len(),
                };
                self.trace_log.push(entry.clone());
                get_global_ring0()
                    .log
                    .debug(&format!("[type-registry] {} {:?}", entry.source, vid));
            }
        } else if self.trace_enabled {
            let entry = TraceEntry {
                vid,
                source: format!("param:{}:no_type", param_name),
                timestamp: self.trace_log.len(),
            };
            self.trace_log.push(entry.clone());
            get_global_ring0()
                .log
                .debug(&format!("[type-registry] {} {:?}", entry.source, vid));
        }
    }

    /// 型注釈を明示的に設定
    pub fn record_type(&mut self, vid: ValueId, ty: MirType) {
        self.types.insert(vid, ty.clone());

        if self.trace_enabled {
            let entry = TraceEntry {
                vid,
                source: format!("type:{:?}", ty),
                timestamp: self.trace_log.len(),
            };
            self.trace_log.push(entry.clone());
            get_global_ring0()
                .log
                .debug(&format!("[type-registry] {} {:?}", entry.source, vid));
        }
    }

    /// 起源を明示的に設定（推論結果など）
    #[allow(dead_code)]
    pub fn record_origin(&mut self, vid: ValueId, origin: String, reason: &str) {
        self.origins.insert(vid, origin.clone());

        if self.trace_enabled {
            let entry = TraceEntry {
                vid,
                source: format!("{}:{}", reason, origin),
                timestamp: self.trace_log.len(),
            };
            self.trace_log.push(entry.clone());
            get_global_ring0()
                .log
                .debug(&format!("[type-registry] {} {:?}", entry.source, vid));
        }
    }

    // ============================================================
    // 🔄 伝播系メソッド（メタデータの伝播）
    // ============================================================

    /// メタデータを src から dst へ伝播
    pub fn propagate(&mut self, src: ValueId, dst: ValueId) {
        let mut propagated = false;

        if let Some(cls) = self.origins.get(&src).cloned() {
            self.origins.insert(dst, cls.clone());
            propagated = true;

            if self.trace_enabled {
                let entry = TraceEntry {
                    vid: dst,
                    source: format!("propagate:from_%{}→{}", src.0, cls),
                    timestamp: self.trace_log.len(),
                };
                self.trace_log.push(entry.clone());
                get_global_ring0()
                    .log
                    .debug(&format!("[type-registry] {} {:?}", entry.source, dst));
            }
        }

        if let Some(ty) = self.types.get(&src).cloned() {
            self.types.insert(dst, ty.clone());

            if self.trace_enabled && !propagated {
                let entry = TraceEntry {
                    vid: dst,
                    source: format!("propagate:from_%{}→{:?}", src.0, ty),
                    timestamp: self.trace_log.len(),
                };
                self.trace_log.push(entry.clone());
                get_global_ring0()
                    .log
                    .debug(&format!("[type-registry] {} {:?}", entry.source, dst));
            }
        }
    }

    // ============================================================
    // 🔍 取得系メソッド（型情報の読み取り）
    // ============================================================

    /// 起源クラス名を取得
    #[allow(dead_code)]
    pub fn get_origin(&self, vid: ValueId) -> Option<&String> {
        self.origins.get(&vid)
    }

    /// 型情報を取得
    #[allow(dead_code)]
    pub fn get_type(&self, vid: ValueId) -> Option<&MirType> {
        self.types.get(&vid)
    }

    /// クラス名を推論（フォールバック戦略付き）
    pub fn infer_class(&self, vid: ValueId, fallback_context: Option<&str>) -> String {
        // 優先1: 起源情報から
        if let Some(cls) = self.origins.get(&vid) {
            return cls.clone();
        }

        // 優先2: 型注釈から
        if let Some(MirType::Box(cls)) = self.types.get(&vid) {
            return cls.clone();
        }

        // フォールバック: コンテキスト名（警告付き）
        if let Some(ctx) = fallback_context {
            if self.trace_enabled {
                get_global_ring0().log.warn(&format!(
                    "[type-registry] WARNING: fallback to context '{}' for %{}",
                    ctx, vid.0
                ));
            }
            return ctx.to_string();
        }

        // 最終フォールバック: UnknownBox
        if self.trace_enabled {
            get_global_ring0().log.warn(&format!(
                "[type-registry] WARNING: UnknownBox for %{}",
                vid.0
            ));
        }
        "UnknownBox".to_string()
    }

    // ============================================================
    // 🔍 デバッグ支援メソッド
    // ============================================================

    /// 起源追跡チェーンを取得
    pub fn trace_origin(&self, vid: ValueId) -> Vec<String> {
        self.trace_log
            .iter()
            .filter(|e| e.vid == vid)
            .map(|e| e.source.clone())
            .collect()
    }

    /// 全トレースログを表示
    #[allow(dead_code)]
    pub fn dump_trace(&self) {
        get_global_ring0().log.debug(&format!(
            "[type-registry] === Trace Log ({} entries) ===",
            self.trace_log.len()
        ));
        for entry in &self.trace_log {
            get_global_ring0().log.debug(&format!(
                "[type-registry] #{:04} %{} ← {}",
                entry.timestamp, entry.vid.0, entry.source
            ));
        }
    }

    /// 統計情報を表示
    #[allow(dead_code)]
    pub fn dump_stats(&self) {
        get_global_ring0()
            .log
            .debug("[type-registry] === Statistics ===");
        get_global_ring0().log.debug(&format!(
            "[type-registry] Origins: {} entries",
            self.origins.len()
        ));
        get_global_ring0().log.debug(&format!(
            "[type-registry] Types: {} entries",
            self.types.len()
        ));
        get_global_ring0().log.debug(&format!(
            "[type-registry] Trace log: {} entries",
            self.trace_log.len()
        ));
    }

    // ============================================================
    // 🧹 クリア系メソッド（BoxCompilationContext用）
    // ============================================================

    /// 起源情報のみクリア（型情報は保持）
    #[allow(dead_code)]
    pub fn clear_origins(&mut self) {
        self.origins.clear();
        if self.trace_enabled {
            get_global_ring0()
                .log
                .debug("[type-registry] cleared origins");
        }
    }

    /// 全情報クリア
    #[allow(dead_code)]
    pub fn clear_all(&mut self) {
        self.origins.clear();
        self.types.clear();
        if self.trace_enabled {
            get_global_ring0().log.debug("[type-registry] cleared all");
        }
    }
}
