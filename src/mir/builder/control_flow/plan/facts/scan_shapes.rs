//! Phase 29ai P0: Loop scan shape enums — skeleton

use crate::mir::policies::{BoundExpr, CmpOp, CondParam, CondProfile, CondSkeleton, StepExpr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LengthMethod {
    Length,
    Size,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum StepShape {
    AssignAddConst { var: String, k: i64 },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionShape {
    VarLessLength {
        idx_var: String,
        haystack_var: String,
        method: LengthMethod,
    },
    VarLessLiteral {
        idx_var: String,
        bound: i64,
    },
    VarLessEqualLengthMinusNeedle {
        idx_var: String,
        haystack_var: String,
        needle_var: String,
        haystack_method: LengthMethod,
        needle_method: LengthMethod,
    },
    VarGreaterEqualZero {
        idx_var: String,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum SplitScanShape {
    Minimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct ScanConditionObservation {
    pub condition_shape: ConditionShape,
    pub step_shape: StepShape,
    pub cond_profile: CondProfile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct ScanWithInitShape {
    pub idx_var: String,
    pub haystack_var: Option<String>,
    pub step_lit: i64,
    pub dynamic_needle: bool,
    pub needle_var: Option<String>,
}

pub(in crate::mir::builder) fn match_scan_with_init_shape(
    condition_shape: &ConditionShape,
    step_shape: &StepShape,
    cond_profile: &CondProfile,
) -> Option<ScanWithInitShape> {
    let idx_var = loop_var_from_profile(cond_profile)?;
    let step_delta = step_delta_from_profile(cond_profile)?;
    match (condition_shape, step_shape) {
        (
            ConditionShape::VarLessLength {
                haystack_var,
                // C20-D: Length/Size difference is ignored for shape matching.
                method: _,
                ..
            },
            StepShape::AssignAddConst { var, .. },
        ) if step_delta == 1 && var == &idx_var => Some(ScanWithInitShape {
            idx_var: idx_var.clone(),
            haystack_var: Some(haystack_var.clone()),
            step_lit: step_delta,
            dynamic_needle: false,
            needle_var: None,
        }),
        (
            ConditionShape::VarLessEqualLengthMinusNeedle {
                haystack_var,
                needle_var: _,
                // C20-D: Length/Size difference is ignored for shape matching.
                haystack_method: _,
                needle_method: _,
                ..
            },
            StepShape::AssignAddConst { var, .. },
        ) if step_delta == 1 && var == &idx_var => Some(ScanWithInitShape {
            idx_var: idx_var.clone(),
            haystack_var: Some(haystack_var.clone()),
            step_lit: step_delta,
            dynamic_needle: true,
            // C20-D2: minus details (needle var) are stored in CondProfile only.
            needle_var: None,
        }),
        _ => match_reverse_scan_with_init_shape(step_shape, cond_profile, step_delta),
    }
}

pub(in crate::mir::builder) fn loop_var_from_profile(cond_profile: &CondProfile) -> Option<String> {
    cond_profile.loop_var_name().map(|s| s.to_string())
}

fn match_reverse_scan_with_init_shape(
    step_shape: &StepShape,
    cond_profile: &CondProfile,
    step_delta: i64,
) -> Option<ScanWithInitShape> {
    if step_delta != -1 {
        return None;
    }
    let loop_var = extract_reverse_scan_profile(cond_profile)?;
    let StepShape::AssignAddConst { var, .. } = step_shape else {
        return None;
    };
    if var != &loop_var {
        return None;
    }

    Some(ScanWithInitShape {
        idx_var: loop_var,
        haystack_var: None,
        step_lit: step_delta,
        dynamic_needle: false,
        needle_var: None,
    })
}

fn extract_reverse_scan_profile(cond_profile: &CondProfile) -> Option<String> {
    let mut loop_var: Option<String> = None;
    let mut bound_is_zero = false;
    let mut cmp_is_ge = false;

    for param in &cond_profile.params {
        match param {
            CondParam::LoopVar(name) => loop_var = Some(name.clone()),
            CondParam::Bound(BoundExpr::LiteralI64(0)) => bound_is_zero = true,
            CondParam::Cmp(CmpOp::Ge) => cmp_is_ge = true,
            _ => {}
        }
    }

    let loop_var = loop_var?;
    if !bound_is_zero || !cmp_is_ge {
        return None;
    }
    Some(loop_var)
}

pub(in crate::mir::builder) fn step_delta_from_profile(cond_profile: &CondProfile) -> Option<i64> {
    cond_profile.params.iter().find_map(|param| {
        if let CondParam::Step(StepExpr::Delta(k)) = param {
            Some(*k)
        } else {
            None
        }
    })
}

pub(in crate::mir::builder) fn scan_condition_observation(
    condition_shape: &ConditionShape,
    step_shape: &StepShape,
) -> ScanConditionObservation {
    ScanConditionObservation {
        condition_shape: condition_shape.clone(),
        step_shape: step_shape.clone(),
        cond_profile: cond_profile_from_scan_shapes(condition_shape, step_shape),
    }
}

pub(in crate::mir::builder) fn cond_profile_from_scan_shapes(
    condition_shape: &ConditionShape,
    step_shape: &StepShape,
) -> CondProfile {
    let mut params = Vec::new();
    let bound = match condition_shape {
        ConditionShape::VarLessLength {
            idx_var,
            haystack_var,
            // C20-D: Length/Size difference is carried via CondProfile only.
            // Shape matching ignores it; BoundExpr collapses to LengthOfVar.
            method: _,
        } => {
            params.push(CondParam::LoopVar(idx_var.clone()));
            Some(BoundExpr::LengthOfVar(haystack_var.clone()))
        }
        ConditionShape::VarLessEqualLengthMinusNeedle {
            idx_var,
            haystack_var,
            needle_var,
            // C20-D: Length/Size difference is carried via CondProfile only.
            haystack_method: _,
            needle_method: _,
        } => {
            params.push(CondParam::LoopVar(idx_var.clone()));
            Some(BoundExpr::LengthMinusVar {
                haystack: haystack_var.clone(),
                needle: needle_var.clone(),
            })
        }
        ConditionShape::VarLessLiteral { idx_var, bound } => {
            params.push(CondParam::LoopVar(idx_var.clone()));
            Some(BoundExpr::LiteralI64(*bound))
        }
        ConditionShape::VarGreaterEqualZero { idx_var } => {
            params.push(CondParam::LoopVar(idx_var.clone()));
            Some(BoundExpr::LiteralI64(0))
        }
        ConditionShape::Unknown => Some(BoundExpr::Unknown),
    };

    if let Some(bound) = bound {
        params.push(CondParam::Bound(bound));
    }

    let cmp = match condition_shape {
        ConditionShape::VarLessLength { .. } => Some(CmpOp::Lt),
        ConditionShape::VarLessEqualLengthMinusNeedle { .. } => Some(CmpOp::Le),
        ConditionShape::VarLessLiteral { .. } => Some(CmpOp::Lt),
        ConditionShape::VarGreaterEqualZero { .. } => Some(CmpOp::Ge),
        ConditionShape::Unknown => None,
    };
    if let Some(cmp) = cmp {
        params.push(CondParam::Cmp(cmp));
    }

    let step = match step_shape {
        StepShape::AssignAddConst { k, .. } => StepExpr::Delta(*k),
        StepShape::Unknown => StepExpr::Unknown,
    };
    params.push(CondParam::Step(step));

    CondProfile::new(CondSkeleton::LoopCond, params)
}
