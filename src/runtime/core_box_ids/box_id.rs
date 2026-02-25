use crate::runtime::runtime_profile::RuntimeProfile;

/// Phase 85 調査結果に基づく Core Box ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoreBoxId {
    // ===== Phase 85: core_required (6個) =====
    /// StringBox - 文字列基本型
    String,
    /// IntegerBox - 整数基本型
    Integer,
    /// BoolBox - 真偽値基本型
    Bool,
    /// ArrayBox - 配列基本型
    Array,
    /// MapBox - マップ基本型
    Map,
    /// ConsoleBox - コンソール入出力
    Console,

    // ===== Phase 85: core_optional (9個) =====
    /// FloatBox - 浮動小数点数
    Float,
    /// NullBox - null値
    Null,
    /// FileBox - ファイル操作
    File,
    /// PathBox - パス操作
    Path,
    /// RegexBox - 正規表現
    Regex,
    /// MathBox - 数学関数
    Math,
    /// TimeBox - 時刻操作
    Time,
    /// JsonBox - JSON操作
    Json,
    /// TomlBox - TOML操作
    Toml,

    // ===== 特殊型 =====
    /// FunctionBox - 第一級関数
    Function,
    /// ResultBox - Result型（QMark対応）
    Result,
    /// MethodBox - メソッド
    Method,
    /// MissingBox - 欠損値
    Missing,
}

/// Phase 87: Core Box カテゴリ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreBoxCategory {
    /// Phase 85: 必須（予約名保護）
    CoreRequired,
    /// Phase 85: 推奨（デフォルトロード）
    CoreOptional,
    /// 特殊型
    Special,
}

impl CoreBoxId {
    /// Box名を返す（例: "StringBox"）
    pub fn name(&self) -> &'static str {
        use CoreBoxId::*;
        match self {
            String => "StringBox",
            Integer => "IntegerBox",
            Bool => "BoolBox",
            Array => "ArrayBox",
            Map => "MapBox",
            Console => "ConsoleBox",
            Float => "FloatBox",
            Null => "NullBox",
            File => "FileBox",
            Path => "PathBox",
            Regex => "RegexBox",
            Math => "MathBox",
            Time => "TimeBox",
            Json => "JsonBox",
            Toml => "TomlBox",
            Function => "FunctionBox",
            Result => "ResultBox",
            Method => "MethodBox",
            Missing => "MissingBox",
        }
    }

    /// 全CoreBoxIdを反復
    pub fn iter() -> impl Iterator<Item = CoreBoxId> {
        use CoreBoxId::*;
        [
            String, Integer, Bool, Array, Map, Console, Float, Null, File, Path, Regex, Math, Time,
            Json, Toml, Function, Result, Method, Missing,
        ]
        .into_iter()
    }

    /// 名前からCoreBoxIdを取得
    pub fn from_name(name: &str) -> Option<CoreBoxId> {
        Self::iter().find(|id| id.name() == name)
    }

    /// Phase 106: core_required チェック
    ///
    /// FileBox は Phase 85 では core_optional として分類していたが、
    /// selfhost/通常ランタイムでは事実上必須（ログ・ツール・ハコチェック等で常用）
    /// であることが明確になったため、「core_required 相当」として扱う設計に統一した。
    ///
    /// **設計原則**:
    /// - 必須判定は CoreBoxId に一本化（provider_lock は「登録・読む」だけ）
    /// - 将来 minimal/no-fs プロファイルを導入する場合は、ここで profile パラメータを追加可能
    pub fn is_core_required(&self) -> bool {
        use CoreBoxId::*;
        matches!(self, String | Integer | Bool | Array | Map | Console | File)
    }

    /// Phase 109: profile-aware required check
    ///
    /// Determines if this CoreBox is required in the given RuntimeProfile.
    ///
    /// - Default: Same as is_core_required() (FileBox is required)
    /// - NoFs: FileBox becomes optional (only String/Integer/Bool/Array/Map/Console required)
    ///
    /// **Future expansion**: TestMock/Sandbox/ReadOnly/Embedded profiles will extend this logic
    pub fn is_required_in(&self, profile: &RuntimeProfile) -> bool {
        match profile {
            RuntimeProfile::Default => {
                // Phase 106: FileBox is required in Default profile
                self.is_core_required()
            }
            RuntimeProfile::NoFs => {
                // Phase 109: FileBox is optional in NoFs profile
                // In NoFs profile, only non-FileBox core required boxes
                self.is_core_required() && *self != Self::File
            }
        }
    }

    /// Phase 87: カテゴリ分類
    pub fn category(&self) -> CoreBoxCategory {
        use CoreBoxId::*;
        match self {
            // Phase 106: File を CoreRequired 側に移動（selfhost/通常ランタイムでは必須）
            String | Integer | Bool | Array | Map | Console | File => CoreBoxCategory::CoreRequired,
            Float | Null | Path | Regex | Math | Time | Json | Toml => {
                CoreBoxCategory::CoreOptional
            }
            Function | Result | Method | Missing => CoreBoxCategory::Special,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_box_id_from_name() {
        assert_eq!(CoreBoxId::from_name("StringBox"), Some(CoreBoxId::String));
        assert_eq!(CoreBoxId::from_name("IntegerBox"), Some(CoreBoxId::Integer));
        assert_eq!(CoreBoxId::from_name("BoolBox"), Some(CoreBoxId::Bool));
        assert_eq!(CoreBoxId::from_name("UnknownBox"), None);
    }

    #[test]
    fn test_core_box_id_name() {
        assert_eq!(CoreBoxId::String.name(), "StringBox");
        assert_eq!(CoreBoxId::Integer.name(), "IntegerBox");
        assert_eq!(CoreBoxId::Console.name(), "ConsoleBox");
    }

    #[test]
    fn test_core_box_id_iter() {
        let count = CoreBoxId::iter().count();
        assert_eq!(count, 19); // Phase 87: 19個の CoreBox
    }

    #[test]
    fn test_core_box_id_is_core_required() {
        // Phase 85: core_required (6個) + FileBox を実質必須扱いに拡張
        assert!(CoreBoxId::String.is_core_required());
        assert!(CoreBoxId::Integer.is_core_required());
        assert!(CoreBoxId::Bool.is_core_required());
        assert!(CoreBoxId::Array.is_core_required());
        assert!(CoreBoxId::Map.is_core_required());
        assert!(CoreBoxId::Console.is_core_required());
        assert!(CoreBoxId::File.is_core_required());

        // core_optional の代表例
        assert!(!CoreBoxId::Float.is_core_required());
    }

    #[test]
    fn test_core_box_id_category() {
        assert_eq!(CoreBoxId::String.category(), CoreBoxCategory::CoreRequired);
        // Phase 106: File の分類を修正
        assert_eq!(CoreBoxId::File.category(), CoreBoxCategory::CoreRequired);
        assert_eq!(CoreBoxId::Function.category(), CoreBoxCategory::Special);
    }

    #[test]
    fn test_core_box_id_is_required_in_default() {
        let profile = RuntimeProfile::Default;
        assert!(CoreBoxId::String.is_required_in(&profile));
        assert!(CoreBoxId::Integer.is_required_in(&profile));
        assert!(CoreBoxId::Bool.is_required_in(&profile));
        assert!(CoreBoxId::Array.is_required_in(&profile));
        assert!(CoreBoxId::Map.is_required_in(&profile));
        assert!(CoreBoxId::Console.is_required_in(&profile));
        assert!(CoreBoxId::File.is_required_in(&profile)); // FileBox required in Default
    }

    #[test]
    fn test_core_box_id_is_required_in_nofs() {
        let profile = RuntimeProfile::NoFs;
        assert!(CoreBoxId::String.is_required_in(&profile));
        assert!(CoreBoxId::Integer.is_required_in(&profile));
        assert!(CoreBoxId::Bool.is_required_in(&profile));
        assert!(CoreBoxId::Array.is_required_in(&profile));
        assert!(CoreBoxId::Map.is_required_in(&profile));
        assert!(CoreBoxId::Console.is_required_in(&profile));
        assert!(!CoreBoxId::File.is_required_in(&profile)); // FileBox optional in NoFs
    }
}
