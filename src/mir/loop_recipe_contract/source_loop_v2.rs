//! One source-tree projector for the selected recursive Loop walk. Logical
//! assembly retains original execution/cleanup obligations; it grants no Home
//! After or physical execution permission by itself.
use std::{collections::BTreeMap, rc::Rc};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::builder::CallableLoopSourceBindingClassV1 as Role;
use crate::mir::resolved_semantics::{BindingRefV1, ResolvedLiteralSourceV1 as Literal,
    ResolvedBinaryOperatorV1 as Binary, ResolvedControlTransferV1 as Transfer, SourceStmtSiteV1};
pub(super) use crate::mir::resolved_semantics::home_new_prefix::{SourceLoopScanSummaryV1 as Summary,
    CheckedSourceLoopBodyV1 as Body, CheckedSourceLoopStatementV1 as Statement,
    CheckedExpressionEvaluationV1 as Evaluation, CheckedExpressionReferenceV1 as Reference,
    CurrentScalarProducerV1 as Producer, SourceLoopValueProducerV1 as SourceProducer,
    SourceLoopTextProducerV1 as TextProducer,
    SourceLoopDynamicProducerV1 as DynamicProducer};
pub(super) use crate::mir::resolved_value_profile::{expression_plan::{ExpressionSourcePlanV1 as Source, ExpressionNodeSourceV1 as Node},
    expression_projection::ExpressionLeafClassV1 as Leaf, expression_class::ExpressionValueClassV1 as Value};
use super::{ids::*, schema_v2::*, typed_schema_v2::{LoopRecipeVerifierV2, VerifiedLoopRecipeV2}};

use super::source_loop_schedule::{SourceLoopScheduleDraftV2, VerifiedSourceLoopScheduleV2, SourceLoopScheduledStepV2};
use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;

#[path = "source_loop_v2_projector.rs"]
mod projector;
use projector::Projector;
#[path = "source_loop_child.rs"]
mod child;
pub(in crate::mir::loop_recipe_contract) use child::{child_loop_boundary_v2, source_child_loop_v2};
/// The same original node and logical result. This correspondence never carries
/// a physical ID, substitute source frame or fabricated Call permission.
#[derive(Debug)]
pub(crate) struct SourceLoopValueV2 {
    pub(crate) source: Rc<Source>,
    pub(crate) node: usize,
    pub(crate) value: LoopValueKeyV1,
}
#[derive(Debug)]
pub(crate) struct SourceLoopItemV2 {
    pub(crate) site: SourceStmtSiteV1,
    pub(crate) item: LoopItemKeyV1,
}
pub(crate) struct VerifiedSourceLoopRecipeProductV2 {
    pub(crate) source: Summary,
    schedule: VerifiedSourceLoopScheduleV2,
    pub(crate) execution: Box<[super::source_loop_execution::SourceLoopEvaluationLifecycleV2]>,
    pub(super) returns: super::source_loop_execution::SourceLoopReturnCarriersV2,
    pub(crate) recipe: VerifiedLoopRecipeV2,
    pub(crate) values: Box<[SourceLoopValueV2]>,
    pub(crate) control: Box<[SourceLoopItemV2]>,
    pub(crate) bindings: BTreeMap<BindingRefV1, LoopBindingKeyV1>,
}
// Equality preserves identity of the single consuming product in source slots;
// equal reissued Recipe/frame data is never an interchangeable permission.
impl PartialEq for VerifiedSourceLoopRecipeProductV2 {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl Eq for VerifiedSourceLoopRecipeProductV2 {}
impl VerifiedSourceLoopRecipeProductV2 {
    /// Borrow the existing source transition of this exact scheduled rebind.
    pub(crate) fn text_rebind_before_v2(
        &self, frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
    ) -> Result<Option<&TextProducer>, String> {
        let mut rows = self.schedule().frames().iter().zip(self.execution.iter())
            .filter(|(scheduled, _)| matches!(scheduled.prefix(), Prefix::BindingRebind { frame: original, .. }
                if Rc::ptr_eq(original, frame)));
        let (_, execution) = rows.next().ok_or_else(|| reject("schedule-original-frame"))?;
        if rows.next().is_some() || !Rc::ptr_eq(&execution.source, frame.source()) {
            return Err(reject("schedule-original-frame"));
        }
        Ok(execution.text_carrier_transition.as_ref().map(|transition| &transition.before))
    }
    pub(crate) fn lifecycle_prefixes_v2(&self) -> Vec<Prefix> {
        fn visit(product: &VerifiedSourceLoopRecipeProductV2, block: LoopBlockKeyV1, output: &mut Vec<Prefix>) {
            for step in product.schedule().block(block).expect("verified source schedule block") {
                match step {
                    SourceLoopScheduledStepV2::Call { site, observation, .. } => output.push(Prefix::Call {
                        site: observation.site().clone(), statement: site.clone(),
                    }),
                    SourceLoopScheduledStepV2::Frame(index) => output.push(product.schedule().frames()[*index].prefix().clone()),
                    SourceLoopScheduledStepV2::OwnedArrayAppend { site, observation, .. } => {
                        let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                            source, call, candidate, ..
                        } = observation else { unreachable!("verified append observation") };
                        output.push(Prefix::OwnedArrayAppend { site: call.clone(), statement: site.clone(),
                            holder: source.clone(), candidate: candidate.clone() });
                    }
                    SourceLoopScheduledStepV2::If { then_block, else_block, .. } => {
                        visit(product, *then_block, output);
                        if let Some(block) = else_block { visit(product, *block, output); }
                    }
                    // A child Loop is one lifecycle group owned by this Loop.
                    SourceLoopScheduledStepV2::ChildLoop { product: child, .. } => output.push(Prefix::Loop {
                        site: crate::mir::resolved_semantics::OwnedExprSiteV1::new(child.source.source.owner(),
                            child.source.condition.source().nodes()[0].site().clone()),
                        product: Rc::clone(child),
                    }),
                    SourceLoopScheduledStepV2::Exit { .. } | SourceLoopScheduledStepV2::Return { .. } => {},
                }
            }
        }
        let root = &self.recipe.as_recipe().loops[0];
        let LoopConditionV2::Predicate { block, .. } = root.condition else { unreachable!("source Loop predicate") };
        let mut output = Vec::new();
        visit(self, block, &mut output); visit(self, root.body, &mut output);
        output
    }
    pub(crate) fn owned_array_append_v2(&self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1)
        -> Option<&crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1> {
        if site.owner() != self.source.source.owner() { return None; }
        source_array_append_v2(&self.source, &SourceStmtSiteV1::from_node(site.site().node().clone()))
    }
    pub(crate) fn schedule(&self) -> &VerifiedSourceLoopScheduleV2 { &self.schedule }
    /// Whether `child` is a child Loop composed by this Loop's own schedule.
    pub(crate) fn owns_child_loop_v2(&self, child: &Rc<Self>) -> bool {
        self.schedule.child_loops_v2().any(|own| Rc::ptr_eq(own, child))
    }
    /// Every checked expression evaluation retained by this Loop source
    /// product, including argument and condition subframes that may belong to
    /// lexical instance calls rather than scheduled static-call steps.
    pub(crate) fn checked_expression_evaluations_v2(
        &self,
    ) -> impl Iterator<Item = &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>
    {
        let scheduled = self.schedule.frames().iter().filter_map(|frame| match frame.prefix() {
            Prefix::BoolCondition { frame, .. } => Some(frame.evaluation()),
            Prefix::InitializerExpression { frame, .. } => Some(frame.evaluation()),
            Prefix::BindingRebind { frame, .. } => Some(frame.evaluation()),
            _ => None,
        });
        scheduled
            .chain(std::iter::once(self.source.condition.evaluation()))
            .chain(
            self.source.body_expressions.iter().flat_map(|plan| {
                plan.evaluation()
                    .into_iter()
                    .chain(plan.argument_frame().map(|frame| frame.evaluation()))
                    .chain(plan.condition_frame().map(|frame| frame.evaluation()))
            }),
        )
    }
    #[cfg(test)]
    pub(crate) fn assert_schedule_for_test(&self, input: ResolvedFunctionLoweringInputV1<'_>) {
        super::source_loop_schedule::assert_original_schedule_for_test(input, self);
    }
    pub(crate) fn owns_expression_v2(&self, source: &Rc<Source>) -> bool {
        source_returns_v2(&self.source).iter().any(|row| Rc::ptr_eq(source, &row.source))
            || Rc::ptr_eq(source, self.source.condition.source())
            || self.source.body_expressions.iter().any(|frame| Rc::ptr_eq(source, frame.source()))
    }
    /// These qualified argument frames execute inside the SAME scheduled Call,
    /// not the Loop frame emitter. Source summary verification binds their Arc,
    /// original ordinal and complete reference coverage before product issuance.
    pub(crate) fn scheduled_call_arguments_v2(&self) -> impl Iterator<Item = &crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1> {
        self.source.body_expressions.iter().filter_map(|plan| plan.argument_frame().map(|frame| frame.as_ref()))
            .filter(|frame| match frame.source().root() {
                crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, statement } =>
                    self.schedule.calls_v2().any(|step| matches!(step, SourceLoopScheduledStepV2::Call { site, observation, .. }
                        if site == statement && observation.site().owner() == call.owner()
                            && observation.site().site() == call.site()
                            && call.arguments().get(*ordinal as usize).is_some_and(|argument|
                                argument.ordinal() == *ordinal && frame.source().nodes().first().is_some_and(|root| root.site() == argument.site())))),
                _ => false,
            })
    }
    /// Exact argument-purpose frames retained by this Loop's source census.
    /// Nested conditional calls are not CallSlots, but their arguments still
    /// use these original checked frames when the BoolCondition owner emits
    /// the call effect at its source reach.
    pub(crate) fn argument_frames_v2(&self) -> impl Iterator<Item = &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>> {
        self.source.body_expressions.iter().filter_map(|plan| plan.argument_frame())
    }
    pub(crate) fn delegates_call_v2(&self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1) -> bool {
        site.owner() == self.source.source.owner() && (self.schedule.calls_v2().any(|step|
            matches!(step, SourceLoopScheduledStepV2::Call { observation, .. } if observation.site() == site))
            || self.checked_expression_evaluations_v2().any(|evaluation|
                evaluation.calls().iter().any(|call| call.site() == site))
            || self.scheduled_call_arguments_v2().any(|frame| frame.source().nodes().iter()
                .any(|node| node.site() == site.site() && matches!(node.source(), Node::MethodCall))))
    }
    pub(crate) fn owns_call_v2(&self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1) -> bool {
        site.owner() == self.source.source.owner() && (std::iter::once(self.source.condition.source())
            .chain(self.source.body_expressions.iter().map(|frame| frame.source()))
            .any(|source| source.nodes().iter().any(|node| node.site() == site.site()
                && matches!(node.source(), Node::MethodCall | Node::DirectCall)))
            || self.schedule.calls_v2().any(|step| matches!(step,
                SourceLoopScheduledStepV2::Call { observation, .. } if observation.site() == site))
            || self.schedule.blocks_array_appends_v2().any(|observation| matches!(observation,
                crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append { call, candidate, .. }
                    if call == site || candidate == site)))
    }
}
pub(super) fn source_returns_v2(source: &Summary)
    -> Vec<&Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>> {
    fn visit<'a>(body: &'a Body, output: &mut Vec<&'a Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>>) {
        for statement in &body.statements { match statement {
            Statement::Return(source) => output.push(source),
            Statement::If { then_body, else_body, .. } => {
                visit(then_body, output); if let Some(body) = else_body { visit(body, output); }
            }
            _ => {},
        } }
    }
    let mut output = Vec::new();
    if let Ok(body) = &source.control { visit(body, &mut output); }
    output
}
pub(super) fn source_call_v2<'a>(source: &'a Summary, site: &SourceStmtSiteV1)
    -> Option<&'a crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1> {
    fn visit<'a>(body: &'a Body, site: &SourceStmtSiteV1)
        -> Option<&'a crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1> {
        for statement in body.statements.iter() { match statement {
            Statement::Call { site: original, observation } if original == site => return Some(observation),
            Statement::If { then_body, else_body, .. } => {
                if let Some(call) = visit(then_body, site).or_else(|| else_body.as_ref().and_then(|body| visit(body, site))) { return Some(call); }
            }
            _ => {},
        } }
        None
    }
    visit(source.control.as_ref().ok()?, site).or_else(|| {
        std::iter::once(&source.condition)
            .chain(source.body_expressions.iter().filter_map(|frame| frame.condition_frame()))
            .flat_map(|frame| frame.calls().iter())
            .find(|call| call.statement() == site)
    })
}
pub(super) fn source_condition_call_v2(
    source: &Summary,
    observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
) -> bool {
    std::iter::once(&source.condition)
        .chain(source.body_expressions.iter().filter_map(|frame| frame.condition_frame()))
        .flat_map(|frame| frame.calls().iter())
        .any(|call| call.site() == observation.site())
}
pub(super) fn source_array_append_v2<'a>(source: &'a Summary, site: &SourceStmtSiteV1)
    -> Option<&'a crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1> {
    fn find<'a>(body: &'a Body, site: &SourceStmtSiteV1)
        -> Option<&'a crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1> {
        for statement in &body.statements {
            match statement {
                Statement::OwnedArrayAppend { site: original, observation } if original == site => return Some(observation),
                Statement::If { then_body, else_body, .. } => {
                    if let Some(row) = find(then_body, site).or_else(|| else_body.as_ref().and_then(|body| find(body, site))) {
                        return Some(row);
                    }
                }
                _ => {}
            }
        }
        None
    }
    find(source.control.as_ref().ok()?, site)
}
/// A borrowed outer Text input retains the original live acquisition. A
/// readonly role alone cannot turn an iteration-local residence into an input.
pub(super) fn is_outer_text_reference_v2(source: &Summary, proof: &Reference) -> bool {
    use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
    let Reference::Residence(read) = proof else { return false; };
    let acquisition = read.acquisition();
    read.binding().owner() == source.source.owner()
        && read.root() == acquisition.binding()
        && acquisition.source().owner() == source.source.owner()
        && acquisition.is_text_residence() && acquisition.owns_root()
        && source.source.bindings().bindings().any(|(binding, role)|
            binding == read.binding() && role == Role::ReadOnlyOperand)
        && !acquisition.source().root().statement().node().segments()
            .starts_with(source.source.site().node().segments())
        && source.condition.prior_homes().iter().filter(|home| home.binding() == read.root())
            .eq(std::iter::once(&LiveHomeV1::Text(acquisition.clone())))
}

pub(super) fn integer_operation_v2(frame: &Evaluation, node: usize) -> Option<LoopBinaryI64OpV2> {
    let mut loans = frame.operations().iter().filter(|loan| loan.node() == node);
    let loan = loans.next()?;
    if loans.next().is_some() || !Rc::ptr_eq(loan.source(), frame.source()) { return None; }
    match loan.integer_operator_v1()? {
        crate::mir::BinaryOp::Add => Some(LoopBinaryI64OpV2::Add),
        crate::mir::BinaryOp::Sub => Some(LoopBinaryI64OpV2::Sub),
        crate::mir::BinaryOp::Mul => Some(LoopBinaryI64OpV2::Mul),
        crate::mir::BinaryOp::Div => Some(LoopBinaryI64OpV2::Div),
        crate::mir::BinaryOp::Mod => Some(LoopBinaryI64OpV2::Mod),
        _ => None,
    }
}

/// Resolve only an original immutable producer already present in this product.
/// Shared by projection and independent source/schedule checking.
pub(super) fn producer_value_v2(producer: &Producer, source: &Summary, recipe: &LoopRecipeV2,
    values: &[SourceLoopValueV2], bindings: &BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    schedule: &SourceLoopScheduleDraftV2,
) -> Option<LoopValueKeyV1> {
    let source_result = |source: &Rc<Source>| values.iter()
        .find(|row| Rc::ptr_eq(&row.source, source) && row.node == 0).map(|row| row.value);
    match producer {
        Producer::Call(call) => schedule.calls_v2().find_map(|step| match step {
            SourceLoopScheduledStepV2::Call { observation, result, .. } if observation == call => *result,
            _ => None,
        }),
        Producer::BranchNormal(source) => schedule.joins_v2().find(|join| join.source == *source).map(|join| join.result),
        Producer::Initializer(frame) => source_result(frame.source()),
        Producer::BindingRebind(frame) => source_result(frame.source()),
        Producer::Integer { site, .. } => values.iter().find(|row|
            row.source.nodes().get(row.node).is_some_and(|node| node.site() == site
                && matches!(node.source(), Node::Literal(Literal::Integer(_))))) .map(|row| row.value),
        Producer::LoopAfter { product, binding } => schedule.child_loops_v2().find_map(|step| match step {
            SourceLoopScheduledStepV2::ChildLoop { product: child, outputs, .. } if Rc::ptr_eq(child, product) =>
                outputs.iter().find(|row| row.0 == *binding).map(|row| row.2),
            _ => None,
        }),
        Producer::SignedEntry(binding) => recipe.read_only_inputs.iter()
            .find(|row| bindings.get(binding) == Some(&row.binding)).map(|row| row.entry_value),
        Producer::LoopHeader(basis) => {
            if !Rc::ptr_eq(basis.source(), &source.source)
                || source.headers.get(&basis.binding()).is_none_or(|original| !Rc::ptr_eq(original, basis))
                || basis.entry_producer().class() != Value::I64 {
                return None;
            }
            let binding = *bindings.get(&basis.binding())?;
            recipe.carriers.iter().find(|row| row.owner_loop == recipe.root_loop
                && row.binding == binding && row.class == LoopValueClassV2::I64)
                .map(|row| row.entry_value)
        }
        _ => None,
    }
}
/// Borrow demand from original checked reads, including joins used only as an
/// incoming producer of a later join. No classifier or alternate flow is built.
pub(super) fn demanded_branch_joins_v2(source: &Summary)
    -> Result<Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>>, String> {
    fn visit(producer: &Producer, output: &mut Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>>) {
        if let Producer::BranchNormal(join) = producer {
            if output.iter().any(|old| old == join) { return; }
            output.push(Rc::clone(join));
            for (_, _, producer) in join.incoming() { visit(producer, output); }
        }
    }
    let mut output = Vec::new();
    for evaluation in std::iter::once(source.condition.evaluation())
        .chain(source.body_expressions.iter().filter_map(|plan| plan.evaluation())) {
        for (&node, reference) in evaluation.references() {
            if let Reference::Scalar(read) = reference {
                if let Some(original) = read.original_read() {
                    if let Producer::BranchNormal(join) = original.producer() {
                        if join.binding() != reference.binding()
                            || evaluation.class(node) != Some(Leaf::Value(Value::I64)) {
                            return Err(reject("scalar-obligation"));
                        }
                    }
                    visit(original.producer(), &mut output);
                }
            }
        }
    }
    for source in source_returns_v2(source) {
        if let Some(read) = &source.read {
            visit(read.producer(), &mut output);
        }
    }
    for carrier in source.incoming.as_ref().map_err(|reason| reject(reason))? {
        for incoming in carrier.header_incoming().iter().chain(carrier.after_incoming().iter()) {
            if matches!(incoming.edge,
                crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::LoopEntry
                    | crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::ConditionFalse) {
                continue;
            }
            if let SourceProducer::Scalar(producer) = &incoming.producer {
                visit(producer, &mut output);
            }
        }
    }
    Ok(output)
}

/// Borrow the exact Text branch joins demanded by original local Text reads
/// and carrier incoming edges. A Text Phi is never inferred from MIR types.
pub(super) fn demanded_text_branch_joins_v2(source: &Summary)
    -> Result<Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>>, String> {
    fn visit(producer: &TextProducer,
        output: &mut Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>>) {
        if let TextProducer::BranchNormal(join) = producer {
            if output.iter().any(|old| old == join) { return; }
            output.push(Rc::clone(join));
            for (_, _, producer) in join.incoming() { visit(producer, output); }
        }
    }
    let mut output = Vec::new();
    for evaluation in std::iter::once(source.condition.evaluation())
        .chain(source.body_expressions.iter().filter_map(|plan| plan.evaluation())) {
        for (&node, reference) in evaluation.references() {
            if let Reference::LoopText(read) = reference {
                if let TextProducer::BranchNormal(join) = read.producer() {
                    if join.binding() != reference.binding()
                        || evaluation.class(node) != Some(Leaf::Value(Value::String)) {
                        return Err(reject("text-branch-obligation"));
                    }
                }
                visit(read.producer(), &mut output);
            }
        }
    }
    for carrier in source.incoming.as_ref().map_err(|reason| reject(reason))? {
        for incoming in carrier.header_incoming().iter().chain(carrier.after_incoming().iter()) {
            if matches!(incoming.edge,
                crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::LoopEntry
                    | crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::ConditionFalse) {
                continue;
            }
            if let SourceProducer::Text(producer) = &incoming.producer {
                visit(producer, &mut output);
            }
        }
    }
    Ok(output)
}

pub(super) fn text_producer_value_v2(producer: &TextProducer, source: &Summary,
    recipe: &LoopRecipeV2, values: &[SourceLoopValueV2],
    bindings: &BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    schedule: &SourceLoopScheduleDraftV2,
) -> Option<LoopValueKeyV1> {
    match producer {
        TextProducer::Header(basis) => {
            if !Rc::ptr_eq(basis.source(), &source.source)
                || source.headers.get(&basis.binding()).is_none_or(|original| !Rc::ptr_eq(original, basis))
                || basis.binding().owner() != source.source.owner() {
                return None;
            }
            let binding = *bindings.get(&basis.binding())?;
            recipe.carriers.iter().find(|row| row.owner_loop == recipe.root_loop
                && row.binding == binding && row.class == LoopValueClassV2::Text)
                .map(|row| row.entry_value)
        }
        TextProducer::Residence(acquisition) => values.iter().find(|row| row.node == 0
            && row.source.owner() == source.source.owner()
            && Rc::ptr_eq(&row.source, acquisition.source()))
            .map(|row| row.value),
        TextProducer::Static { binding, definition, .. } => values.iter().find(|row|
            row.source.owner() == binding.owner()
                && row.source.nodes().get(row.node).is_some_and(|node| node.site() == definition)
                && recipe.values.get(row.value.raw() as usize)
                    .is_some_and(|value| value.class == LoopValueClassV2::Text))
            .map(|row| row.value),
        TextProducer::BranchNormal(join) => schedule.text_joins_v2()
            .find(|row| row.source == *join).map(|row| row.result),
        TextProducer::After { .. } => None,
    }
}
impl std::fmt::Debug for VerifiedSourceLoopRecipeProductV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerifiedSourceLoopRecipeProductV2").field("recipe", &self.recipe)
            .field("values", &self.values).field("control", &self.control).finish_non_exhaustive()
    }
}
pub(super) fn reject(reason: &str) -> String { format!("[freeze:contract][source-loop/v2/{reason}]") }
fn class(leaf: Option<Leaf>) -> Result<LoopValueClassV2, String> {
    match leaf {
        Some(Leaf::Value(Value::I64)) => Ok(LoopValueClassV2::I64),
        Some(Leaf::Value(Value::Dynamic)) => Ok(LoopValueClassV2::Dynamic),
        Some(Leaf::Value(Value::Bool)) => Ok(LoopValueClassV2::Bool),
        Some(Leaf::Value(Value::String)) => Ok(LoopValueClassV2::Text),
        _ => Err(reject("value-class")),
    }
}
fn verify_dynamic_add_source_v2(
    frame: &Evaluation,
    node: usize,
    binary: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
) -> Result<(), String> {
    use crate::mir::dynamic_operator_contract::{
        DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as ResultKind,
        DynamicOperatorValueClassV1 as Operand,
    };
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
    let class_at = |site| frame.source().nodes().iter().position(|row| row.site() == site)
        .and_then(|index| frame.class(index));
    if binary.operator() != Op::Add
        || frame.class(node) != Some(Leaf::Value(Value::Dynamic))
        || frame.owned_result_nodes().contains(&node)
        || class_at(binary.lhs()) != Some(Leaf::Value(Value::Dynamic))
        || class_at(binary.rhs()) != Some(Leaf::Value(Value::I64))
    {
        return Err(reject("dynamic-add-source"));
    }
    let mut loans = frame.operations().iter().filter(|loan| loan.node() == node);
    let loan = loans.next().ok_or_else(|| reject("dynamic-add-loan"))?;
    if loans.next().is_some() || !Rc::ptr_eq(loan.source(), frame.source()) {
        return Err(reject("dynamic-add-loan"));
    }
    let envelope = loan.single_envelope().ok_or_else(|| reject("dynamic-add-envelope"))?;
    let domain = envelope.domain();
    if domain.family() != Family::Add
        || domain.left() != Operand::Dynamic
        || domain.right() != Operand::I64
        || envelope.normal_result() != ResultKind::SelfContainedNonAliasingDynamicCarrier
    {
        return Err(reject("dynamic-add-domain"));
    }
    Ok(())
}
fn verify_dynamic_sub_source_v2(
    frame: &Evaluation,
    node: usize,
    binary: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
) -> Result<(), String> {
    use crate::mir::dynamic_operator_contract::{
        DynamicOperatorFamilyV1 as Family, DynamicOperatorNormalResultV1 as ResultKind,
        DynamicOperatorValueClassV1 as Operand,
    };
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
    let class_at = |site| {
        frame
            .source()
            .nodes()
            .iter()
            .position(|row| row.site() == site)
            .and_then(|index| frame.class(index))
    };
    if binary.operator() != Op::Subtract
        || frame.class(node) != Some(Leaf::Value(Value::Dynamic))
        || !frame.owned_result_nodes().contains(&node)
        || class_at(binary.lhs()) != Some(Leaf::Value(Value::Dynamic))
        || class_at(binary.rhs()) != Some(Leaf::Value(Value::I64))
    {
        return Err(reject("dynamic-sub-source"));
    }
    let mut loans = frame.operations().iter().filter(|loan| loan.node() == node);
    let loan = loans.next().ok_or_else(|| reject("dynamic-sub-loan"))?;
    if loans.next().is_some() || !Rc::ptr_eq(loan.source(), frame.source()) {
        return Err(reject("dynamic-sub-loan"));
    }
    let envelope = loan.single_envelope().ok_or_else(|| reject("dynamic-sub-envelope"))?;
    let domain = envelope.domain();
    if domain.family() != Family::Sub
        || domain.left() != Operand::Dynamic
        || domain.right() != Operand::I64
        || envelope.normal_result() != ResultKind::SelfContainedNonAliasingDynamicCarrier
    {
        return Err(reject("dynamic-sub-domain"));
    }
    Ok(())
}
pub(super) fn verify_text_add_source_v2(frame: &Evaluation, node: usize,
    binary: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1) -> Result<(), String> {
    use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family,
        DynamicOperatorNormalResultV1 as ResultKind, DynamicOperatorValueClassV1 as Class};
    if !frame.owned_result_nodes().contains(&node)
        || frame.class(node) != Some(Leaf::Value(Value::String))
        || binary.operator() != crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add {
        return Err(reject("text-add-result"));
    }
    let mut loans = frame.operations().iter().filter(|loan| loan.node() == node);
    let loan = loans.next().ok_or_else(|| reject("text-add-loan"))?;
    if loans.next().is_some() || !Rc::ptr_eq(loan.source(), frame.source()) {
        return Err(reject("text-add-loan"));
    }
    let envelope = loan.single_envelope().ok_or_else(|| reject("text-add-envelope"))?;
    let operand_class = |site| {
        frame.source().nodes().iter().position(|candidate| candidate.site() == site)
            .and_then(|index| frame.class(index)).and_then(|leaf| match leaf {
                Leaf::Value(Value::String) => Some(Class::String),
                Leaf::Value(Value::I64) => Some(Class::NormalInteger),
                Leaf::Value(Value::Dynamic) => Some(Class::Dynamic),
                _ => None,
            })
    };
    let domain = envelope.domain();
    if domain.family() != Family::Add || envelope.normal_result() != ResultKind::OwnedStringValue
        || operand_class(binary.lhs()) != Some(domain.left())
        || operand_class(binary.rhs()) != Some(domain.right())
        || !matches!((domain.left(), domain.right()),
            (Class::String, Class::String | Class::NormalInteger | Class::Dynamic)) {
        return Err(reject("text-add-domain"));
    }
    Ok(())
}
pub(crate) fn produce_source_loop_recipe_v2(input: ResolvedFunctionLoweringInputV1<'_>, source: Summary)
    -> Result<VerifiedSourceLoopRecipeProductV2, String> {
    source.validate_for_recipe_v1(input).map_err(reject)?;
    let (execution, returns) = super::source_loop_execution::verify(input, &source)?;
    let root = LoopNodeKeyV1::new(0);
    let mut projector = Projector { source: &source, schedule: SourceLoopScheduleDraftV2::default(), recipe: LoopRecipeV2 {
        root_loop: root, loops: Vec::new(), blocks: Vec::new(), items: Vec::new(), bindings: Vec::new(), values: Vec::new(),
        inputs: Vec::new(), carriers: Vec::new(), read_only_inputs: Vec::new(), exits: Vec::new(),
    }, bindings: BTreeMap::new(), values: Vec::new(), control: Vec::new() };
    for (binding, basis) in &source.headers {
        if basis.binding() != *binding || !Rc::ptr_eq(basis.source(), &source.source) { return Err(reject("carrier-basis")); }
        let class = match basis.entry_producer().class() {
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64 => LoopValueClassV2::I64,
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic => LoopValueClassV2::Dynamic,
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String => LoopValueClassV2::Text,
            _ => return Err(reject("carrier-class")),
        };
        let key = projector.binding(*binding, class)?;
        let entry_value = projector.value(class);
        projector.recipe.inputs.push(entry_value);
        projector.recipe.carriers.push(LoopRecipeCarrierV2 { key: LoopCarrierKeyV1::new(projector.recipe.carriers.len() as u32),
            owner_loop: root, binding: key, class, entry_value });
    }
    let condition_block = projector.block();
    let predicate = projector.condition(condition_block, &source.condition)?;
    let body_block = projector.block();
    projector.body(body_block, source.control.as_ref().map_err(|reason| reject(reason))?)?;
    projector.recipe.loops.push(LoopNodeV2 { key: root, parent: None,
        condition: LoopConditionV2::Predicate { block: condition_block, value: predicate }, body: body_block });
    let recipe = LoopRecipeVerifierV2::verify(projector.recipe).map_err(|reason| format!("{} {reason:?}", reject("typed")))?;
    let schedule = projector.schedule.finish(input, &source, &recipe, &execution,
        &projector.values, &projector.control, &projector.bindings)?;
    let values = projector.values.into_boxed_slice();
    let control = projector.control.into_boxed_slice();
    let bindings = projector.bindings;
    Ok(VerifiedSourceLoopRecipeProductV2 { source, schedule, execution, returns, recipe, values, control, bindings })
}
