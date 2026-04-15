//! FlowBox observability tags (strict/dev only).

use crate::mir::builder::control_flow::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::lower::{CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::planner::Freeze;

#[derive(Clone, Copy)]
pub(in crate::mir::builder) enum FlowboxVia {
    Shadow,
    Release,
}

impl FlowboxVia {
    fn as_str(self) -> &'static str {
        match self {
            FlowboxVia::Shadow => "shadow",
            FlowboxVia::Release => "release",
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::mir::builder) enum FlowboxBoxKind {
    Loop,
    If2,
    BranchN,
    Seq,
    Leaf,
}

impl FlowboxBoxKind {
    fn as_str(self) -> &'static str {
        match self {
            FlowboxBoxKind::Loop => "Loop",
            FlowboxBoxKind::If2 => "If2",
            FlowboxBoxKind::BranchN => "BranchN",
            FlowboxBoxKind::Seq => "Seq",
            FlowboxBoxKind::Leaf => "Leaf",
        }
    }
}

pub(in crate::mir::builder) fn box_kind_from_core_plan(
    core_plan: &LoweredRecipe,
) -> FlowboxBoxKind {
    match core_plan {
        CorePlan::Loop(_) => FlowboxBoxKind::Loop,
        CorePlan::If(_) => FlowboxBoxKind::If2,
        CorePlan::BranchN(_) => FlowboxBoxKind::BranchN,
        CorePlan::Seq(_) => FlowboxBoxKind::Seq,
        CorePlan::Effect(_) | CorePlan::Exit(_) => FlowboxBoxKind::Leaf,
    }
}

pub(in crate::mir::builder) fn box_kind_from_facts(
    facts: Option<&CanonicalLoopFacts>,
) -> FlowboxBoxKind {
    let Some(facts) = facts else {
        return FlowboxBoxKind::Loop;
    };
    match facts.skeleton_kind {
        SkeletonKind::Loop => FlowboxBoxKind::Loop,
        SkeletonKind::If2 => FlowboxBoxKind::If2,
        SkeletonKind::BranchN => FlowboxBoxKind::BranchN,
        SkeletonKind::StraightLine => FlowboxBoxKind::Seq,
    }
}

pub(in crate::mir::builder) fn features_from_facts(
    facts: Option<&CanonicalLoopFacts>,
) -> Vec<&'static str> {
    let Some(facts) = facts else {
        return Vec::new();
    };
    let mut features = Vec::new();
    if facts.exit_usage.has_return {
        features.push("return");
    }
    if facts.exit_usage.has_break {
        features.push("break");
    }
    if facts.exit_usage.has_continue {
        features.push("continue");
    }
    if facts.exit_usage.has_unwind {
        features.push("unwind");
    }
    if facts.value_join_needed {
        features.push("value_join");
    }
    if !facts.cleanup_kinds_present.is_empty() {
        features.push("cleanup");
    }
    if facts.nested_loop {
        features.push("nested_loop");
    }
    features
}

pub(in crate::mir::builder) fn emit_flowbox_adopt_tag_from_coreplan(
    strict_or_dev: bool,
    core_plan: &LoweredRecipe,
    facts: Option<&CanonicalLoopFacts>,
    via: FlowboxVia,
) {
    emit_flowbox_adopt_tag_for_coreplan(strict_or_dev, core_plan, facts, &[], via);
}

pub(in crate::mir::builder) fn emit_flowbox_adopt_tag_for_coreplan(
    strict_or_dev: bool,
    core_plan: &LoweredRecipe,
    facts: Option<&CanonicalLoopFacts>,
    extra_features: &[&'static str],
    via: FlowboxVia,
) {
    let mut features = features_from_facts(facts);
    for feature in extra_features {
        if !features.iter().any(|f| f == feature) {
            features.push(*feature);
        }
    }
    emit_flowbox_adopt_tag(
        strict_or_dev,
        box_kind_from_core_plan(core_plan),
        &features,
        via,
    );
}

pub(in crate::mir::builder) fn emit_flowbox_adopt_tag(
    strict_or_dev: bool,
    box_kind: FlowboxBoxKind,
    features: &[&'static str],
    via: FlowboxVia,
) {
    if !strict_or_dev {
        return;
    }
    let features_csv = features.join(",");
    let ring0 = crate::runtime::get_global_ring0();
    let msg = format!(
        "[flowbox/adopt box_kind={} features={} via={}]",
        box_kind.as_str(),
        features_csv,
        via.as_str()
    );
    let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
}

pub(in crate::mir::builder) fn emit_flowbox_freeze_tag_from_facts(
    strict_or_dev: bool,
    code: &str,
    facts: Option<&CanonicalLoopFacts>,
) {
    let features = features_from_facts(facts);
    let box_kind = box_kind_from_facts(facts);
    emit_flowbox_freeze_tag(strict_or_dev, code, box_kind, &features);
}

pub(in crate::mir::builder) fn emit_flowbox_freeze_contract(
    strict_or_dev: bool,
    code: &str,
    facts: Option<&CanonicalLoopFacts>,
    message: &str,
) -> String {
    emit_flowbox_freeze_tag_from_facts(strict_or_dev, code, facts);
    Freeze::contract(message).to_string()
}

pub(in crate::mir::builder) fn emit_flowbox_freeze_tag(
    strict_or_dev: bool,
    code: &str,
    box_kind: FlowboxBoxKind,
    features: &[&'static str],
) {
    if !strict_or_dev {
        return;
    }
    let features_csv = features.join(",");
    let ring0 = crate::runtime::get_global_ring0();
    let msg = format!(
        "[flowbox/freeze code={} box_kind={} features={}]",
        code,
        box_kind.as_str(),
        features_csv
    );
    let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
}
