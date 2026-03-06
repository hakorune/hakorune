//! Phase 231: Scope manager for unified variable lookup.
//!
//! This box aggregates the environments used during JoinIR expression lowering and
//! answers two questions only: where a variable comes from, and which JoinIR value
//! currently represents it.

use super::carrier_info::CarrierInfo;
use super::condition_env::ConditionEnv;
use super::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::loop_pattern_detection::function_scope_capture::CapturedEnv;
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarScopeKind {
    LoopVar,
    Carrier,
    LoopBodyLocal,
    Captured,
}

pub trait ScopeManager {
    fn lookup(&self, name: &str) -> Option<ValueId>;
    fn scope_of(&self, name: &str) -> Option<VarScopeKind>;
}

pub struct LoopBreakScopeManager<'a> {
    pub condition_env: &'a ConditionEnv,
    pub loop_body_local_env: Option<&'a LoopBodyLocalEnv>,
    pub captured_env: Option<&'a CapturedEnv>,
    pub carrier_info: &'a CarrierInfo,
}

impl<'a> ScopeManager for LoopBreakScopeManager<'a> {
    fn lookup(&self, name: &str) -> Option<ValueId> {
        if let Some(id) = self.condition_env.get(name) {
            return Some(id);
        }

        if let Some(env) = self.loop_body_local_env {
            if let Some(id) = env.get(name) {
                return Some(id);
            }
        }

        if let Some(env) = self.captured_env {
            for var in &env.vars {
                if var.name == name {
                    return self.condition_env.get(name);
                }
            }
        }

        #[allow(deprecated)]
        {
            self.carrier_info.resolve_promoted_join_id(name)
        }
    }

    fn scope_of(&self, name: &str) -> Option<VarScopeKind> {
        if name == self.carrier_info.loop_var_name {
            return Some(VarScopeKind::LoopVar);
        }

        if self.carrier_info.carriers.iter().any(|c| c.name == name) {
            return Some(VarScopeKind::Carrier);
        }

        if let Some(env) = self.loop_body_local_env {
            if env.contains(name) {
                return Some(VarScopeKind::LoopBodyLocal);
            }
        }

        if let Some(env) = self.captured_env {
            if env.vars.iter().any(|v| v.name == name) {
                return Some(VarScopeKind::Captured);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, CarrierInit, CarrierRole, CarrierVar};
    use crate::mir::loop_pattern_detection::function_scope_capture::CapturedVar;

    #[test]
    fn test_loop_break_scope_manager_loop_var() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("i"), Some(ValueId(100)));
        assert_eq!(scope.scope_of("i"), Some(VarScopeKind::LoopVar));
    }

    #[test]
    fn test_loop_break_scope_manager_carrier() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));
        condition_env.insert("sum".to_string(), ValueId(101));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(2),
                join_id: Some(ValueId(101)),
                role: CarrierRole::LoopState,
                init: CarrierInit::FromHost,
            }],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("sum"), Some(ValueId(101)));
        assert_eq!(scope.scope_of("sum"), Some(VarScopeKind::Carrier));
    }

    #[test]
    fn test_loop_break_scope_manager_promoted_variable() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![CarrierVar {
                name: "is_digit_pos".to_string(),
                host_id: ValueId(2),
                join_id: Some(ValueId(102)),
                role: CarrierRole::ConditionOnly,
                init: CarrierInit::BoolConst(false),
            }],
            trim_helper: None,
            promoted_body_locals: vec!["digit_pos".to_string()],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("digit_pos"), Some(ValueId(102)));
    }

    #[test]
    fn test_loop_break_scope_manager_body_local() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let mut body_local_env = LoopBodyLocalEnv::new();
        body_local_env.insert("temp".to_string(), ValueId(200));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: Some(&body_local_env),
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("temp"), Some(ValueId(200)));
        assert_eq!(scope.scope_of("temp"), Some(VarScopeKind::LoopBodyLocal));
    }

    #[test]
    fn test_loop_break_scope_manager_captured() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));
        condition_env.insert("len".to_string(), ValueId(201));

        let mut captured_env =
            crate::mir::loop_pattern_detection::function_scope_capture::CapturedEnv::new();
        captured_env.add_var(CapturedVar {
            name: "len".to_string(),
            host_id: ValueId(42),
            is_immutable: true,
            kind: crate::mir::loop_pattern_detection::function_scope_capture::CapturedKind::Explicit,
        });

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: Some(&captured_env),
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("len"), Some(ValueId(201)));
        assert_eq!(scope.scope_of("len"), Some(VarScopeKind::Captured));
    }
}
