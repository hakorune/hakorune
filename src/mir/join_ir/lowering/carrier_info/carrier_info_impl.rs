use super::types::{CarrierInfo, CarrierInit, CarrierRole, CarrierVar};
use crate::mir::ValueId;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

impl CarrierInfo {
    /// Phase 193-2: Create CarrierInfo from a variable_map
    ///
    /// Automatically extracts all non-loop-control variables from the host's
    /// variable_map. This eliminates manual carrier listing for simple cases.
    ///
    /// # Arguments
    ///
    /// * `loop_var_name` - Name of the loop control variable (e.g., "i")
    /// * `variable_map` - Host function's variable_map (String → ValueId)
    ///
    /// # Returns
    ///
    /// CarrierInfo with loop_var and all other variables as carriers
    ///
    /// # Example
    ///
    /// ```ignore
    /// let carrier_info = CarrierInfo::from_variable_map(
    ///     "i".to_string(),
    ///     &variable_map  // {"i": ValueId(5), "sum": ValueId(10), "count": ValueId(11)}
    /// )?;
    /// // Result: CarrierInfo with loop_var="i", carriers=[sum, count]
    /// ```
    pub fn from_variable_map(
        loop_var_name: String,
        variable_map: &BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism
    ) -> Result<Self, String> {
        // Find loop variable
        let loop_var_id = variable_map.get(&loop_var_name).copied().ok_or_else(|| {
            format!(
                "Loop variable '{}' not found in variable_map",
                loop_var_name
            )
        })?;

        // Collect all non-loop-var variables as carriers
        let mut carriers: Vec<CarrierVar> = variable_map
            .iter()
            .filter(|(name, _)| *name != &loop_var_name)
            .map(|(name, &id)| CarrierVar {
                name: name.clone(),
                host_id: id,
                join_id: None, // Phase 177-STRUCT-1: Set by header PHI generation
                role: CarrierRole::LoopState, // Phase 227: Default to LoopState
                init: CarrierInit::FromHost, // Phase 228: Default to FromHost
            })
            .collect();

        // Sort for determinism
        carriers.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(CarrierInfo {
            loop_var_name,
            loop_var_id,
            carriers,
            trim_helper: None, // Phase 171-C-5: No Trim pattern by default
            promoted_body_locals: Vec::new(), // Phase 224: No promoted variables by default
        })
    }

    /// Phase 193-2: Create CarrierInfo with explicit carrier list
    ///
    /// Useful when you have specific carriers in mind and want explicit control
    /// over which variables are treated as carriers.
    ///
    /// # Arguments
    ///
    /// * `loop_var_name` - Name of the loop control variable
    /// * `loop_var_id` - ValueId of the loop variable
    /// * `carrier_names` - Names of carrier variables (will look up in variable_map)
    /// * `variable_map` - Host function's variable_map for lookups
    ///
    /// # Returns
    ///
    /// CarrierInfo with only the specified carriers
    ///
    /// # Example
    ///
    /// ```ignore
    /// let carrier_info = CarrierInfo::with_explicit_carriers(
    ///     "i".to_string(),
    ///     ValueId(5),
    ///     vec!["sum".to_string(), "count".to_string()],
    ///     &variable_map
    /// )?;
    /// ```
    pub fn with_explicit_carriers(
        loop_var_name: String,
        loop_var_id: ValueId,
        carrier_names: Vec<String>,
        variable_map: &BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism
    ) -> Result<Self, String> {
        let mut carriers = Vec::new();

        for name in carrier_names {
            let host_id = variable_map
                .get(&name)
                .copied()
                .ok_or_else(|| format!("Carrier variable '{}' not found in variable_map", name))?;

            carriers.push(CarrierVar {
                name,
                host_id,
                join_id: None, // Phase 177-STRUCT-1: Set by header PHI generation
                role: CarrierRole::LoopState, // Phase 227: Default to LoopState
                init: CarrierInit::FromHost, // Phase 228: Default to FromHost
            });
        }

        // Sort for determinism
        carriers.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(CarrierInfo {
            loop_var_name,
            loop_var_id,
            carriers,
            trim_helper: None, // Phase 171-C-5: No Trim pattern by default
            promoted_body_locals: Vec::new(), // Phase 224: No promoted variables by default
        })
    }

    /// Phase 193-2: Create CarrierInfo with manual CarrierVar list
    ///
    /// Most explicit construction method - you provide everything directly.
    /// Useful when you already have CarrierVar structs built elsewhere.
    ///
    /// # Arguments
    ///
    /// * `loop_var_name` - Name of the loop control variable
    /// * `loop_var_id` - ValueId of the loop variable
    /// * `carriers` - Vec of already-constructed CarrierVar structs
    pub fn with_carriers(
        loop_var_name: String,
        loop_var_id: ValueId,
        mut carriers: Vec<CarrierVar>,
    ) -> Self {
        // Sort for determinism
        carriers.sort_by(|a, b| a.name.cmp(&b.name));

        Self {
            loop_var_name,
            loop_var_id,
            carriers,
            trim_helper: None, // Phase 171-C-5: No Trim pattern by default
            promoted_body_locals: Vec::new(), // Phase 224: No promoted variables by default
        }
    }

    /// Phase 193-2: Get carrier count
    ///
    /// Convenience method for checking how many carriers this info has.
    pub fn carrier_count(&self) -> usize {
        self.carriers.len()
    }

    /// Phase 193-2: Check if this has multiple carriers
    ///
    /// Useful for pattern matching: "is this a multi-carrier loop?"
    pub fn is_multi_carrier(&self) -> bool {
        self.carriers.len() > 1
    }

    /// Phase 193-2: Find a carrier by name
    ///
    /// Lookup a specific carrier variable by name.
    pub fn find_carrier(&self, name: &str) -> Option<&CarrierVar> {
        self.carriers.iter().find(|c| c.name == name)
    }

    /// Phase 171-C-4: Merge carriers from another CarrierInfo
    ///
    /// Deduplicates by carrier name. If a carrier with the same name already exists,
    /// it will not be added again.
    ///
    /// # Arguments
    ///
    /// * `other` - Another CarrierInfo to merge from
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut carrier_info = CarrierInfo::from_variable_map("i", &variable_map)?;
    /// let promoted_carrier = TrimPatternInfo::to_carrier_info();
    /// carrier_info.merge_from(&promoted_carrier);
    /// ```
    pub fn merge_from(&mut self, other: &CarrierInfo) {
        for carrier in &other.carriers {
            if !self.carriers.iter().any(|c| c.name == carrier.name) {
                self.carriers.push(carrier.clone());
            }
        }
        // Maintain sorted order for determinism
        self.carriers.sort_by(|a, b| a.name.cmp(&b.name));

        // Phase 171-C-5: Also merge trim_helper if present
        if other.trim_helper.is_some() {
            self.trim_helper = other.trim_helper.clone();
        }

        // Phase 224: Merge promoted_body_locals (deduplicate)
        for promoted_var in &other.promoted_body_locals {
            if !self.promoted_body_locals.contains(promoted_var) {
                self.promoted_body_locals.push(promoted_var.clone());
            }
        }

    }

    /// Phase 171-C-5: Get Trim pattern helper
    ///
    /// Returns the TrimLoopHelper if this CarrierInfo was created from Trim promotion.
    ///
    /// # Returns
    ///
    /// * `Some(&TrimLoopHelper)` - If this CarrierInfo contains Trim pattern information
    /// * `None` - If this is a regular CarrierInfo (not from Trim promotion)
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(helper) = carrier_info.trim_helper() {
    ///     eprintln!("Trim pattern detected: {}", helper.carrier_name);
    ///     eprintln!("Whitespace chars: {:?}", helper.whitespace_chars);
    /// }
    /// ```
    pub fn trim_helper(
        &self,
    ) -> Option<&crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper> {
        self.trim_helper.as_ref()
    }

    /// Phase 229/231: Resolve promoted LoopBodyLocal name to carrier JoinIR ValueId
    ///
    /// This helper centralizes the naming convention for promoted variables so that
    /// ScopeManager 実装がそれぞれ命名規約を再実装しなくて済むようにするよ。
    ///
    /// 命名規約:
    /// - DigitPos パターン: `"var"` → `"is_var"`（例: "digit_pos" → "is_digit_pos"）
    /// - Trim パターン   : `"var"` → `"is_var_match"`（例: "ch" → "is_ch_match"）
    ///
    /// # Arguments
    ///
    /// * `original_name` - 元の LoopBodyLocal 名（例: "digit_pos"）
    ///
    /// # Returns
    ///
    /// * `Some(ValueId)` - 対応する carrier の join_id が見つかった場合
    /// * `None` - promoted_body_locals に含まれない、または join_id 未設定の場合
    ///
    /// Historical note:
    /// this still uses naming conventions (`is_*`, `is_*_match`) because the
    /// retired BindingId pilot was removed in the R4 cleanup.
    pub fn resolve_promoted_join_id(&self, original_name: &str) -> Option<ValueId> {
        if !self
            .promoted_body_locals
            .contains(&original_name.to_string())
        {
            return None;
        }

        let candidates = [
            format!("is_{}", original_name),       // DigitPos pattern
            format!("is_{}_match", original_name), // Trim pattern
        ];

        for carrier_name in &candidates {
            // loop_var 自身が ConditionOnly carrier として扱われるケースは現状ほぼないが、
            // 将来の拡張に備えて loop_var_name も一応チェックしておく。
            if carrier_name == &self.loop_var_name {
                if let Some(carrier) = self.carriers.iter().find(|c| c.name == self.loop_var_name) {
                    if let Some(join_id) = carrier.join_id {
                        return Some(join_id);
                    }
                }
            }

            if let Some(carrier) = self.carriers.iter().find(|c| c.name == *carrier_name) {
                if let Some(join_id) = carrier.join_id {
                    return Some(join_id);
                }
            }
        }

        None
    }
}
