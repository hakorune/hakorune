//! One source-tree projector for the selected recursive Loop walk. Logical
//! assembly retains original execution/cleanup obligations; it grants no Home
//! After or physical execution permission by itself.
use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::builder::CallableLoopSourceBindingClassV1 as Role;
use crate::mir::resolved_semantics::{BindingRefV1, ResolvedLiteralSourceV1 as Literal,
    ResolvedBinaryOperatorV1 as Binary, ResolvedControlTransferV1 as Transfer, SourceStmtSiteV1};
pub(super) use crate::mir::resolved_semantics::home_new_prefix::{SourceLoopScanSummaryV1 as Summary,
    CheckedSourceLoopBodyV1 as Body, CheckedSourceLoopStatementV1 as Statement,
    CheckedExpressionEvaluationV1 as Evaluation, CheckedExpressionReferenceV1 as Reference,
    CurrentScalarProducerV1 as Producer};
pub(super) use crate::mir::resolved_value_profile::{expression_plan::{ExpressionSourcePlanV1 as Source, ExpressionNodeSourceV1 as Node},
    expression_projection::ExpressionLeafClassV1 as Leaf, expression_class::ExpressionValueClassV1 as Value};
use super::{ids::*, schema_v2::*, typed_schema_v2::{LoopRecipeVerifierV2, VerifiedLoopRecipeV2},
    join_sig::{VerifiedLoopJoinClosureV2, VerifiedLoopAfterBinding}};

use super::source_loop_schedule::{SourceLoopScheduleDraftV2, VerifiedSourceLoopScheduleV2, SourceLoopScheduledStepV2};
use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;

type Closure = VerifiedLoopJoinClosureV2<Box<[VerifiedLoopAfterBinding<LoopValueClassV2>]>>;
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
    pub(crate) recipe: VerifiedLoopRecipeV2,
    pub(crate) closure: Closure,
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
                    SourceLoopScheduledStepV2::Exit { .. } => {},
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
    #[cfg(test)]
    pub(crate) fn assert_schedule_for_test(&self, input: ResolvedFunctionLoweringInputV1<'_>) {
        super::source_loop_schedule::assert_original_schedule_for_test(input, self);
    }
    pub(crate) fn owns_expression_v2(&self, source: &Rc<Source>) -> bool {
        Rc::ptr_eq(source, self.source.condition.source())
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
    pub(crate) fn delegates_call_v2(&self, site: &crate::mir::resolved_semantics::OwnedExprSiteV1) -> bool {
        site.owner() == self.source.source.owner() && (self.schedule.calls_v2().any(|step|
            matches!(step, SourceLoopScheduledStepV2::Call { observation, .. } if observation.site() == site))
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
    visit(source.control.as_ref().ok()?, site)
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
/// Resolve only an original immutable producer already present in this product.
/// Shared by projection and independent source/schedule checking.
pub(super) fn producer_value_v2(producer: &Producer, recipe: &LoopRecipeV2,
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
        Producer::SignedEntry(binding) => recipe.read_only_inputs.iter()
            .find(|row| bindings.get(binding) == Some(&row.binding)).map(|row| row.entry_value),
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
    Ok(output)
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
        Some(Leaf::Value(Value::Bool)) => Ok(LoopValueClassV2::Bool),
        Some(Leaf::Value(Value::String)) => Ok(LoopValueClassV2::Text),
        _ => Err(reject("value-class")),
    }
}
struct Projector<'a> {
    source: &'a Summary,
    schedule: SourceLoopScheduleDraftV2,
    recipe: LoopRecipeV2,
    bindings: BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    values: Vec<SourceLoopValueV2>,
    control: Vec<SourceLoopItemV2>,
}
impl Projector<'_> {
    fn value(&mut self, class: LoopValueClassV2) -> LoopValueKeyV1 {
        let key = LoopValueKeyV1::new(self.recipe.values.len() as u32);
        self.recipe.values.push(LoopRecipeValueV2 { key, class }); key
    }
    fn item(&mut self, block: LoopBlockKeyV1, item: LoopRecipeItemV2) -> LoopItemKeyV1 {
        let key = LoopItemKeyV1::new(self.recipe.items.len() as u32);
        self.recipe.items.push(LoopRecipeItemRowV2 { key, item });
        self.recipe.blocks[block.raw() as usize].items.push(key); key
    }
    fn operation(&mut self, block: LoopBlockKeyV1, source: &Rc<Source>, operation: LoopOperationV2) -> LoopItemKeyV1 {
        let item = self.item(block, LoopRecipeItemV2::Operation { operation });
        self.schedule.operation(item, source);
        item
    }
    fn block(&mut self) -> LoopBlockKeyV1 {
        let key = LoopBlockKeyV1::new(self.recipe.blocks.len() as u32);
        self.recipe.blocks.push(LoopRecipeBlockV2 { key, owner_loop: self.recipe.root_loop, items: Vec::new() });
        self.schedule.block(key); key
    }
    fn binding(&mut self, binding: BindingRefV1, class: LoopValueClassV2) -> Result<LoopBindingKeyV1, String> {
        if let Some(key) = self.bindings.get(&binding) {
            return (self.recipe.bindings[key.raw() as usize].class == class).then_some(*key).ok_or_else(|| reject("binding-class"));
        }
        let key = LoopBindingKeyV1::new(self.recipe.bindings.len() as u32);
        self.recipe.bindings.push(LoopRecipeBindingV2 { key, label: format!("source:{binding:?}"), class });
        self.bindings.insert(binding, key); Ok(key)
    }
    fn remember(&mut self, source: &Rc<Source>, node: usize, value: LoopValueKeyV1) -> Result<(), String> {
        if self.values.iter().any(|row| Rc::ptr_eq(&row.source, source) && row.node == node) {
            return Err(reject("duplicate-node-result"));
        }
        self.values.push(SourceLoopValueV2 { source: Rc::clone(source), node, value }); Ok(())
    }
    fn result(&self, source: &Rc<Source>) -> Option<LoopValueKeyV1> {
        self.values.iter().find(|row| Rc::ptr_eq(&row.source, source) && row.node == 0).map(|row| row.value)
    }
    fn reference(&mut self, block: LoopBlockKeyV1, source: &Rc<Source>, proof: &Reference, class: LoopValueClassV2) -> Result<LoopValueKeyV1, String> {
        let binding = proof.binding();
        let role = self.source.source.bindings().bindings().find_map(|(actual, role)| (actual == binding).then_some(role))
            .ok_or_else(|| reject("reference-role"))?;
        let local_result = match proof {
            Reference::Residence(read) => self.result(read.acquisition().source()),
            Reference::Scalar(read) => {
                let original = read.original_read().ok_or_else(|| reject("scalar-obligation"))?;
                match original.producer() {
                    Producer::LoopHeader(basis) => {
                        if role != Role::Carrier || !Rc::ptr_eq(basis.source(), &self.source.source)
                            || self.source.headers.get(&binding).is_none_or(|actual| !Rc::ptr_eq(actual, basis)) {
                            return Err(reject("header-identity"));
                        }
                        None
                    }
                    Producer::BranchNormal(join) => {
                        if !join.corroborates_original() || join.binding() != binding {
                            return Err(reject("scalar-obligation"));
                        }
                        producer_value_v2(original.producer(), &self.recipe, &self.values,
                            &self.bindings, &self.schedule)
                    }
                    producer => producer_value_v2(producer, &self.recipe, &self.values,
                        &self.bindings, &self.schedule),
                }
            }
            _ => None,
        };
        if let Some(value) = local_result {
            return (self.recipe.values[value.raw() as usize].class == class).then_some(value).ok_or_else(|| reject("local-result-class"));
        }
        if role == Role::IterationLocal || matches!(proof, Reference::Residence(_)) {
            return Err(reject("local-result-unavailable"));
        }
        if role == Role::Carrier && !matches!(proof,
            Reference::Scalar(read) if matches!(read.original_read().map(|read| read.producer()), Some(Producer::LoopHeader(_)))) {
            return Err(reject("carrier-read-producer"));
        }
        let key = self.binding(binding, class)?;
        if role == Role::ReadOnlyOperand && !self.recipe.read_only_inputs.iter().any(|row| row.binding == key) {
            let entry_value = self.value(class);
            self.recipe.inputs.push(entry_value);
            self.recipe.read_only_inputs.push(LoopRecipeReadOnlyInputV2 {
                owner_loop: self.recipe.root_loop, binding: key, class, entry_value,
            });
        }
        let result = self.value(class);
        self.operation(block, source, LoopOperationV2::ReadBinding { binding: key, result }); Ok(result)
    }
    fn condition(&mut self, block: LoopBlockKeyV1,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>) -> Result<LoopValueKeyV1, String> {
        let start = self.recipe.blocks[block.raw() as usize].items.len();
        let projection = self.source.bool_expressions.as_ref().map_err(|_| reject("condition-projection"))?
            .iter().find(|projection| projection.matches_frame(frame)).ok_or_else(|| reject("condition-identity"))?;
        let mut inputs = Vec::new();
        for &(node, class) in projection.imports() {
            let proof = frame.references().get(&node).ok_or_else(|| reject("condition-import"))?;
            let value = self.reference(block, frame.source(), proof, class)?;
            if node != 0 { self.remember(frame.source(), node, value)?; }
            inputs.push(value);
        }
        let result = self.value(LoopValueClassV2::Bool);
        self.operation(block, frame.source(), projection.operation(inputs, result)?);
        self.remember(frame.source(), 0, result)?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone());
        self.schedule.frame(Prefix::BoolCondition { site, frame: Rc::clone(frame) }, block,
            start..self.recipe.blocks[block.raw() as usize].items.len(), result);
        Ok(result)
    }
    fn evaluation(&mut self, block: LoopBlockKeyV1, frame: &Evaluation) -> Result<LoopValueKeyV1, String> {
        if !frame.reference_coverage_is_exact() || !frame.calls().is_empty() { return Err(reject("evaluation-census")); }
        let source = frame.source();
        let mut values = BTreeMap::new();
        for &node in frame.ordered_nodes() {
            if frame.transfers_node(node) || frame.node_is_identity(node) { return Err(reject("transferred-or-identity-node")); }
            let original = source.nodes().get(node).ok_or_else(|| reject("evaluation-node"))?;
            let value_class = class(frame.class(node))?;
            let by_site = |site: &crate::mir::resolved_semantics::SourceExprSiteV1| -> Result<LoopValueKeyV1, String> {
                source.nodes().iter().position(|row| row.site() == site).and_then(|node| values.get(&node).copied())
                    .ok_or_else(|| reject("original-child-order"))
            };
            let value = match original.source() {
                Node::Reference(_) => self.reference(block, source, frame.references().get(&node).ok_or_else(|| reject("reference-proof"))?, value_class)?,
                Node::Literal(Literal::Integer(value)) if value_class == LoopValueClassV2::I64 => {
                    let result = self.value(value_class); self.operation(block, source, LoopOperationV2::ConstI64 { result, value: *value }); result
                }
                Node::Literal(Literal::String(value)) if value_class == LoopValueClassV2::Text => {
                    let result = self.value(value_class); self.operation(block, source, LoopOperationV2::ConstText { result, value: value.to_string() }); result
                }
                Node::Binary(binary) if binary.operator() == Binary::Add && value_class == LoopValueClassV2::I64 => {
                    use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family, DynamicOperatorValueClassV1 as Operand};
                    let mut loans = frame.operations().iter().filter(|loan| loan.node() == node);
                    let loan = loans.next().ok_or_else(|| reject("add-loan"))?;
                    let domain = loan.single_envelope().ok_or_else(|| reject("add-domain"))?.domain();
                    if loans.next().is_some() || !Rc::ptr_eq(loan.source(), source) || domain.family() != Family::Add
                        || domain.left() != Operand::NormalInteger || domain.right() != Operand::NormalInteger {
                        return Err(reject("add-identity"));
                    }
                    let left = by_site(binary.lhs())?; let right = by_site(binary.rhs())?;
                    let result = self.value(value_class);
                    self.operation(block, source, LoopOperationV2::BinaryI64 { op: LoopBinaryI64OpV2::Add, left, right, result }); result
                }
                Node::MethodCall => {
                    let core = frame.cores().get(&node).ok_or_else(|| reject("core-loan"))?;
                    if !core.matches_plan(source, node) { return Err(reject("core-identity")); }
                    let method = core.source().source();
                    let receiver = Some(by_site(method.receiver_site())?);
                    let args = method.arguments().iter().map(|arg| by_site(arg.site())).collect::<Result<Vec<_>, _>>()?;
                    let result = self.value(value_class);
                    self.operation(block, source, LoopOperationV2::CallSlot { receiver, args, result: Some(result) }); result
                }
                _ => return Err(reject("unsupported-checked-node")),
            };
            values.insert(node, value);
            self.remember(source, node, value)?;
        }
        values.get(&0).copied().ok_or_else(|| reject("evaluation-root"))
    }
    fn body(&mut self, block: LoopBlockKeyV1, body: &Body) -> Result<(), String> {
        for statement in body.statements.iter() {
            match statement {
                Statement::Call { site, observation } => {
                    let result = if let Some((_, binding)) = observation.local_binding() {
                        if self.source.source.bindings().bindings().find(|(original, _)| *original == binding)
                            .is_none_or(|(_, role)| role != Role::IterationLocal) { return Err(reject("initializer-role")); }
                        let class = match observation.result() {
                            crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64 => LoopValueClassV2::I64,
                            crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool => LoopValueClassV2::Bool,
                            _ => return Err(reject("local-result-class")),
                        };
                        Some(self.value(class))
                    } else { None };
                    let item = self.item(block, LoopRecipeItemV2::Operation {
                        operation: LoopOperationV2::CallSlot { receiver: None, args: Vec::new(), result },
                    });
                    self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                    self.schedule.step(block, SourceLoopScheduledStepV2::Call {
                        item, site: site.clone(), observation: observation.clone(), result,
                    });
                }
                Statement::OwnedArrayAppend { site, observation } => {
                    // A unit effect slot delegates receiver/candidate/cleanup
                    // to the original owned-array packet. No synthetic value,
                    // scalar class or second expression evaluator is issued.
                    let item = self.item(block, LoopRecipeItemV2::Operation {
                        operation: LoopOperationV2::CallSlot { receiver: None, args: Vec::new(), result: None },
                    });
                    self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                    self.schedule.step(block, SourceLoopScheduledStepV2::OwnedArrayAppend {
                        item, site: site.clone(), observation: observation.clone(),
                    });
                }
                Statement::Expression { site, frame } => {
                    let start = self.recipe.blocks[block.raw() as usize].items.len();
                    let prefix_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone());
                    if let Some(initializer) = frame.initializer_frame() {
                        if self.source.source.bindings().bindings().find(|(binding, _)| *binding == initializer.binding())
                            .is_none_or(|(_, role)| role != Role::IterationLocal) { return Err(reject("initializer-role")); }
                        let result = self.evaluation(block, initializer.evaluation())?;
                        self.schedule.frame(Prefix::InitializerExpression { site: prefix_site, frame: Rc::clone(initializer) },
                            block, start..self.recipe.blocks[block.raw() as usize].items.len(), result);
                    } else if let Some(rebind) = frame.rebind_frame() {
                        let value = self.evaluation(block, rebind.evaluation())?;
                        if rebind.is_text_residence() {
                            if self.source.source.bindings().bindings().find(|(binding, _)| *binding == rebind.target())
                                .is_none_or(|(_, role)| role != Role::IterationLocal) { return Err(reject("text-rebind-role")); }
                        } else if self.source.source.bindings().bindings().find(|(binding, _)| *binding == rebind.target())
                            .is_some_and(|(_, role)| role == Role::Carrier) {
                            let binding = self.bindings.get(&rebind.target()).copied().ok_or_else(|| reject("rebind-carrier"))?;
                            if !self.recipe.carriers.iter().any(|row| row.binding == binding) { return Err(reject("rebind-readonly")); }
                            let item = self.operation(block, rebind.source(), LoopOperationV2::WriteBinding { binding, value });
                            self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                        } else if self.source.source.bindings().bindings().find(|(binding, _)| *binding == rebind.target())
                            .is_none_or(|(_, role)| role != Role::IterationLocal) {
                            return Err(reject("rebind-readonly"));
                        }
                        self.schedule.frame(Prefix::BindingRebind { site: prefix_site, frame: Rc::clone(rebind) },
                            block, start..self.recipe.blocks[block.raw() as usize].items.len(), value);
                    } else { return Err(reject("statement-execution-family")); }
                }
                Statement::If { site, bundle, condition, then_body, else_body } => {
                    let condition_frame = self.schedule.frame_count();
                    let condition = self.condition(block, condition)?;
                    let then_block = self.block();
                    let item = self.item(block, LoopRecipeItemV2::If { condition, then_block, else_block: None });
                    self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                    self.body(then_block, then_body)?;
                    if let Some(body) = else_body {
                        let else_block = self.block();
                        let LoopRecipeItemV2::If { else_block: slot, .. } = &mut self.recipe.items[item.raw() as usize].item else { unreachable!() };
                        *slot = Some(else_block); self.body(else_block, body)?;
                    }
                    let LoopRecipeItemV2::If { else_block, .. } = self.recipe.items[item.raw() as usize].item else { unreachable!() };
                    let mut joins = Vec::new();
                    for source in demanded_branch_joins_v2(self.source)?.into_iter().filter(|source| source.site() == site) {
                        if !source.corroborates_original() { return Err(reject("scalar-obligation")); }
                        let [(_, _, yes), (_, _, no)] = source.incoming() else { return Err(reject("scalar-obligation")); };
                        let then_value = producer_value_v2(yes, &self.recipe, &self.values, &self.bindings,
                            &self.schedule).ok_or_else(|| reject("local-result-unavailable"))?;
                        let else_value = producer_value_v2(no, &self.recipe, &self.values, &self.bindings,
                            &self.schedule).ok_or_else(|| reject("local-result-unavailable"))?;
                        if [then_value, else_value].iter().any(|value|
                            self.recipe.values[value.raw() as usize].class != LoopValueClassV2::I64) {
                            return Err(reject("local-result-class"));
                        }
                        // BranchNormal is qualified only by the original integer scalar walk.
                        let result = self.value(LoopValueClassV2::I64);
                        let phi = self.item(block, LoopRecipeItemV2::Operation {
                            operation: LoopOperationV2::BranchPhi { source_if: item, then_value, else_value, result },
                        });
                        joins.push(super::source_loop_schedule::SourceLoopBranchPhiV2 { source, item: phi, result });
                    }
                    self.schedule.step(block, SourceLoopScheduledStepV2::If { item, site: site.clone(), bundle: *bundle,
                        condition: condition_frame, then_block, else_block, joins });
                }
                Statement::Exit { site, record, port } => {
                    let root = self.recipe.root_loop;
                    let kind = match record.transfer() {
                        Transfer::Break { .. } => LoopExitKindV2::Break { target_loop: root },
                        Transfer::Continue { .. } => LoopExitKindV2::Continue { target_loop: root },
                        _ => return Err(reject("exit-family")),
                    };
                    let exit = LoopExitKeyV1::new(self.recipe.exits.len() as u32);
                    self.recipe.exits.push(LoopRecipeExitV2 { key: exit, owner_loop: root, kind });
                    let item = self.item(block, LoopRecipeItemV2::Exit { exit });
                    self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                    self.schedule.step(block, SourceLoopScheduledStepV2::Exit { item, site: site.clone(),
                        record: *record, port: *port, exit });
                }
            }
        }
        Ok(())
    }
}

pub(crate) fn produce_source_loop_recipe_v2(input: ResolvedFunctionLoweringInputV1<'_>, source: Summary)
    -> Result<VerifiedSourceLoopRecipeProductV2, String> {
    source.validate_for_recipe_v1(input).map_err(reject)?;
    let execution = super::source_loop_execution::verify(input, &source)?;
    let root = LoopNodeKeyV1::new(0);
    let mut projector = Projector { source: &source, schedule: SourceLoopScheduleDraftV2::default(), recipe: LoopRecipeV2 {
        root_loop: root, loops: Vec::new(), blocks: Vec::new(), items: Vec::new(), bindings: Vec::new(), values: Vec::new(),
        inputs: Vec::new(), carriers: Vec::new(), read_only_inputs: Vec::new(), exits: Vec::new(),
    }, bindings: BTreeMap::new(), values: Vec::new(), control: Vec::new() };
    for (binding, basis) in &source.headers {
        if basis.binding() != *binding || !Rc::ptr_eq(basis.source(), &source.source) { return Err(reject("carrier-basis")); }
        let key = projector.binding(*binding, LoopValueClassV2::I64)?;
        let entry_value = projector.value(LoopValueClassV2::I64);
        projector.recipe.inputs.push(entry_value);
        projector.recipe.carriers.push(LoopRecipeCarrierV2 { key: LoopCarrierKeyV1::new(projector.recipe.carriers.len() as u32),
            owner_loop: root, binding: key, class: LoopValueClassV2::I64, entry_value });
    }
    let condition_block = projector.block();
    let predicate = projector.condition(condition_block, &source.condition)?;
    let body_block = projector.block();
    projector.body(body_block, source.control.as_ref().map_err(|reason| reject(reason))?)?;
    projector.recipe.loops.push(LoopNodeV2 { key: root, parent: None,
        condition: LoopConditionV2::Predicate { block: condition_block, value: predicate }, body: body_block });
    let recipe = LoopRecipeVerifierV2::verify(projector.recipe).map_err(|reason| format!("{} {reason:?}", reject("typed")))?;
    let closure = super::join_sig::issue_root_carrier_join_closure_v2(&recipe)
        .map_err(|reason| format!("{} {reason:?}", reject("join")))?;
    validate_incoming(&source, &recipe, &closure, &projector.values, &projector.control, &projector.bindings)?;
    let schedule = projector.schedule.finish(input, &source, &recipe, &execution,
        &projector.values, &projector.control, &projector.bindings)?;
    let values = projector.values.into_boxed_slice();
    let control = projector.control.into_boxed_slice();
    let bindings = projector.bindings;
    Ok(VerifiedSourceLoopRecipeProductV2 { source, schedule, execution, recipe, closure, values, control, bindings })
}

/// Match every original incoming edge to the SAME JoinSig transfer and result.
/// A carrier entry key is JoinSig's logical Header token, never a permission to
/// substitute the pre-loop runtime value for a physical Header Phi.
fn validate_incoming(source: &Summary, recipe: &VerifiedLoopRecipeV2, closure: &Closure,
    values: &[SourceLoopValueV2], control: &[SourceLoopItemV2],
    bindings: &BTreeMap<BindingRefV1, LoopBindingKeyV1>,
) -> Result<(), String> {
    let view = closure.logical_transfer_view().map_err(|reason| format!("{} {reason:?}", reject("transfer-view")))?;
    let incoming = source.incoming.as_ref().map_err(|reason| reject(reason))?;
    let expected: BTreeSet<_> = source.source.bindings().bindings()
        .filter_map(|(binding, role)| (role == crate::mir::builder::CallableLoopSourceBindingClassV1::Carrier).then_some(binding)).collect();
    let mut actual = BTreeSet::new();
    let mut logical = BTreeSet::new();
    if expected != source.headers.keys().copied().collect()
        || incoming.len() != expected.len() || incoming.len() != recipe.as_recipe().carriers.len()
        || view.afters().len() != incoming.len() {
        return Err(reject("incoming-carrier-census"));
    }
    for row in incoming.iter() {
        let binding = row.basis.binding();
        if !actual.insert(binding) || source.headers.get(&binding).is_none_or(|basis| !Rc::ptr_eq(basis, &row.basis)) {
            return Err(reject("incoming-carrier-basis"));
        }
        let key = bindings.get(&binding).ok_or_else(|| reject("incoming-binding"))?;
        if !logical.insert(*key) { return Err(reject("incoming-logical-binding-duplicate")); }
    }
    if actual != expected || logical != recipe.as_recipe().carriers.iter().filter_map(|row|
        (row.owner_loop == recipe.root_loop()).then_some(row.binding)).collect() {
        return Err(reject("incoming-carrier-membership"));
    }
    for source_row in incoming.iter() {
        let binding = bindings.get(&source_row.basis.binding()).ok_or_else(|| reject("incoming-binding"))?;
        let carrier = recipe.as_recipe().carriers.iter().find(|row| row.binding == *binding)
            .ok_or_else(|| reject("incoming-carrier"))?;
        if !view.afters().iter().any(|row| row.binding() == *binding && row.class() == carrier.class) {
            return Err(reject("incoming-after-identity"));
        }
        for original in source_row.header_incoming.iter().chain(source_row.after_incoming.iter()) {
            let expected = match &original.producer {
                Producer::LoopHeader(basis) if Rc::ptr_eq(basis, &source_row.basis) => carrier.entry_value,
                producer if producer == source_row.basis.entry_producer() => carrier.entry_value,
                Producer::Initializer(frame) => values.iter().find(|row| row.node == 0 && Rc::ptr_eq(&row.source, frame.source()))
                    .map(|row| row.value).ok_or_else(|| reject("incoming-initializer-result"))?,
                Producer::BindingRebind(frame) => values.iter().find(|row| row.node == 0 && Rc::ptr_eq(&row.source, frame.source()))
                    .map(|row| row.value).ok_or_else(|| reject("incoming-rebind-result"))?,
                _ => return Err(reject("incoming-producer")),
            };
            with_incoming_payload_v2(recipe, closure, control, &original.edge, |payload|
                validate_payload_value_v2(payload, *binding, expected, carrier.class, incoming.len()))?;
        }
    }
    Ok(())
}

/// Borrow one exact executable JoinSig transfer; summary edges never execute.
fn with_incoming_payload_v2<R>(recipe: &VerifiedLoopRecipeV2, closure: &Closure,
    control: &[SourceLoopItemV2], edge: &crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1,
    read: impl FnOnce(&[super::join_sig::LoopJoinPayloadV2]) -> Result<R, String>,
) -> Result<R, String> {
    use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
    use super::join_sig::{LoopJoinEdgeRoleV1 as Role, LoopJoinBranchArmTransferRefV2 as Arm};
    let view = closure.logical_transfer_view().map_err(|reason| format!("{} {reason:?}", reject("transfer-view")))?;
    let payload = |edge: &Edge| -> Result<&[super::join_sig::LoopJoinPayloadV2], String> {
        let role = match edge {
            Edge::LoopEntry => Role::Enter,
            Edge::ConditionFalse => Role::PredicateFalse,
            Edge::Backedge => Role::Backedge,
            Edge::Break(_) => Role::Break,
            Edge::Continue(_) => Role::Continue,
            _ => return Err(reject("incoming-edge-kind")),
        };
        if let Edge::Break(site) | Edge::Continue(site) = edge {
            let mut items = control.iter().filter(|row| &row.site == site);
            let item = items.next().ok_or_else(|| reject("incoming-exit-source"))?;
            if items.next().is_some() { return Err(reject("incoming-exit-duplicate")); }
            let mut exits = view.branches().iter().flat_map(|branch| [branch.then_arm, branch.else_arm])
                .filter_map(|arm| match arm {
                    Arm::Exit(exit) if exit.exit_item == item.item && exit.role == role => Some(exit.payload),
                    _ => None,
                });
            let payload = exits.next().ok_or_else(|| reject("incoming-exit-transfer"))?;
            if exits.next().is_some() { return Err(reject("incoming-exit-transfer-duplicate")); }
            Ok(payload)
        } else {
            let mut edges = view.boundaries().iter().filter(|row| row.loop_key == recipe.root_loop() && row.role == role);
            let row = edges.next().ok_or_else(|| reject("incoming-boundary"))?;
            if edges.next().is_some() { return Err(reject("incoming-boundary-duplicate")); }
            Ok(row.payload)
        }
    };
    read(payload(edge)?)
}

fn validate_payload_value_v2(payload: &[super::join_sig::LoopJoinPayloadV2],
    binding: LoopBindingKeyV1, expected: LoopValueKeyV1, class: LoopValueClassV2, count: usize,
) -> Result<(), String> {
    if payload.len() != count { return Err(reject("incoming-payload-census")); }
    let mut rows = payload.iter().filter(|row| row.binding == binding);
    let row = rows.next().ok_or_else(|| reject("incoming-payload-binding"))?;
    if rows.next().is_some() || row.value != expected || row.class != class {
        return Err(reject("incoming-payload-value"));
    }
    Ok(())
}

impl VerifiedSourceLoopRecipeProductV2 {
    pub(in crate::mir) fn validate_incoming_value_v2(&self,
        edge: &crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1,
        source_binding: BindingRefV1, value: LoopValueKeyV1,
    ) -> Result<(), String> {
        let binding = *self.bindings.get(&source_binding).ok_or_else(|| reject("incoming-binding"))?;
        let carrier = self.recipe.as_recipe().carriers.iter().find(|row| row.binding == binding)
            .ok_or_else(|| reject("incoming-carrier"))?;
        with_incoming_payload_v2(&self.recipe, &self.closure, &self.control, edge, |payload|
            validate_payload_value_v2(payload, binding, value, carrier.class, self.source.headers.len()))
    }
}
