//! MIR NamingBox — static box / entry naming rules
//!
// 責務:
// - static box メソッドを MIR 関数名にエンコード/デコードする。
// - 「Main._nop/0」などの名前付けを一箇所に集約し、Builder/Interpreter 間で共有する。
// - 当面は minimal: `main.*` → `Main.*` のケースを安全に扱う。
//!
// 非責務:
// - 動的 dispatch や BoxFactory 側の名前解決。
// - エントリポイント選択（NYASH_ENTRY）のポリシー決定。

/// Encode a static box method into a MIR function name: `BoxName.method/arity`.
pub fn encode_static_method(box_name: &str, method: &str, arity: usize) -> String {
    format!(
        "{}.{}{}",
        canonical_box_name(box_name),
        method,
        format!("/{}", arity)
    )
}

/// Canonicalize a static box name for MIR-level usage.
///
/// 現状のルール:
/// - "main" → "Main"（最小限の補正）
/// - それ以外はそのまま返す（広域な仕様変更は避ける）。
pub fn canonical_box_name(raw: &str) -> String {
    match raw {
        "main" => "Main".to_string(),
        _ => raw.to_string(),
    }
}

/// If `func_name` looks like a static box method like `main._nop/0`,
/// normalize the box part (`main` → `Main`) and return canonical form.
///
/// 例:
/// - "main._nop/0" → "Main._nop/0"
/// - "Main._nop/0" → "Main._nop/0"（変化なし）
/// - その他の名前は入力そのまま返す。
pub fn normalize_static_global_name(func_name: &str) -> String {
    if let Some((box_part, rest)) = func_name.split_once('.') {
        // rest には "method/arity" が入る想定
        let canon = canonical_box_name(box_part);
        if canon != box_part {
            return format!("{}.{}", canon, rest);
        }
    }
    func_name.to_string()
}

/// Decode a MIR function name into (box_name, method, arity).
///
/// Format: "BoxName.method/arity"
///
/// Returns:
/// - Some((box_name, method, arity)) if successfully parsed
/// - None if not in expected format
///
/// Examples:
/// - "Main.main/0" → Some(("Main", "main", 0))
/// - "Calculator.add/2" → Some(("Calculator", "add", 2))
/// - "print" → None (no dot)
/// - "Main.main" → None (no arity)
pub fn decode_static_method(func_name: &str) -> Option<(&str, &str, usize)> {
    // Split by '.' to extract box_name and "method/arity"
    let (box_name, method_arity) = func_name.split_once('.')?;

    // Split by '/' to extract method and arity
    let (method, arity_str) = method_arity.split_once('/')?;

    // Parse arity as usize
    let arity = arity_str.parse::<usize>().ok()?;

    Some((box_name, method, arity))
}

/// Check if a function name is a static box method.
///
/// Returns true if the name matches "BoxName.method/arity" format.
///
/// Examples:
/// - "Main.main/0" → true
/// - "Calculator.add/2" → true
/// - "print" → false
/// - "instance.method" → false (no arity)
pub fn is_static_method_name(func_name: &str) -> bool {
    decode_static_method(func_name).is_some()
}

// =========================================================================
// Phase 1: StaticMethodId - 構造化された関数名表現
// =========================================================================

/// Global 関数名の構造化表現
///
/// MIR の Global 関数名（"Box.method/N"）をパース・生成するための型。
///
/// # Examples
///
/// ```
/// use hakorune_selfhost::mir::naming::StaticMethodId;
///
/// // パース（arity 有り）
/// let id = StaticMethodId::parse("StringUtils.starts_with/2").unwrap();
/// assert_eq!(id.box_name, "StringUtils");
/// assert_eq!(id.method, "starts_with");
/// assert_eq!(id.arity, Some(2));
///
/// // パース（arity 無し）
/// let id = StaticMethodId::parse("StringUtils.starts_with").unwrap();
/// assert_eq!(id.arity, None);
///
/// // フォーマット
/// let id = StaticMethodId {
///     box_name: "Main".to_string(),
///     method: "main".to_string(),
///     arity: Some(0),
/// };
/// assert_eq!(id.format(), "Main.main/0");
///
/// // arity 補完
/// let with_arity = id.with_arity(2);
/// assert_eq!(with_arity.format(), "Main.main/2");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticMethodId {
    pub box_name: String,
    pub method: String,
    pub arity: Option<usize>, // None = arity 未定（後で補完）
}

impl StaticMethodId {
    /// "Box.method/N" or "Box.method" をパース
    ///
    /// # Arguments
    /// * `name` - 関数名（"BoxName.method/arity" または "BoxName.method"）
    ///
    /// # Returns
    /// * `Some(StaticMethodId)` - パース成功
    /// * `None` - パース失敗（不正なフォーマット）
    ///
    /// # Examples
    ///
    /// ```
    /// use hakorune_selfhost::mir::naming::StaticMethodId;
    ///
    /// // arity 有り
    /// let id = StaticMethodId::parse("Main._nop/0").unwrap();
    /// assert_eq!(id.box_name, "Main");
    /// assert_eq!(id.method, "_nop");
    /// assert_eq!(id.arity, Some(0));
    ///
    /// // arity 無し
    /// let id = StaticMethodId::parse("StringUtils.starts_with").unwrap();
    /// assert_eq!(id.box_name, "StringUtils");
    /// assert_eq!(id.method, "starts_with");
    /// assert_eq!(id.arity, None);
    ///
    /// // main → Main に normalize
    /// let id = StaticMethodId::parse("main._nop/0").unwrap();
    /// assert_eq!(id.box_name, "Main");
    /// ```
    pub fn parse(name: &str) -> Option<Self> {
        // 1. arity 分離: "Box.method/2" → ("Box.method", Some(2))
        let (base, arity) = if let Some(idx) = name.rfind('/') {
            let (b, a) = name.split_at(idx);
            let arity_num = a[1..].parse::<usize>().ok()?;
            (b, Some(arity_num))
        } else {
            (name, None)
        };

        // 2. box_name/method 分離
        let dot_idx = base.rfind('.')?;
        let box_name = base[..dot_idx].to_string();
        let method = base[dot_idx + 1..].to_string();

        // 3. box_name を normalize（main → Main など）
        let normalized_box = canonical_box_name(&box_name);

        Some(Self {
            box_name: normalized_box,
            method,
            arity,
        })
    }

    /// "Box.method/N" 形式で出力（arity が None なら /N なし）
    ///
    /// # Examples
    ///
    /// ```
    /// use hakorune_selfhost::mir::naming::StaticMethodId;
    ///
    /// let id = StaticMethodId {
    ///     box_name: "StringUtils".to_string(),
    ///     method: "starts_with".to_string(),
    ///     arity: Some(2),
    /// };
    /// assert_eq!(id.format(), "StringUtils.starts_with/2");
    ///
    /// let no_arity = StaticMethodId {
    ///     box_name: "StringUtils".to_string(),
    ///     method: "starts_with".to_string(),
    ///     arity: None,
    /// };
    /// assert_eq!(no_arity.format(), "StringUtils.starts_with");
    /// ```
    pub fn format(&self) -> String {
        match self.arity {
            Some(n) => format!("{}.{}/{}", self.box_name, self.method, n),
            None => format!("{}.{}", self.box_name, self.method),
        }
    }

    /// arity を補完して新しい StaticMethodId を返す
    ///
    /// # Examples
    ///
    /// ```
    /// use hakorune_selfhost::mir::naming::StaticMethodId;
    ///
    /// let id = StaticMethodId::parse("StringUtils.starts_with").unwrap();
    /// let with_arity = id.with_arity(2);
    /// assert_eq!(with_arity.arity, Some(2));
    /// assert_eq!(with_arity.format(), "StringUtils.starts_with/2");
    /// ```
    pub fn with_arity(&self, arity: usize) -> Self {
        Self {
            box_name: self.box_name.clone(),
            method: self.method.clone(),
            arity: Some(arity),
        }
    }
}

// 既存関数のエイリアス（互換性維持）

/// `StaticMethodId::parse()` のエイリアス
///
/// 互換性のために提供。新しいコードでは `StaticMethodId::parse()` を使用してください。
pub fn parse_global_name(name: &str) -> Option<StaticMethodId> {
    StaticMethodId::parse(name)
}

/// `StaticMethodId::format()` のエイリアス
///
/// 互換性のために提供。新しいコードでは `StaticMethodId::format()` を使用してください。
pub fn format_global_name(id: &StaticMethodId) -> String {
    id.format()
}
