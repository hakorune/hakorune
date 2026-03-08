//! Phase P5: If Return route-shape lowering
//!
//! ## 責務（1行で表現）
//! **if-then-else で異なる値を return する route shape を Select に落とす**
//!
//! ## Route-shape examples
//! ```nyash
//! // simple route shape
//! if cond { return 10 } else { return 20 }
//!
//! // local route shape
//! if cond { x = 10 } else { x = 20 }
//! return x
//!
//! // json_shape route shape
//! if at { return v.substring(0, at) } else { return v }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - 単一関数: cond 評価 → Select(cond, then_val, else_val) → Ret

use super::{AstToJoinIrLowerer, BTreeMap, ExtractCtx, JoinFunction, JoinInst, JoinModule};
use crate::mir::join_ir::JoinIrPhase;

impl AstToJoinIrLowerer {
    /// If Return route shape の共通 lowering
    ///
    /// Phase 34-2/34-3/34-4: simple/local/json_shape 対応
    /// Phase 34-5: extract_value ベースに統一（Int/Var/Method 構造まで）
    ///
    /// - simple: `if cond { return 10 } else { return 20 }`
    /// - local: `if cond { x=10 } else { x=20 }; return x` (意味論的)
    /// - json_shape: `if at { return v.substring(0, at) } else { return v }` (Var/Method)
    ///
    /// すべて同じ JoinIR Select に正規化される
    pub(super) fn lower_if_return_pattern(
        &mut self,
        program_json: &serde_json::Value,
    ) -> JoinModule {
        // 1. Program(JSON) から defs を取得
        let defs = program_json["defs"]
            .as_array()
            .expect("Program(JSON v0) must have 'defs' array");

        // 2. 最初の関数定義を取得
        let func_def = defs
            .get(0)
            .expect("At least one function definition required");

        let func_name = func_def["name"]
            .as_str()
            .expect("Function must have 'name'");

        let params = func_def["params"]
            .as_array()
            .expect("Function must have 'params' array");

        // 3. body 内の If statement を検索（Phase 34-2/34-3 共通）
        let body = &func_def["body"]["body"];
        let if_stmt = body
            .as_array()
            .and_then(|stmts| stmts.get(0))
            .expect("Function body must have at least one statement");

        assert_eq!(
            if_stmt["type"].as_str(),
            Some("If"),
            "First statement must be If"
        );

        // 4. then/else の Return から値を抽出
        let then_stmts = if_stmt["then"]
            .as_array()
            .expect("If must have 'then' array");
        let else_stmts = if_stmt["else"]
            .as_array()
            .expect("If must have 'else' array (simple route shape)");

        let then_ret = then_stmts.get(0).expect("then branch must have Return");
        let else_ret = else_stmts.get(0).expect("else branch must have Return");

        assert_eq!(
            then_ret["type"].as_str(),
            Some("Return"),
            "then branch must be Return"
        );
        assert_eq!(
            else_ret["type"].as_str(),
            Some("Return"),
            "else branch must be Return"
        );

        // Phase 34-5: extract_value ベースの新実装
        // 5. ExtractCtx を作成し、パラメータを登録
        let func_id = self.next_func_id();

        let mut ctx = ExtractCtx::new(params.len() as u32);

        // パラメータを ExtractCtx に登録（cond, at など）
        for (i, param) in params.iter().enumerate() {
            let param_name = param
                .as_str()
                .expect("Parameter must be string")
                .to_string();
            ctx.register_param(param_name, crate::mir::ValueId(i as u32));
        }

        // Phase 34-6: cond/then/else の expr を extract_value で処理
        let (cond_var, cond_insts) = self.extract_value(&if_stmt["cond"], &mut ctx);
        let (then_var, then_insts) = self.extract_value(&then_ret["expr"], &mut ctx);
        let (else_var, else_insts) = self.extract_value(&else_ret["expr"], &mut ctx);

        // 7. Select 結果変数を割り当て
        let result_var = ctx.alloc_var();

        // 8. JoinIR 命令列を組み立て（cond → then → else → Select の順）
        let mut insts = Vec::new();

        // cond の計算命令を先頭に追加
        insts.extend(cond_insts);

        // then/else の計算命令を追加
        insts.extend(then_insts);
        insts.extend(else_insts);

        // Select: result = Select(cond, then_var, else_var)
        insts.push(JoinInst::Select {
            dst: result_var,
            cond: cond_var,
            then_val: then_var,
            else_val: else_var,
            type_hint: None, // Phase 63-3
        });

        // Ret result
        insts.push(JoinInst::Ret {
            value: Some(result_var),
        });

        let func = JoinFunction {
            id: func_id,
            name: func_name.to_string(),
            params: (0..params.len())
                .map(|i| crate::mir::ValueId(i as u32))
                .collect(),
            body: insts,
            exit_cont: None, // Phase 34-2/34-3: ルート関数なので exit_cont は None
        };

        let mut functions = BTreeMap::new();
        functions.insert(func_id, func);

        JoinModule {
            functions,
            entry: Some(func_id),
            phase: JoinIrPhase::Structured,
        }
    }
}
