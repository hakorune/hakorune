use rustc_driver::{run_compiler, Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::mir::{
    BasicBlockData, BorrowKind, MutBorrowKind, NonDivergingIntrinsic, Operand, Rvalue,
    StatementKind, TerminatorKind,
};
use rustc_middle::ty::TyCtxt;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const CONTRACT: &str = "rustc-semir-adapter-binding-context-mir-lifecycle-facts-v0";

#[derive(Default)]
struct MirLifecycleCallbacks {
    crate_root: PathBuf,
    bodies: Vec<BodyFacts>,
}

struct BodyFacts {
    hir_owner_reference: String,
    thir_owner_reference: String,
    rustc_owner_path: String,
    module_id: String,
    source_path: String,
    source_line: usize,
    source_column: usize,
    mir_phase: String,
    local_count: usize,
    basic_block_count: usize,
    statement_count: usize,
    terminator_count: usize,
    copy_count: usize,
    move_count: usize,
    constant_operand_count: usize,
    shared_borrow_count: usize,
    mutable_borrow_count: usize,
    fake_borrow_count: usize,
    raw_pointer_count: usize,
    drop_terminator_count: usize,
    storage_live_count: usize,
    storage_dead_count: usize,
    call_terminator_count: usize,
    direct_call_target_samples: Vec<String>,
    statement_kind_counts: BTreeMap<String, usize>,
    terminator_kind_counts: BTreeMap<String, usize>,
    drop_classification: String,
}

#[derive(Default)]
struct LifecycleCounts {
    copy_count: usize,
    move_count: usize,
    constant_operand_count: usize,
    shared_borrow_count: usize,
    mutable_borrow_count: usize,
    fake_borrow_count: usize,
    raw_pointer_count: usize,
    drop_terminator_count: usize,
    storage_live_count: usize,
    storage_dead_count: usize,
    call_terminator_count: usize,
    direct_call_target_samples: Vec<String>,
    statement_kind_counts: BTreeMap<String, usize>,
    terminator_kind_counts: BTreeMap<String, usize>,
}

impl Callbacks for MirLifecycleCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        tcx.dcx().abort_if_errors();
        let source_map = compiler.sess.source_map();
        for def_id in tcx.hir_body_owners() {
            let owner_path = tcx.def_path_str(def_id);
            if !owner_path.starts_with("binding_context::") {
                continue;
            }
            let body = tcx.optimized_mir(def_id.to_def_id());
            let source = source_loc(source_map, tcx.def_span(def_id), &self.crate_root);
            let mut counts = LifecycleCounts::default();

            for block in body.basic_blocks.iter() {
                inspect_basic_block(block, &mut counts);
            }

            self.bodies.push(BodyFacts {
                hir_owner_reference: format!("body:{}", canonical_path_from_def_path(&owner_path)),
                thir_owner_reference: format!("body:{}", canonical_path_from_def_path(&owner_path)),
                rustc_owner_path: owner_path.clone(),
                module_id: module_id_for_body_path(&owner_path),
                source_path: source.0,
                source_line: source.1,
                source_column: source.2,
                mir_phase: format!("{:?}", body.phase),
                local_count: body.local_decls.len(),
                basic_block_count: body.basic_blocks.len(),
                statement_count: body
                    .basic_blocks
                    .iter()
                    .map(|block| block.statements.len())
                    .sum(),
                terminator_count: body
                    .basic_blocks
                    .iter()
                    .filter(|block| block.terminator.is_some())
                    .count(),
                copy_count: counts.copy_count,
                move_count: counts.move_count,
                constant_operand_count: counts.constant_operand_count,
                shared_borrow_count: counts.shared_borrow_count,
                mutable_borrow_count: counts.mutable_borrow_count,
                fake_borrow_count: counts.fake_borrow_count,
                raw_pointer_count: counts.raw_pointer_count,
                drop_terminator_count: counts.drop_terminator_count,
                storage_live_count: counts.storage_live_count,
                storage_dead_count: counts.storage_dead_count,
                call_terminator_count: counts.call_terminator_count,
                direct_call_target_samples: counts.direct_call_target_samples,
                statement_kind_counts: counts.statement_kind_counts,
                terminator_kind_counts: counts.terminator_kind_counts,
                drop_classification: if counts.drop_terminator_count == 0 {
                    "no_explicit_drop_terminator_observed".to_string()
                } else {
                    "explicit_drop_terminator_observed".to_string()
                },
            });
        }
        self.bodies
            .sort_by(|a, b| a.hir_owner_reference.cmp(&b.hir_owner_reference));
        Compilation::Stop
    }
}

fn inspect_basic_block<'tcx>(block: &BasicBlockData<'tcx>, counts: &mut LifecycleCounts) {
    for statement in &block.statements {
        *counts
            .statement_kind_counts
            .entry(statement_kind_name(&statement.kind).to_string())
            .or_default() += 1;
        match &statement.kind {
            StatementKind::Assign(assign) => inspect_rvalue(&assign.1, counts),
            StatementKind::StorageLive(_) => counts.storage_live_count += 1,
            StatementKind::StorageDead(_) => counts.storage_dead_count += 1,
            StatementKind::Intrinsic(intrinsic) => inspect_intrinsic(intrinsic, counts),
            _ => {}
        }
    }

    if let Some(terminator) = &block.terminator {
        *counts
            .terminator_kind_counts
            .entry(terminator_kind_name(&terminator.kind).to_string())
            .or_default() += 1;
        inspect_terminator(&terminator.kind, counts);
    }
}

fn inspect_intrinsic<'tcx>(intrinsic: &NonDivergingIntrinsic<'tcx>, counts: &mut LifecycleCounts) {
    match intrinsic {
        NonDivergingIntrinsic::Assume(operand) => inspect_operand(operand, counts),
        NonDivergingIntrinsic::CopyNonOverlapping(copy) => {
            inspect_operand(&copy.src, counts);
            inspect_operand(&copy.dst, counts);
            inspect_operand(&copy.count, counts);
        }
    }
}

fn inspect_terminator<'tcx>(terminator: &TerminatorKind<'tcx>, counts: &mut LifecycleCounts) {
    match terminator {
        TerminatorKind::SwitchInt { discr, .. } => inspect_operand(discr, counts),
        TerminatorKind::Drop { .. } => counts.drop_terminator_count += 1,
        TerminatorKind::Call { func, args, .. } => {
            counts.call_terminator_count += 1;
            inspect_operand(func, counts);
            maybe_record_direct_call(func, counts);
            for arg in args.iter() {
                inspect_operand(&arg.node, counts);
            }
        }
        TerminatorKind::TailCall { func, args, .. } => {
            counts.call_terminator_count += 1;
            inspect_operand(func, counts);
            maybe_record_direct_call(func, counts);
            for arg in args.iter() {
                inspect_operand(&arg.node, counts);
            }
        }
        TerminatorKind::Assert { cond, .. } => inspect_operand(cond, counts),
        TerminatorKind::Yield { value, .. } => inspect_operand(value, counts),
        _ => {}
    }
}

fn inspect_rvalue<'tcx>(rvalue: &Rvalue<'tcx>, counts: &mut LifecycleCounts) {
    match rvalue {
        Rvalue::Use(operand, _) => inspect_operand(operand, counts),
        Rvalue::Repeat(operand, _) => inspect_operand(operand, counts),
        Rvalue::Ref(_, kind, _) => inspect_borrow_kind(kind, counts),
        Rvalue::RawPtr(_, _) => counts.raw_pointer_count += 1,
        Rvalue::Cast(_, operand, _) => inspect_operand(operand, counts),
        Rvalue::BinaryOp(_, operands) => {
            inspect_operand(&operands.0, counts);
            inspect_operand(&operands.1, counts);
        }
        Rvalue::UnaryOp(_, operand) => inspect_operand(operand, counts),
        Rvalue::Aggregate(_, operands) => {
            for operand in operands.iter() {
                inspect_operand(operand, counts);
            }
        }
        Rvalue::CopyForDeref(_) => counts.copy_count += 1,
        Rvalue::WrapUnsafeBinder(operand, _) => inspect_operand(operand, counts),
        Rvalue::Reborrow(_, mutability, _) => {
            if mutability.is_mut() {
                counts.mutable_borrow_count += 1;
            } else {
                counts.shared_borrow_count += 1;
            }
        }
        _ => {}
    }
}

fn inspect_operand<'tcx>(operand: &Operand<'tcx>, counts: &mut LifecycleCounts) {
    match operand {
        Operand::Copy(_) => counts.copy_count += 1,
        Operand::Move(_) => counts.move_count += 1,
        Operand::Constant(_) => counts.constant_operand_count += 1,
        Operand::RuntimeChecks(_) => {}
    }
}

fn inspect_borrow_kind(kind: &BorrowKind, counts: &mut LifecycleCounts) {
    match kind {
        BorrowKind::Shared => counts.shared_borrow_count += 1,
        BorrowKind::Fake(_) => counts.fake_borrow_count += 1,
        BorrowKind::Mut {
            kind: MutBorrowKind::Default | MutBorrowKind::TwoPhaseBorrow,
        } => counts.mutable_borrow_count += 1,
        BorrowKind::Mut {
            kind: MutBorrowKind::ClosureCapture,
        } => counts.mutable_borrow_count += 1,
    }
}

fn maybe_record_direct_call<'tcx>(func: &Operand<'tcx>, counts: &mut LifecycleCounts) {
    if counts.direct_call_target_samples.len() >= 8 {
        return;
    }
    let sample = format!("{func:?}");
    if !counts
        .direct_call_target_samples
        .iter()
        .any(|existing| existing == &sample)
    {
        counts.direct_call_target_samples.push(sample);
    }
}

fn statement_kind_name(kind: &StatementKind<'_>) -> &'static str {
    match kind {
        StatementKind::Assign(_) => "Assign",
        StatementKind::FakeRead(_) => "FakeRead",
        StatementKind::SetDiscriminant { .. } => "SetDiscriminant",
        StatementKind::StorageLive(_) => "StorageLive",
        StatementKind::StorageDead(_) => "StorageDead",
        StatementKind::PlaceMention(_) => "PlaceMention",
        StatementKind::AscribeUserType(_, _) => "AscribeUserType",
        StatementKind::Coverage(_) => "Coverage",
        StatementKind::Intrinsic(_) => "Intrinsic",
        StatementKind::ConstEvalCounter => "ConstEvalCounter",
        StatementKind::Nop => "Nop",
        StatementKind::BackwardIncompatibleDropHint { .. } => "BackwardIncompatibleDropHint",
    }
}

fn terminator_kind_name(kind: &TerminatorKind<'_>) -> &'static str {
    match kind {
        TerminatorKind::Goto { .. } => "Goto",
        TerminatorKind::SwitchInt { .. } => "SwitchInt",
        TerminatorKind::UnwindResume => "UnwindResume",
        TerminatorKind::UnwindTerminate(_) => "UnwindTerminate",
        TerminatorKind::Return => "Return",
        TerminatorKind::Unreachable => "Unreachable",
        TerminatorKind::Drop { .. } => "Drop",
        TerminatorKind::Call { .. } => "Call",
        TerminatorKind::TailCall { .. } => "TailCall",
        TerminatorKind::Assert { .. } => "Assert",
        TerminatorKind::Yield { .. } => "Yield",
        TerminatorKind::CoroutineDrop => "CoroutineDrop",
        TerminatorKind::FalseEdge { .. } => "FalseEdge",
        TerminatorKind::FalseUnwind { .. } => "FalseUnwind",
        TerminatorKind::InlineAsm { .. } => "InlineAsm",
    }
}

fn source_loc(
    source_map: &rustc_span::source_map::SourceMap,
    span: rustc_span::Span,
    crate_root: &Path,
) -> (String, usize, usize) {
    let pos = source_map.lookup_char_pos(span.lo());
    let raw_path = pos.file.name.prefer_local_unconditionally().to_string();
    (
        normalize_source_path(&raw_path, crate_root),
        pos.line,
        pos.col_display + 1,
    )
}

fn normalize_source_path(raw_path: &str, crate_root: &Path) -> String {
    let path = Path::new(raw_path);
    if path.is_absolute() {
        if let Ok(stripped) = path.strip_prefix(crate_root.parent().unwrap_or(crate_root)) {
            return stripped.to_string_lossy().replace('\\', "/");
        }
        return path
            .file_name()
            .map(|name| name.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|| "external.rs".to_string());
    }
    raw_path.replace('\\', "/")
}

fn canonical_path_from_def_path(def_path: &str) -> String {
    if def_path.is_empty() {
        "crate".to_string()
    } else {
        format!("crate::{def_path}")
    }
}

fn module_id_for_body_path(def_path: &str) -> String {
    let mut segments: Vec<&str> = def_path.split("::").filter(|s| !s.is_empty()).collect();
    while matches!(segments.last(), Some(segment) if segment.starts_with("{impl#")) {
        segments.pop();
    }
    if segments.len() >= 2
        && segments[segments.len() - 2]
            .chars()
            .next()
            .is_some_and(char::is_uppercase)
    {
        segments.pop();
        segments.pop();
    } else if matches!(segments.last(), Some(segment) if !segment.starts_with("{impl#")) {
        segments.pop();
    }
    if segments.is_empty() {
        "crate".to_string()
    } else {
        format!("crate::{}", segments.join("::"))
    }
}

fn render_json(bodies: &[BodyFacts]) -> String {
    let totals = totals(bodies);
    let mut out = String::new();
    out.push_str("{\n");
    field(&mut out, 1, "output_contract", CONTRACT, true);
    out.push_str("  \"schema_version\": 0,\n");
    field(
        &mut out,
        1,
        "kind",
        "RustcSemirBindingContextMirLifecycleFacts",
        true,
    );
    out.push_str("  \"family\": \"BindingContext\",\n");
    out.push_str("  \"bodies\": [\n");
    for (index, body) in bodies.iter().enumerate() {
        render_body(&mut out, body);
        if index + 1 != bodies.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n");
    out.push_str("  \"coverage\": {\n");
    out.push_str(&format!(
        "    \"selected_definition_count\": {},\n",
        bodies.len()
    ));
    out.push_str("    \"binding_context_family_selected\": 1,\n");
    out.push_str("    \"hir_owner_reference_used\": 1,\n");
    out.push_str("    \"thir_owner_reference_used\": 1,\n");
    out.push_str(&format!(
        "    \"copy_move_inventory_present\": {},\n",
        bool_int(totals.copy_count + totals.move_count > 0)
    ));
    out.push_str(&format!(
        "    \"borrow_inventory_present\": {},\n",
        bool_int(
            totals.shared_borrow_count
                + totals.mutable_borrow_count
                + totals.fake_borrow_count
                + totals.raw_pointer_count
                > 0
        )
    ));
    out.push_str("    \"drop_classification_present\": 1\n");
    out.push_str("  },\n");
    out.push_str("  \"totals\": {\n");
    numeric_field(&mut out, 2, "copy_count", totals.copy_count, true);
    numeric_field(&mut out, 2, "move_count", totals.move_count, true);
    numeric_field(
        &mut out,
        2,
        "constant_operand_count",
        totals.constant_operand_count,
        true,
    );
    numeric_field(
        &mut out,
        2,
        "shared_borrow_count",
        totals.shared_borrow_count,
        true,
    );
    numeric_field(
        &mut out,
        2,
        "mutable_borrow_count",
        totals.mutable_borrow_count,
        true,
    );
    numeric_field(
        &mut out,
        2,
        "fake_borrow_count",
        totals.fake_borrow_count,
        true,
    );
    numeric_field(
        &mut out,
        2,
        "raw_pointer_count",
        totals.raw_pointer_count,
        true,
    );
    numeric_field(
        &mut out,
        2,
        "drop_terminator_count",
        totals.drop_terminator_count,
        false,
    );
    out.push_str("  },\n");
    out.push_str("  \"claims\": {\n");
    out.push_str("    \"HakoLifecyclePlan_emitted\": 0,\n");
    out.push_str("    \"hako_plan_emitted\": 0,\n");
    out.push_str("    \"hako_source_emitted\": 0,\n");
    out.push_str("    \"backend_behavior_changed\": 0,\n");
    out.push_str("    \"authority_promoted\": 0\n");
    out.push_str("  }\n");
    out.push_str("}\n");
    out
}

fn totals(bodies: &[BodyFacts]) -> LifecycleCounts {
    let mut totals = LifecycleCounts::default();
    for body in bodies {
        totals.copy_count += body.copy_count;
        totals.move_count += body.move_count;
        totals.constant_operand_count += body.constant_operand_count;
        totals.shared_borrow_count += body.shared_borrow_count;
        totals.mutable_borrow_count += body.mutable_borrow_count;
        totals.fake_borrow_count += body.fake_borrow_count;
        totals.raw_pointer_count += body.raw_pointer_count;
        totals.drop_terminator_count += body.drop_terminator_count;
    }
    totals
}

fn render_body(out: &mut String, body: &BodyFacts) {
    out.push_str("    {\n");
    field(
        out,
        3,
        "hir_owner_reference",
        &body.hir_owner_reference,
        true,
    );
    field(
        out,
        3,
        "thir_owner_reference",
        &body.thir_owner_reference,
        true,
    );
    field(out, 3, "rustc_owner_path", &body.rustc_owner_path, true);
    field(out, 3, "module_id", &body.module_id, true);
    out.push_str("      \"source\": {\n");
    field(out, 4, "path", &body.source_path, true);
    out.push_str(&format!(
        "        \"start\": {{\"line\": {}, \"column\": {}}}\n",
        body.source_line, body.source_column
    ));
    out.push_str("      },\n");
    field(out, 3, "mir_phase", &body.mir_phase, true);
    numeric_field(out, 3, "local_count", body.local_count, true);
    numeric_field(out, 3, "basic_block_count", body.basic_block_count, true);
    numeric_field(out, 3, "statement_count", body.statement_count, true);
    numeric_field(out, 3, "terminator_count", body.terminator_count, true);
    numeric_field(out, 3, "copy_count", body.copy_count, true);
    numeric_field(out, 3, "move_count", body.move_count, true);
    numeric_field(
        out,
        3,
        "constant_operand_count",
        body.constant_operand_count,
        true,
    );
    numeric_field(
        out,
        3,
        "shared_borrow_count",
        body.shared_borrow_count,
        true,
    );
    numeric_field(
        out,
        3,
        "mutable_borrow_count",
        body.mutable_borrow_count,
        true,
    );
    numeric_field(out, 3, "fake_borrow_count", body.fake_borrow_count, true);
    numeric_field(out, 3, "raw_pointer_count", body.raw_pointer_count, true);
    numeric_field(
        out,
        3,
        "drop_terminator_count",
        body.drop_terminator_count,
        true,
    );
    numeric_field(out, 3, "storage_live_count", body.storage_live_count, true);
    numeric_field(out, 3, "storage_dead_count", body.storage_dead_count, true);
    numeric_field(
        out,
        3,
        "call_terminator_count",
        body.call_terminator_count,
        true,
    );
    field(
        out,
        3,
        "drop_classification",
        &body.drop_classification,
        true,
    );
    counts(
        out,
        3,
        "statement_kind_counts",
        &body.statement_kind_counts,
        true,
    );
    counts(
        out,
        3,
        "terminator_kind_counts",
        &body.terminator_kind_counts,
        true,
    );
    array(
        out,
        3,
        "direct_call_target_samples",
        &body.direct_call_target_samples,
        false,
    );
    out.push_str("    }");
}

fn field(out: &mut String, indent: usize, key: &str, value: &str, comma: bool) {
    out.push_str(&format!(
        "{}\"{}\": \"{}\"{}",
        "  ".repeat(indent),
        escape(key),
        escape(value),
        if comma { "," } else { "" }
    ));
    out.push('\n');
}

fn numeric_field(out: &mut String, indent: usize, key: &str, value: usize, comma: bool) {
    out.push_str(&format!(
        "{}\"{}\": {}{}",
        "  ".repeat(indent),
        escape(key),
        value,
        if comma { "," } else { "" }
    ));
    out.push('\n');
}

fn counts(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &BTreeMap<String, usize>,
    comma: bool,
) {
    out.push_str(&format!("{}\"{}\": {{", "  ".repeat(indent), escape(key)));
    if !values.is_empty() {
        out.push('\n');
    }
    for (index, (name, count)) in values.iter().enumerate() {
        out.push_str(&format!(
            "{}\"{}\": {}{}",
            "  ".repeat(indent + 1),
            escape(name),
            count,
            if index + 1 != values.len() { "," } else { "" }
        ));
        out.push('\n');
    }
    out.push_str(&format!(
        "{}}}{}",
        "  ".repeat(indent),
        if comma { "," } else { "" }
    ));
    out.push('\n');
}

fn array(out: &mut String, indent: usize, key: &str, values: &[String], comma: bool) {
    out.push_str(&format!("{}\"{}\": [", "  ".repeat(indent), escape(key)));
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("\"{}\"", escape(value)));
    }
    out.push(']');
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn bool_int(value: bool) -> usize {
    if value {
        1
    } else {
        0
    }
}

fn escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => escaped.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => escaped.push(ch),
        }
    }
    escaped
}

pub fn run_binding_context(inputs: &[String]) {
    if inputs.is_empty() {
        eprintln!(
            "usage: rustc-semir-adapter --binding-context-mir-lifecycle-facts <rust-source> [rustc-arg...]"
        );
        std::process::exit(2);
    }
    let input_path = PathBuf::from(&inputs[0]);
    let crate_root = input_path.canonicalize().unwrap_or(input_path.clone());
    let mut callbacks = MirLifecycleCallbacks {
        crate_root,
        bodies: Vec::new(),
    };
    let mut rustc_args = vec![
        "rustc-semir-adapter".to_string(),
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        inputs[0].clone(),
    ];
    rustc_args.extend(inputs.iter().skip(1).cloned());
    run_compiler(&rustc_args, &mut callbacks);

    if callbacks.bodies.is_empty() {
        eprintln!("rustc-semir-adapter: no BindingContext MIR bodies selected");
        std::process::exit(1);
    }
    print!("{}", render_json(&callbacks.bodies));
}
