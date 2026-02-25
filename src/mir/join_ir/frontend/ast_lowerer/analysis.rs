use super::{AstToJoinIrLowerer, HashSet, JoinModule};
use crate::mir::join_ir::frontend::func_meta::{JoinFuncMeta, JoinFuncMetaMap};

impl AstToJoinIrLowerer {
    /// Phase 40-1で実装予定: ループ内if文の変数追跡
    ///
    /// # Purpose
    ///
    /// array_ext.filter等のif-in-loopパターンで、ループ内で修正される変数を
    /// 追跡し、ループ出口PHI生成に使用する。
    ///
    /// # Implementation Plan (Phase 40-1)
    ///
    /// ## Input
    /// - `loop_body`: ループ本体AST（JSON v0形式）
    /// - `loop_vars`: ループで使用される変数（Header PHIで定義）
    ///
    /// ## Output
    /// - `HashSet<String>`: if分岐内で修正された変数名セット
    ///
    /// ## Logic
    /// 1. Recursive AST walk (helper: `extract_assigned_vars_from_body`)
    /// 2. Detect assignments in if branches only
    /// 3. Filter for variables in `loop_vars` (loop-carried variables)
    /// 4. Return set of modified variable names
    ///
    /// ## Integration Point
    /// - Call from: `lower_loop_case_a_simple()` or similar loop lowering
    /// - Use output for: Loop exit PHI generation in `create_exit_function()`
    ///
    /// # Example
    ///
    /// ```nyash,ignore
    /// local out = new ArrayBox()  // loop_vars = {out, i}
    /// local i = 0
    /// loop(i < n) {
    ///   if fn(arr[i]) {           // ← この中の代入を検出
    ///     out.push(arr[i])        // ← out修正検出
    ///   }
    ///   i = i + 1
    /// }
    /// // Result: extract_if_in_loop_modified_vars() = {out}
    /// // → Loop exit PHI: phi out_exit = (out_header, out_loop_modified)
    /// ```
    ///
    /// # Replaces (Phase 40-1削除対象)
    ///
    /// - `if_phi::collect_assigned_vars()` (32 lines)
    ///   - Current callsites: loop_builder.rs:1069, 1075
    ///   - この関数実装でcallsites削除可能
    ///
    /// # See Also
    ///
    /// - Design: `docs/.../phase-39-if-phi-level2/joinir_extension_design.md`
    /// - A/B test: array_ext.filter (Primary representative function)
    ///
    /// # TODO(Phase 40-1)
    ///
    /// ```rust,ignore
    /// fn extract_if_in_loop_modified_vars(
    ///     &mut self,
    ///     loop_body: &serde_json::Value,
    ///     loop_vars: &HashSet<String>,
    /// ) -> HashSet<String> {
    ///     // Step 1: Recursive AST walk
    ///     let all_assigned = self.extract_assigned_vars_from_body(loop_body);
    ///
    ///     // Step 2: Filter for if-statement assignments only
    ///     let if_assigned = all_assigned.iter()
    ///         .filter(|var| self.is_assigned_in_if_branch(loop_body, var))
    ///         .cloned()
    ///         .collect::<HashSet<_>>();
    ///
    ///     // Step 3: Filter for loop-carried variables
    ///     if_assigned.intersection(loop_vars).cloned().collect()
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn extract_if_in_loop_modified_vars(
        &mut self,
        loop_body: &serde_json::Value,
        loop_vars: &HashSet<String>,
    ) -> HashSet<String> {
        // Step 1: Recursive AST walk to collect all assigned variables
        let all_assigned = self.extract_assigned_vars_from_body(loop_body);

        // Step 2: Filter for if-statement assignments only
        let if_assigned = self.extract_if_assigned_vars(loop_body);

        // Step 3: Filter for loop-carried variables
        // Return intersection of (if_assigned ∩ loop_vars)
        if_assigned
            .intersection(loop_vars)
            .filter(|var| all_assigned.contains(*var))
            .cloned()
            .collect()
    }

    /// Phase 40-1: if文内での代入変数抽出
    ///
    /// # Purpose
    ///
    /// loop body内のif文でのみ代入される変数を抽出する。
    /// これはloop exit PHI生成に必要。
    pub fn extract_if_assigned_vars(
        &mut self,
        body: &serde_json::Value,
    ) -> std::collections::HashSet<String> {
        use std::collections::HashSet;
        let mut if_assigned = HashSet::new();

        // Handle array of statements
        if let Some(stmts) = body.as_array() {
            for stmt in stmts {
                if stmt.get("type").and_then(|t| t.as_str()) == Some("If") {
                    // Extract assignments from then/else branches
                    if let Some(then_body) = stmt.get("then") {
                        if_assigned.extend(self.extract_assigned_vars_from_body(then_body));
                    }
                    if let Some(else_body) = stmt.get("else") {
                        if_assigned.extend(self.extract_assigned_vars_from_body(else_body));
                    }
                }
                // Recursive: nested loops
                else if stmt.get("type").and_then(|t| t.as_str()) == Some("Loop") {
                    if let Some(loop_body) = stmt.get("body") {
                        if_assigned.extend(self.extract_if_assigned_vars(loop_body));
                    }
                }
            }
        }
        // Handle Block node
        else if let Some(stmts) = body.get("body").and_then(|b| b.as_array()) {
            for stmt in stmts {
                if stmt.get("type").and_then(|t| t.as_str()) == Some("If") {
                    if let Some(then_body) = stmt.get("then") {
                        if_assigned.extend(self.extract_assigned_vars_from_body(then_body));
                    }
                    if let Some(else_body) = stmt.get("else") {
                        if_assigned.extend(self.extract_assigned_vars_from_body(else_body));
                    }
                } else if stmt.get("type").and_then(|t| t.as_str()) == Some("Loop") {
                    if let Some(loop_body) = stmt.get("body") {
                        if_assigned.extend(self.extract_if_assigned_vars(loop_body));
                    }
                }
            }
        }

        if_assigned
    }

    /// Phase 40-1で実装予定: 再帰的AST走査（代入検出）
    ///
    /// # Purpose
    ///
    /// AST bodyを再帰的に走査し、代入文を検出する。
    ///
    /// # Implementation Plan
    ///
    /// ## Recursive Descent
    /// - Handle: "Local" assignments (`local x = ...` or `x = ...`)
    /// - Handle: Nested blocks (`{ ... }`)
    /// - Handle: If/Loop bodies (recursive call)
    ///
    /// ## Return
    /// - `HashSet<String>`: 代入された変数名全て
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///   "type": "Block",
    ///   "body": [
    ///     {"type": "Local", "name": "x", "expr": ...},  // x assigned
    ///     {"type": "If", "cond": ..., "then": [
    ///       {"type": "Local", "name": "y", "expr": ...}  // y assigned
    ///     ]}
    ///   ]
    /// }
    /// ```
    /// Result: {x, y}
    ///
    #[allow(dead_code)]
    pub fn extract_assigned_vars_from_body(
        &mut self,
        body: &serde_json::Value,
    ) -> std::collections::HashSet<String> {
        use std::collections::HashSet;
        let mut assigned = HashSet::new();

        // Handle array of statements
        if let Some(stmts) = body.as_array() {
            for stmt in stmts {
                self.extract_assigned_vars_from_stmt(stmt, &mut assigned);
            }
        }
        // Handle single statement (Block node)
        else if let Some(stmts) = body.get("body").and_then(|b| b.as_array()) {
            for stmt in stmts {
                self.extract_assigned_vars_from_stmt(stmt, &mut assigned);
            }
        }

        assigned
    }

    /// Phase 40-1: 再帰的AST走査ヘルパー（単一文処理）
    ///
    /// # Purpose
    ///
    /// 単一のAST文を処理し、代入された変数を収集する。
    pub(crate) fn extract_assigned_vars_from_stmt(
        &mut self,
        stmt: &serde_json::Value,
        assigned: &mut std::collections::HashSet<String>,
    ) {
        match stmt.get("type").and_then(|t| t.as_str()) {
            Some("Local") => {
                // local x = ... または x = ...
                if let Some(name) = stmt.get("name").and_then(|n| n.as_str()) {
                    assigned.insert(name.to_string());
                }
            }
            Some("If") => {
                // if 文の then/else 分岐を再帰処理
                if let Some(then_body) = stmt.get("then") {
                    assigned.extend(self.extract_assigned_vars_from_body(then_body));
                }
                if let Some(else_body) = stmt.get("else") {
                    assigned.extend(self.extract_assigned_vars_from_body(else_body));
                }
            }
            Some("Loop") => {
                // loop 文の body を再帰処理
                if let Some(loop_body) = stmt.get("body") {
                    assigned.extend(self.extract_assigned_vars_from_body(loop_body));
                }
            }
            Some("Block") => {
                // { ... } ブロックの body を再帰処理
                if let Some(block_body) = stmt.get("body") {
                    assigned.extend(self.extract_assigned_vars_from_body(block_body));
                }
            }
            _ => {
                // その他の文は無視（Return, Call, etc.）
            }
        }
    }

    /// Phase 40-1実験用: array_ext.filter パターン専用lowering
    ///
    /// 通常の Simple パターンループ lowering に加えて、
    /// if-in-loop modified varsをJoinFuncMetaMapとして返す。
    ///
    /// # Returns
    /// - `JoinModule`: 通常のJoinIR module
    /// - `JoinFuncMetaMap`: loop_step関数のif_modified_vars情報
    ///
    /// # Phase 40-1専用
    /// この関数はPhase 40-1 A/Bテスト専用。
    /// 本番パスでは使わない（新しいloop_patterns::simple::lower()を使う）。
    ///
    /// # Phase 85 Note
    /// loop_patterns_old.rs削除に伴い、loop_patterns::simple::lower()に委譲
    #[allow(dead_code)]
    pub fn lower_loop_with_if_meta(
        &mut self,
        program_json: &serde_json::Value,
    ) -> (JoinModule, JoinFuncMetaMap) {
        // 1. 通常のJoinModule生成（新パターンシステムに委譲）
        use super::loop_patterns;
        let module = loop_patterns::simple::lower(self, program_json)
            .expect("Simple pattern lowering failed in lower_loop_with_if_meta");

        // 2. loop body ASTからif-in-loop modified varsを抽出
        let loop_body = self.extract_loop_body_from_program(program_json);
        let loop_vars = self.get_loop_carried_vars_from_program(program_json);
        let if_modified = self.extract_if_in_loop_modified_vars(&loop_body, &loop_vars);

        // 3. JoinFuncMetaMap組み立て
        // loop_step関数のIDを特定（通常は2番目の関数、名前に"loop_step"を含む）
        let mut meta_map = JoinFuncMetaMap::new();
        if let Some((loop_step_id, _)) = module
            .functions
            .iter()
            .find(|(_, func)| func.name.contains("loop_step"))
        {
            meta_map.insert(
                *loop_step_id,
                JoinFuncMeta {
                    if_modified_vars: if if_modified.is_empty() {
                        None
                    } else {
                        Some(if_modified)
                    },
                    ..Default::default()
                },
            );
        }

        (module, meta_map)
    }

    /// loop body ASTを抽出するヘルパー
    ///
    /// # Purpose
    /// Program → defs[0] → body → Loop statement → body のパスでloop bodyを取得
    pub fn extract_loop_body_from_program(
        &self,
        program_json: &serde_json::Value,
    ) -> serde_json::Value {
        // Program → defs[0] → body → Loop statement → body
        program_json["defs"][0]["body"]
            .as_array()
            .and_then(|stmts| stmts.iter().find(|s| s["type"] == "Loop"))
            .and_then(|loop_stmt| loop_stmt.get("body"))
            .cloned()
            .unwrap_or(serde_json::Value::Null)
    }

    /// loop-carried varsを抽出するヘルパー
    ///
    /// # Purpose
    /// loop前に宣言された変数を収集（簡易実装）
    /// より正確な実装はPhase 40-2で拡張
    pub fn get_loop_carried_vars_from_program(
        &self,
        program_json: &serde_json::Value,
    ) -> HashSet<String> {
        let mut vars = HashSet::new();
        if let Some(body) = program_json["defs"][0]["body"].as_array() {
            for stmt in body {
                if stmt["type"] == "Local" {
                    if let Some(name) = stmt["name"].as_str() {
                        vars.insert(name.to_string());
                    }
                }
                if stmt["type"] == "Loop" {
                    break; // loop以降は無視
                }
            }
        }
        vars
    }

    /// Phase 85: Loop body に Break があるかチェック（再帰的探索）
    ///
    /// ループパターン検出（loop_frontend_binding）で使用される。
    /// ネストした If/Block 内の Break ステートメントも検出する。
    ///
    /// # Arguments
    /// * `loop_body` - ループ本体のステートメント配列
    ///
    /// # Returns
    /// ループ内にBreakがあればtrue
    pub(crate) fn has_break_in_loop_body(loop_body: &[serde_json::Value]) -> bool {
        Self::has_break_recursive(loop_body)
    }

    /// 再帰的に Break を探索
    fn has_break_recursive(stmts: &[serde_json::Value]) -> bool {
        for stmt in stmts {
            match stmt["type"].as_str() {
                Some("Break") => return true,
                Some("If") => {
                    // then 分岐を再帰探索
                    if let Some(then_body) = stmt["then"].as_array() {
                        if Self::has_break_recursive(then_body) {
                            return true;
                        }
                    }
                    // else 分岐を再帰探索
                    if let Some(else_body) = stmt["else"].as_array() {
                        if Self::has_break_recursive(else_body) {
                            return true;
                        }
                    }
                }
                Some("Block") => {
                    // Block 内を再帰探索
                    if let Some(block_body) = stmt["body"].as_array() {
                        if Self::has_break_recursive(block_body) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Phase 85: Loop body に Continue があるかチェック（再帰的探索）
    ///
    /// ループパターン検出（loop_frontend_binding）で使用される。
    /// ネストした If/Block 内の Continue ステートメントも検出する。
    ///
    /// # Arguments
    /// * `loop_body` - ループ本体のステートメント配列
    ///
    /// # Returns
    /// ループ内にContinueがあればtrue
    pub(crate) fn has_continue_in_loop_body(loop_body: &[serde_json::Value]) -> bool {
        Self::has_continue_recursive(loop_body)
    }

    /// 再帰的に Continue を探索
    fn has_continue_recursive(stmts: &[serde_json::Value]) -> bool {
        for stmt in stmts {
            match stmt["type"].as_str() {
                Some("Continue") => return true,
                Some("If") => {
                    // then 分岐を再帰探索
                    if let Some(then_body) = stmt["then"].as_array() {
                        if Self::has_continue_recursive(then_body) {
                            return true;
                        }
                    }
                    // else 分岐を再帰探索
                    if let Some(else_body) = stmt["else"].as_array() {
                        if Self::has_continue_recursive(else_body) {
                            return true;
                        }
                    }
                }
                Some("Block") => {
                    // Block 内を再帰探索
                    if let Some(block_body) = stmt["body"].as_array() {
                        if Self::has_continue_recursive(block_body) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Phase 89: Loop body に Return があるかチェック（再帰的探索）
    ///
    /// ループパターン検出（loop_frontend_binding）で使用される。
    /// ネストした If/Block 内の Return ステートメントも検出する（loop-internal early return）。
    ///
    /// # Arguments
    /// * `loop_body` - ループ本体のステートメント配列
    ///
    /// # Returns
    /// ループ内にReturnがあればtrue
    pub(crate) fn has_return_in_loop_body(loop_body: &[serde_json::Value]) -> bool {
        Self::has_return_recursive(loop_body)
    }

    /// 再帰的に Return を探索
    fn has_return_recursive(stmts: &[serde_json::Value]) -> bool {
        for stmt in stmts {
            match stmt["type"].as_str() {
                Some("Return") => return true,
                Some("If") => {
                    // then 分岐を再帰探索
                    if let Some(then_body) = stmt["then"].as_array() {
                        if Self::has_return_recursive(then_body) {
                            return true;
                        }
                    }
                    // else 分岐を再帰探索
                    if let Some(else_body) = stmt["else"].as_array() {
                        if Self::has_return_recursive(else_body) {
                            return true;
                        }
                    }
                }
                Some("Block") => {
                    // Block 内を再帰探索
                    if let Some(block_body) = stmt["body"].as_array() {
                        if Self::has_return_recursive(block_body) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }
}
