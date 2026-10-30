//! Sole source Loop projection into logical recipe keys and scheduled frames.
use super::*;

pub(super) struct Projector<'a> {
    pub(super) source: &'a Summary,
    pub(super) schedule: SourceLoopScheduleDraftV2,
    pub(super) recipe: LoopRecipeV2,
    pub(super) bindings: BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    pub(super) values: Vec<SourceLoopValueV2>,
    pub(super) control: Vec<SourceLoopItemV2>,
}
impl Projector<'_> {
    pub(super) fn value(&mut self, class: LoopValueClassV2) -> LoopValueKeyV1 {
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
    pub(super) fn block(&mut self) -> LoopBlockKeyV1 {
        let key = LoopBlockKeyV1::new(self.recipe.blocks.len() as u32);
        self.recipe.blocks.push(LoopRecipeBlockV2 { key, owner_loop: self.recipe.root_loop, items: Vec::new() });
        self.schedule.block(key); key
    }
    pub(super) fn binding(&mut self, binding: BindingRefV1, class: LoopValueClassV2) -> Result<LoopBindingKeyV1, String> {
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
            Reference::DynamicInvocationResult(frame) => {
                let valid = class == LoopValueClassV2::Dynamic
                    && matches!(role, Role::IterationLocal | Role::ReadOnlyOperand)
                    && frame.binding() == binding && frame.is_dynamic_value()
                    && frame.source().owner() == source.owner();
                if !valid {
                    return Err(reject("dynamic-invocation-result-identity"));
                }
                self.result(frame.source())
            }
            Reference::Residence(read) => self.result(read.acquisition().source()),
            Reference::LoopText(read) => {
                let node = source.nodes().iter().position(|original| original.site() == read.read())
                    .ok_or_else(|| reject("header-text-read-node"))?;
                if class != LoopValueClassV2::Text || !read.matches_source_node(source, node) {
                    return Err(reject("header-text-identity"));
                }
                match read.producer() {
                    TextProducer::Header(basis) => {
                        if role != Role::Carrier || !Rc::ptr_eq(basis.source(), &self.source.source)
                            || basis.binding() != binding
                            || self.source.headers.get(&binding).is_none_or(|actual| !Rc::ptr_eq(actual, basis)) {
                            return Err(reject("header-text-identity"));
                        }
                        None
                    }
                    TextProducer::BranchNormal(join) => {
                        if join.binding() != binding || !join.corroborates_original() {
                            return Err(reject("text-branch-obligation"));
                        }
                        self.schedule.text_joins_v2().find(|row| row.source == *join)
                            .map(|row| row.result)
                    }
                    _ => return Err(reject("after-text-read-in-loop-body")),
                }
            }
            Reference::LoopDynamic(read) => {
                let node = source.nodes().iter().position(|original| original.site() == read.read())
                    .ok_or_else(|| reject("dynamic-read-node"))?;
                if class != LoopValueClassV2::Dynamic || !read.matches_source_node(source, node) {
                    return Err(reject("dynamic-read-identity"));
                }
                match read.producer() {
                    DynamicProducer::Initializer(_) | DynamicProducer::BindingRebind(_) =>
                        return Err(reject("dynamic-read-identity")),
                    DynamicProducer::Header(basis) => {
                        if role != Role::Carrier
                            || !Rc::ptr_eq(basis.source(), &self.source.source)
                            || basis.binding() != binding
                            || self.source.headers.get(&binding).is_none_or(|actual| !Rc::ptr_eq(actual, basis))
                        {
                            return Err(reject("dynamic-header-identity"));
                        }
                    }
                    DynamicProducer::Source(point) => {
                        if role == Role::Carrier {
                            return Err(reject("dynamic-carrier-source-producer"));
                        }
                        if let Some(site) = point.expression_site_v1() {
                            let mut produced = self.values.iter().filter(|row| {
                                row.source.owner() == source.owner()
                                    && row.source.nodes().get(row.node).is_some_and(|original| original.site() == site)
                            });
                            let row = produced.next().ok_or_else(|| reject("dynamic-source-result"))?;
                            if produced.next().is_some()
                                || self.recipe.values.get(row.value.raw() as usize).is_none_or(|value| value.class != class)
                            {
                                return Err(reject("dynamic-source-result"));
                            }
                            return Ok(row.value);
                        }
                        if role != Role::ReadOnlyOperand
                            || !point.proves_source_value_v1(binding, read.value())
                        {
                            return Err(reject("dynamic-entry-source"));
                        }
                    }
                    DynamicProducer::After { .. } => return Err(reject("dynamic-after-read-in-loop-body")),
                }
                None
            }
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
                        producer_value_v2(original.producer(), self.source, &self.recipe, &self.values,
                            &self.bindings, &self.schedule)
                    }
                    producer => producer_value_v2(producer, self.source, &self.recipe, &self.values,
                        &self.bindings, &self.schedule),
                }
            }
            _ => None,
        };
        if let Some(value) = local_result {
            return (self.recipe.values[value.raw() as usize].class == class).then_some(value).ok_or_else(|| reject("local-result-class"));
        }
        if role == Role::IterationLocal || (matches!(proof, Reference::Residence(_))
            && !(class == LoopValueClassV2::Text && is_outer_text_reference_v2(self.source, proof))) {
            return Err(reject("local-result-unavailable"));
        }
        let loop_header_read = match proof {
            Reference::Scalar(read) => matches!(read.original_read().map(|read| read.producer()), Some(Producer::LoopHeader(_))),
            Reference::LoopText(read) => matches!(read.producer(), TextProducer::Header(_)),
            Reference::LoopDynamic(read) => matches!(read.producer(), DynamicProducer::Header(_)),
            _ => false,
        };
        if role == Role::Carrier && !loop_header_read {
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
    pub(super) fn condition(&mut self, block: LoopBlockKeyV1,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>) -> Result<LoopValueKeyV1, String> {
        let projection = self.source.bool_expressions.as_ref().map_err(|_| reject("condition-projection"))?
            .iter().find(|projection| projection.matches_frame(frame)).ok_or_else(|| reject("condition-identity"))?;
        let mut call_values = BTreeMap::new();
        // A direct root call keeps the existing CallSlot owner and static
        // publication handoff. Nested calls stay inside the source Bool frame
        // so its And/Or control flow owns their exact reach and cleanup.
        for &(node, class) in projection.imports() {
            let original = frame.source().nodes().get(node).ok_or_else(|| reject("condition-import"))?;
            if !matches!(original.source(), Node::MethodCall) { continue; }
            if node != 0 || !matches!(class, LoopValueClassV2::I64 | LoopValueClassV2::Bool) {
                return Err(reject("condition-root-call-shape"));
            }
            let mut rows = frame.calls().iter().filter(|call| call.site().site() == original.site());
            let call = rows.next().ok_or_else(|| reject("condition-call-import"))?;
            let call_class = match call.result() {
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64 => LoopValueClassV2::I64,
                crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool => LoopValueClassV2::Bool,
                _ => return Err(reject("condition-call-class")),
            };
            if rows.next().is_some() || call_class != class || call.owner() != frame.source().owner()
                || call.statement() != frame.source().root().statement()
                || !call.is_expression_value() || call.result_source().is_none() {
                return Err(reject("condition-call-source"));
            }
            let result = self.value(call_class);
            let item = self.item(block, LoopRecipeItemV2::Operation {
                operation: LoopOperationV2::CallSlot { receiver: None, args: Vec::new(), result: Some(result) },
            });
            self.schedule.step(block, SourceLoopScheduledStepV2::Call {
                item,
                site: call.statement().clone(),
                observation: call.clone(),
                result: Some(result),
            });
            self.control.push(SourceLoopItemV2 { site: call.statement().clone(), item });
            if call_values.insert(node, result).is_some() { return Err(reject("condition-call-duplicate")); }
        }
        let frame_start = self.recipe.blocks[block.raw() as usize].items.len();
        let mut inputs = Vec::new();
        for &(node, import_class) in projection.imports() {
            let original = frame.source().nodes().get(node).ok_or_else(|| reject("condition-import"))?;
            let value = if matches!(original.source(), Node::MethodCall) {
                *call_values.get(&node).ok_or_else(|| reject("condition-call-value"))?
            } else {
                let proof = frame.references().get(&node).ok_or_else(|| reject("condition-import"))?;
                self.reference(block, frame.source(), proof, import_class)?
            };
            if node != 0 { self.remember(frame.source(), node, value)?; }
            inputs.push(value);
        }
        let result = self.value(LoopValueClassV2::Bool);
        self.operation(block, frame.source(), projection.operation(inputs, result)?);
        self.remember(frame.source(), 0, result)?;
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone());
        self.schedule.frame(Prefix::BoolCondition { site, frame: Rc::clone(frame) }, block,
            frame_start..self.recipe.blocks[block.raw() as usize].items.len(), result);
        Ok(result)
    }
    fn evaluation(&mut self, block: LoopBlockKeyV1, frame: &Evaluation) -> Result<LoopValueKeyV1, String> {
        if !frame.reference_coverage_is_exact() { return Err(reject("evaluation-census")); }
        let source = frame.source();
        let mut values = BTreeMap::new();
        for &node in frame.ordered_nodes() {
            if frame.transfers_node(node) { continue; }
            if frame.node_is_identity(node) { return Err(reject("identity-node")); }
            let original = source.nodes().get(node).ok_or_else(|| reject("evaluation-node"))?;
            // A Text-only element proof changes the plan class of an Array dynamic get, but the
            // loop value stays the owned Dynamic carrier; a Text add reads it as its right operand.
            let value_class = if frame.cores().get(&node).is_some_and(|core| core.is_array_dynamic_get_v1() && core.proves_text_result_v1()) {
                LoopValueClassV2::Dynamic
            } else {
                class(frame.class(node))?
            };
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
                Node::Binary(binary) if matches!(binary.operator(), Binary::Add | Binary::Subtract | Binary::Multiply | Binary::Divide | Binary::Modulo)
                    && value_class == LoopValueClassV2::I64 => {
                    let op = integer_operation_v2(frame, node).ok_or_else(|| reject("add-domain"))?;
                    let left = by_site(binary.lhs())?; let right = by_site(binary.rhs())?;
                    let result = self.value(value_class);
                    self.operation(block, source, LoopOperationV2::BinaryI64 { op, left, right, result }); result
                }
                Node::Binary(binary) if binary.operator() == Binary::Add && value_class == LoopValueClassV2::Text => {
                    verify_text_add_source_v2(frame, node, binary)?;
                    let left = by_site(binary.lhs())?; let right = by_site(binary.rhs())?;
                    let result = self.value(value_class);
                    self.operation(block, source, LoopOperationV2::TextAdd { left, right, result }); result
                }
                Node::Binary(binary) if binary.operator() == Binary::Add && value_class == LoopValueClassV2::Dynamic => {
                    verify_dynamic_add_source_v2(frame, node, binary)?;
                    let left = by_site(binary.lhs())?; let right = by_site(binary.rhs())?;
                    let result = self.value(value_class);
                    self.operation(block, source, LoopOperationV2::DynamicAdd { left, right, result }); result
                }
                Node::Binary(binary) if binary.operator() == Binary::Subtract && value_class == LoopValueClassV2::Dynamic => {
                    verify_dynamic_sub_source_v2(frame, node, binary)?;
                    let left = by_site(binary.lhs())?;
                    let right = by_site(binary.rhs())?;
                    let result = self.value(value_class);
                    self.operation(block, source, LoopOperationV2::DynamicSub { left, right, result }); result
                }
                Node::MethodCall => {
                    let mut calls = frame.calls().iter().filter(|call|
                        call.site().owner() == source.owner() && call.site().site() == original.site());
                    let call = calls.next();
                    if calls.next().is_some() { return Err(reject("evaluation-call-duplicate")); }
                    if let Some(call) = call {
                        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as ResultClass;
                        let result_class = match call.result() {
                            ResultClass::Dynamic => LoopValueClassV2::Dynamic,
                            ResultClass::I64 => LoopValueClassV2::I64,
                            ResultClass::Bool => LoopValueClassV2::Bool,
                            ResultClass::Text => LoopValueClassV2::Text,
                            _ => return Err(reject("evaluation-call-result-class")),
                        };
                        if result_class != value_class || call.owner() != source.owner()
                            || call.statement() != source.root().statement()
                            || !call.is_expression_value() || call.result_source().is_none()
                            || frame.cores().contains_key(&node) {
                            return Err(reject("evaluation-call-source"));
                        }
                        let result = self.value(value_class);
                        self.operation(block, source, LoopOperationV2::CallSlot {
                            receiver: None, args: Vec::new(), result: Some(result),
                        });
                        result
                    } else {
                        let core = frame.cores().get(&node).ok_or_else(|| reject("core-loan"))?;
                        if !core.matches_plan(source, node) { return Err(reject("core-identity")); }
                        let method = core.source().source();
                        let receiver = if core.field_array_access_v1().is_some() {
                            None
                        } else {
                            Some(by_site(method.receiver_site())?)
                        };
                        let args = method.arguments().iter().map(|arg| by_site(arg.site())).collect::<Result<Vec<_>, _>>()?;
                        let result = self.value(value_class);
                        self.operation(block, source, LoopOperationV2::CallSlot { receiver, args, result: Some(result) }); result
                    }
                }
                _ => return Err(reject("unsupported-checked-node")),
            };
            values.insert(node, value);
            self.remember(source, node, value)?;
        }
        values.get(&0).copied().ok_or_else(|| reject("evaluation-root"))
    }
    pub(super) fn body(&mut self, block: LoopBlockKeyV1, body: &Body) -> Result<(), String> {
        for statement in body.statements.iter() {
            match statement {
                Statement::Call { site, observation } => {
                    let result = if let Some((_, binding)) = observation.local_binding() {
                        if self.source.source.bindings().bindings().find(|(original, _)| *original == binding)
                            .is_none_or(|(_, role)| role != Role::IterationLocal) { return Err(reject("initializer-role")); }
                        let class = match observation.result() {
                            crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64 => Some(LoopValueClassV2::I64),
                            crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool => Some(LoopValueClassV2::Bool),
                            // The original CallReceived local owns this value;
                            // it is neither a scalar recipe value nor a carrier.
                            crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Handle
                            | crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Nullable => None,
                            _ => return Err(reject("local-result-class")),
                        };
                        class.map(|class| self.value(class))
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
                        let role = self.source.source.bindings().bindings().find(|(binding, _)| *binding == rebind.target())
                            .map(|(_, role)| role).ok_or_else(|| reject("rebind-role"))?;
                        if rebind.is_text_residence() {
                            match role {
                                Role::Carrier => {
                                    let binding = self.bindings.get(&rebind.target()).copied().ok_or_else(|| reject("text-rebind-carrier"))?;
                                    if !self.recipe.carriers.iter().any(|row|
                                        row.binding == binding && row.class == LoopValueClassV2::Text) {
                                        return Err(reject("text-rebind-carrier-class"));
                                    }
                                    let item = self.operation(block, rebind.source(), LoopOperationV2::WriteBinding { binding, value });
                                    self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                                }
                                Role::IterationLocal => {},
                                _ => return Err(reject("text-rebind-role")),
                            }
                        } else if role == Role::Carrier {
                            let binding = self.bindings.get(&rebind.target()).copied().ok_or_else(|| reject("rebind-carrier"))?;
                            if !self.recipe.carriers.iter().any(|row| row.binding == binding) { return Err(reject("rebind-readonly")); }
                            let item = self.operation(block, rebind.source(), LoopOperationV2::WriteBinding { binding, value });
                            self.control.push(SourceLoopItemV2 { site: site.clone(), item });
                        } else if role != Role::IterationLocal {
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
                        let then_value = producer_value_v2(yes, self.source, &self.recipe, &self.values,
                            &self.bindings, &self.schedule).ok_or_else(|| reject("local-result-unavailable"))?;
                        let else_value = producer_value_v2(no, self.source, &self.recipe, &self.values,
                            &self.bindings, &self.schedule).ok_or_else(|| reject("local-result-unavailable"))?;
                        if [then_value, else_value].iter().any(|value|
                            self.recipe.values[value.raw() as usize].class != LoopValueClassV2::I64) {
                            return Err(reject("local-result-class"));
                        }
                        // BranchNormal is qualified only by the original integer scalar walk.
                        let result = self.value(LoopValueClassV2::I64);
                        let phi = self.item(block, LoopRecipeItemV2::Operation {
                            operation: LoopOperationV2::BranchPhi { source_if: item, then_value, else_value, result },
                        });
                        let binding = self.source.headers.contains_key(&source.binding())
                            .then(|| self.bindings.get(&source.binding()).copied())
                            .flatten();
                        let write_item = binding.map(|binding| self.item(block, LoopRecipeItemV2::Operation {
                            operation: LoopOperationV2::WriteBinding { binding, value: result },
                        }));
                        joins.push(super::super::source_loop_schedule::SourceLoopBranchPhiV2 {
                            source, item: phi, result, binding, write_item,
                        });
                    }
                    let mut text_joins = Vec::new();
                    for source in demanded_text_branch_joins_v2(self.source)?
                        .into_iter().filter(|source| source.site() == site) {
                        if !source.corroborates_original() { return Err(reject("text-branch-obligation")); }
                        let [(_, _, yes), (_, _, no)] = source.incoming()
                            else { return Err(reject("text-branch-obligation")); };
                        let then_value = text_producer_value_v2(yes, self.source, &self.recipe,
                            &self.values, &self.bindings, &self.schedule)
                            .ok_or_else(|| reject("text-branch-value-unavailable"))?;
                        let else_value = text_producer_value_v2(no, self.source, &self.recipe,
                            &self.values, &self.bindings, &self.schedule)
                            .ok_or_else(|| reject("text-branch-value-unavailable"))?;
                        if [then_value, else_value].iter().any(|value|
                            self.recipe.values.get(value.raw() as usize)
                                .is_none_or(|value| value.class != LoopValueClassV2::Text)) {
                            return Err(reject("text-branch-value-class"));
                        }
                        let result = self.value(LoopValueClassV2::Text);
                        let phi = self.item(block, LoopRecipeItemV2::Operation {
                            operation: LoopOperationV2::BranchPhi {
                                source_if: item, then_value, else_value, result,
                            },
                        });
                        let binding = self.source.headers.contains_key(&source.binding())
                            .then(|| self.bindings.get(&source.binding()).copied()).flatten();
                        let write_item = binding.map(|binding| self.item(block,
                            LoopRecipeItemV2::Operation { operation: LoopOperationV2::WriteBinding {
                                binding, value: result,
                            } }));
                        text_joins.push(super::super::source_loop_schedule::SourceLoopTextBranchPhiV2 {
                            source, item: phi, result, binding, write_item,
                        });
                    }
                    self.schedule.step(block, SourceLoopScheduledStepV2::If { item, site: site.clone(), bundle: *bundle,
                        condition: condition_frame, then_block, else_block, joins, text_joins });
                }
                Statement::Return(source) => {
                    let start = self.recipe.blocks[block.raw() as usize].items.len();
                    let value = if let Some(read) = &source.read {
                        let reference = Reference::Scalar(
                            crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Original(read.clone()),
                        );
                        self.reference(block, &source.source, &reference, LoopValueClassV2::I64)?
                    } else if source.is_null_literal_v1() {
                        let result = self.value(LoopValueClassV2::Dynamic);
                        self.operation(block, &source.source, LoopOperationV2::ConstNull { result });
                        result
                    } else {
                        let literal = source.integer_literal_value_v1()
                            .ok_or_else(|| reject("return-value-source"))?;
                        let result = self.value(LoopValueClassV2::I64);
                        self.operation(block, &source.source, LoopOperationV2::ConstI64 {
                            result,
                            value: literal,
                        });
                        result
                    };
                    self.remember(&source.source, 0, value)?;
                    let end = self.recipe.blocks[block.raw() as usize].items.len();
                    let exit = LoopExitKeyV1::new(self.recipe.exits.len() as u32);
                    self.recipe.exits.push(LoopRecipeExitV2 { key: exit, owner_loop: self.recipe.root_loop,
                        kind: LoopExitKindV2::Return { value: Some(value) } });
                    let item = self.item(block, LoopRecipeItemV2::Exit { exit });
                    self.control.push(SourceLoopItemV2 { site: source.site.clone(), item });
                    self.schedule.step(block, SourceLoopScheduledStepV2::Return {
                        source: Rc::clone(source), operations: start..end, item, exit });
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
