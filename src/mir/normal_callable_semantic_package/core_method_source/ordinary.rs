//! Original ordinary Core source issuance; no Loop or physical capability.
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ClosedOriginalCoreSourceV1;
use crate::mir::resolved_semantics::{
    CoreMethodInstanceTargetIssuerV1, ResolverCoreMethodCallableContractIssuerV1,
    VerifiedResolverOrdinaryCoreMethodSourceV1,
};
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct OrdinaryCoreSourceCallV1 {
    operands: Rc<ClosedOriginalCoreSourceV1>,
    source: VerifiedResolverOrdinaryCoreMethodSourceV1,
}
impl OrdinaryCoreSourceCallV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn source(
        &self,
    ) -> &VerifiedResolverOrdinaryCoreMethodSourceV1 {
        &self.source
    }
    pub(in crate::mir::normal_callable_semantic_package) fn operands(
        &self,
    ) -> &Rc<ClosedOriginalCoreSourceV1> {
        &self.operands
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_ordinary_core_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    operands: &Rc<ClosedOriginalCoreSourceV1>,
) -> Result<Rc<OrdinaryCoreSourceCallV1>, String> {
    let reject = || "[freeze:contract][ordinary-core/source-identity]".to_owned();
    let method = operands.method();
    if input.owner() != method.owner()
        || !input
            .function()
            .method_call_source_v1(method.site())
            .is_some_and(|original| std::sync::Arc::ptr_eq(original, method))
    {
        return Err(reject());
    }
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| reject())?;
    use crate::mir::core_method_op::CoreMethodOp as Op;
    let op = operands.manifest().row().op;
    let map = matches!(op, Op::MapHas | Op::MapKeys);
    if map != operands.field_map().is_some() {
        return Err(reject());
    }
    let count = operands.field_array_count_v1();
    if (op == Op::ArrayLen) != (operands.receiver_core().is_some() ^ count.is_some())
        || (op != Op::ArrayLen && (operands.receiver_core().is_some() || count.is_some()))
        || operands.receiver_core().is_some_and(|inner|
            inner.manifest().row().op != Op::MapKeys || inner.field_map().is_none()
                || inner.method().owner() != method.owner()
                || inner.method().result_site() != method.receiver_site()
                || !input.function().method_call_source_v1(inner.method().site())
                    .is_some_and(|original| std::sync::Arc::ptr_eq(original, inner.method()))) {
        return Err(reject());
    }
    if let Some(count) = count {
        if !count.is_count() || !count.matches_original_call_v1(input)
            || count.call().owner() != method.owner() || count.call().site() != method.site()
            || count.receiver().site() != method.receiver_site()
            || count.count_manifest_v1().is_none_or(|original|
                !std::ptr::eq(original.row(), operands.manifest().row()) || original.arity() != operands.manifest().arity()) {
            return Err(reject());
        }
        count.recheck_census(input.owner(), &std::collections::BTreeMap::new())?;
    }
    let mut issuer = match op {
        Op::MapHas => CoreMethodInstanceTargetIssuerV1::map_text_presence(operands.manifest().brand()),
        Op::MapKeys => CoreMethodInstanceTargetIssuerV1::map_text_keys(operands.manifest().brand()),
        Op::ArrayLen => CoreMethodInstanceTargetIssuerV1::array_count(operands.manifest().brand()),
        _ => CoreMethodInstanceTargetIssuerV1::string_box_text(operands.manifest().brand()),
    }
    .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    let target = issuer
        .issue(operands.manifest())
        .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    if target.checked_code_point_law_v1().is_none() && !target.checked_map_presence_law_v1()
        && !target.checked_text_keys_law_v1() && !target.checked_array_count_law_v1() {
        return Err("[freeze:contract][ordinary-core/semantic-law]".to_owned());
    }
    let source =
        ResolverCoreMethodCallableContractIssuerV1::issue_ordinary_source(&ledger, method, target)
            .map_err(|error| format!("[freeze:contract][ordinary-core/source] {error:?}"))?;
    if !std::sync::Arc::ptr_eq(source.source(), method) {
        return Err(reject());
    }
    Ok(Rc::new(OrdinaryCoreSourceCallV1 {
        source,
        operands: Rc::clone(operands),
    }))
}

/// A loan of one canonical source receipt inside the SAME original plan.
/// This is neither an executable call packet nor an acquisition/entry grant.
#[derive(Debug, Clone)]
pub(crate) struct ExpressionCoreSourceLoanV1 {
    plan: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    node: usize,
    original: Rc<OrdinaryCoreSourceCallV1>,
    text_entry_receiver_read:
        Option<crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1>,
}
impl PartialEq for ExpressionCoreSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && Rc::ptr_eq(&self.plan, &other.plan)
            && Rc::ptr_eq(&self.original, &other.original)
            && self.text_entry_receiver_read == other.text_entry_receiver_read
    }
}
impl Eq for ExpressionCoreSourceLoanV1 {}
impl ExpressionCoreSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn borrows_original_v1(&self, original: &Rc<OrdinaryCoreSourceCallV1>) -> bool {
        Rc::ptr_eq(&self.original, original)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_receiver_result_v1(&self, inner: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &inner.plan)
            && self.source().source().receiver_site() == inner.source().source().result_site()
            && self.original.operands.receiver_core().is_some_and(|original|
                Rc::ptr_eq(original, inner.original.operands()))
            && inner.source().target().checked_text_keys_law_v1()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn lend(
        input: ResolvedFunctionLoweringInputV1<'_>,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        node: usize,
        original: &Rc<OrdinaryCoreSourceCallV1>,
    ) -> Result<Self, String> {
        let loan = Self {
            plan: Rc::clone(plan),
            node,
            original: Rc::clone(original),
            text_entry_receiver_read: None,
        };
        if input.owner() != plan.owner()
            || !loan.matches_plan(plan, node)
            || !input
                .function()
                .method_call_source_v1(loan.source().source().site())
                .is_some_and(|method| std::sync::Arc::ptr_eq(method, loan.source().source()))
        {
            return Err("[freeze:contract][expression-core/source-identity]".to_owned());
        }
        Ok(loan)
    }
    pub(crate) fn source(&self) -> &VerifiedResolverOrdinaryCoreMethodSourceV1 {
        self.original.source()
    }
    pub(crate) fn node(&self) -> usize {
        self.node
    }
    pub(crate) fn plan(
        &self,
    ) -> &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn dynamic_i64_operand_contains(
        &self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> bool {
        self.original.operands().dynamic_i64_operand_contains(site)
    }
    pub(crate) fn text_entry_receiver_read(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1>
    {
        self.text_entry_receiver_read.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn with_text_entry_receiver_read_v1(
        mut self,
        read: Option<
            crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1,
        >,
    ) -> Result<Self, String> {
        use crate::mir::resolved_semantics::{
            ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
        };
        let reject = || "[freeze:contract][expression-core/entry-read-identity]".to_owned();
        if let Some(read) = &read {
            let source = self.source().source();
            let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
                source.receiver()
            else {
                return Err(reject());
            };
            if self.source().target().checked_code_point_law_v1().is_none()
                || !Rc::ptr_eq(read.source_plan(), &self.plan)
                || self.plan.nodes().get(read.node()).is_none_or(|node|
                    node.site() != source.receiver_site()
                        || !matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                            ResolvedLexicalRefV1::Local(actual)) if *actual == binding && binding == read.binding())
                ) {
                return Err(reject());
            }
        }
        self.text_entry_receiver_read = read;
        Ok(self)
    }
    pub(crate) fn matches_plan(
        &self,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        node: usize,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        let entry_read_matches = self.text_entry_receiver_read.as_ref().is_none_or(|read| {
            let source = self.source().source();
            self.source().target().checked_code_point_law_v1().is_some()
                && Rc::ptr_eq(read.source_plan(), plan)
                && plan.nodes().get(read.node()).is_some_and(|row|
                    row.site() == source.receiver_site()
                        && matches!(row.source(), Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                            if *binding == read.binding())
                )
        });
        Rc::ptr_eq(plan, &self.plan)
            && node == self.node
            && entry_read_matches
            && self.source().source().owner() == plan.owner()
            && plan.nodes().get(node).is_some_and(|row| {
                row.site() == self.source().source().site()
                    && matches!(row.source(), Node::MethodCall)
            })
            && (self.source().target().checked_code_point_law_v1().is_some()
                || self.field_map().is_some_and(|field| {
                    (self.source().target().checked_map_presence_law_v1()
                        || self.source().target().checked_text_keys_law_v1()) && field.matches_plan(plan)
                })
                || (self.source().target().checked_array_count_law_v1()
                    && self.field_array_count_v1().is_some_and(|count| count.matches_expression_receiver_v1(plan)))
                || (self.source().target().checked_array_count_law_v1()
                    && self.original.operands.receiver_core().is_some_and(|inner|
                        inner.method().result_site() == self.source().source().receiver_site()
                            && plan.nodes().iter().filter(|node| node.site() == inner.method().site()
                                && matches!(node.source(), Node::MethodCall)).count() == 1
                            && inner.field_map().is_some_and(|field| field.matches_plan(plan)))))
            && std::iter::once(self.source().source().receiver_site())
                .chain(
                    self.source()
                        .source()
                        .arguments()
                        .iter()
                        .map(|arg| arg.site()),
                )
                .all(|site| plan.nodes().iter().filter(|row| row.site() == site).count() == 1)
    }
    pub(crate) fn result_class(
        &self,
    ) -> Option<crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1> {
        self.source().target().expression_result_class_v1()
    }
    pub(crate) fn field_array_count_v1(&self) -> Option<&Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>> {
        self.original.operands.field_array_count_v1()
    }
    pub(crate) fn field_map(&self) -> Option<&FieldMapReceiverSourceV1> {
        self.original.operands.field_map().map(Rc::as_ref)
    }
    pub(crate) fn owns_result(&self) -> bool {
        self.source().target().result()
            == crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller
    }
}

/// Receiver projection kept inside the SAME Core call, never a Map SSA value.
/// The immutable all-assignment source proves a sole Map Birth writer. Map
/// entry mutations are independent; presence grants no entry class or borrow.
#[derive(Debug)]
pub(crate) struct FieldMapReceiverSourceV1 {
    source: Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::DeclaredFieldValueSourceV1>,
    receiver: crate::mir::resolved_semantics::BindingRefV1,
    receiver_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    field_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    name: Box<str>,
    root: Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ExpressionFieldRootSourceV1>,
    birth_key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    birth_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    birth_store: crate::mir::normal_callable_semantic_package::ConstructionStoreV1,
}
impl FieldMapReceiverSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        method: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        claims: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewFieldWriteClaimsV1,
        receiver_proof: (
            crate::mir::resolved_semantics::BindingRefV1,
            &crate::parser::ParserOrdinaryBoxSourceRowV1,
        ),
        entry: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1,
    ) -> Result<Self, String> {
        use crate::mir::resolved_semantics::{
            BindingKindV1, BodyExpressionShapeV1 as Shape, BodyMeReceiverV1,
        };
        let reject = || "[freeze:contract][ordinary-core/source-identity]".to_owned();
        if caller.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || input.owner() != method.owner()
        {
            return Err(reject());
        }
        let shape = input.body_shape().ok_or_else(reject)?;
        let Some(Shape::FieldAccess { object, field, .. }) =
            shape.expression_shape(method.receiver_site())
        else {
            return Err(reject());
        };
        let Some(Shape::Me {
            receiver: BodyMeReceiverV1::Lexical(receiver),
            ..
        }) = shape.expression_shape(object)
        else {
            return Err(reject());
        };
        if input
            .function()
            .binding(*receiver)
            .is_none_or(|row| row.kind() != BindingKindV1::Receiver)
        {
            return Err(reject());
        }
        let source = claims
            .value_source(caller.owner(), field)
            .ok_or_else(reject)?
            .as_ref()
            .map_err(|_| reject())?;
        let writers: Vec<_> = source.writers().collect();
        let store = source
            .birth_store(constructors, 0)
            .map_err(|_| reject())?
            .ok_or_else(reject)?;
        if writers.len() != 1
            || !writers[0].in_birth()
            || source.declared_type_name().is_some()
            || store.mode() != Ok(crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize)
            || !matches!(store.rhs(), crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::ProviderConstruction {
                object: None, class, .. } if class.as_ref() == "MapBox")
        {
            return Err(reject());
        }
        let root = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ExpressionFieldRootSourceV1::issue_entry_v1(
            constructors, receiver_proof, entry, input.owner(), *receiver, source.field())?;
        let birth_key = constructors
            .published_birth_key(writers[0].birth_source().ok_or_else(reject)?)
            .ok_or_else(reject)?
            .clone();
        Ok(Self {
            source: Rc::clone(source),
            receiver: *receiver,
            receiver_site: object.clone(),
            field_site: method.receiver_site().clone(),
            name: field.clone(),
            root,
            birth_key,
            birth_owner: writers[0].owner(),
            birth_store: store.clone(),
        })
    }
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 {
        self.source.field()
    }
    pub(crate) fn receiver(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.receiver
    }
    pub(crate) fn receiver_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        &self.receiver_site
    }
    pub(crate) fn field_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        &self.field_site
    }
    /// Same all-writer field authority across distinct original method owners.
    pub(in crate::mir::normal_callable_semantic_package) fn same_field_source_v1(
        &self,
        other: &Self,
    ) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_declared_source_v1(
        &self, original: &Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::DeclaredFieldValueSourceV1>,
    ) -> bool {
        Rc::ptr_eq(&self.source, original)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_source(
        &self,
        other: &Self,
    ) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && self.receiver == other.receiver
            && self.receiver_site == other.receiver_site
            && self.field_site == other.field_site
            && self.name == other.name
    }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(
        &self,
        claims: &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewFieldWriteClaimsV1,
    ) -> bool {
        self.source.same_inventory(claims)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn require_finished_birth_v1(
        &self,
        source: &crate::mir::normal_callable_semantic_package::FinalizedRootSourceHandoffV1,
        module: &crate::mir::MirModule,
    ) -> Result<(), String> {
        source.require_finished_map_birth_v1(
            module,
            &self.birth_key,
            self.birth_owner,
            &self.birth_store,
        )
    }
    pub(in crate::mir::normal_callable_semantic_package) fn require_entry_source_v1(
        &self,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        if !ledger.corroborate_field_map_source_v1(self) {
            return Err("expression-core/source-identity".into());
        }
        self.root.recheck_entry_v1(ledger, self.receiver.owner())
    }
    pub(crate) fn require_receiver_v1(
        &self,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        read: &crate::mir::builder::ExactLexicalReadV1,
    ) -> Result<crate::mir::ValueId, String> {
        self.require_receiver_at_v1(context, function, read, &self.receiver_site)
    }
    /// Another original use of the SAME field retains its own affine read site.
    /// The caller must corroborate that use against its retained source frame.
    pub(in crate::mir::normal_callable_semantic_package) fn require_receiver_at_v1(
        &self,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction,
        read: &crate::mir::builder::ExactLexicalReadV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<crate::mir::ValueId, String> {
        let ledger = context.ledger();
        self.require_entry_source_v1(ledger)?;
        let (_, entry) =
            self.root
                .checked_entry_value_v1(ledger, self.receiver.owner(), function)?;
        read.loan_forwarded_copies(self.receiver, entry)?;
        read.value_for(self.receiver.owner(), site.node(), self.receiver)
            .map_err(|error| error.to_string())
    }
    fn matches_plan(
        &self,
        plan: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    ) -> bool {
        use crate::mir::resolved_semantics::{BodyExpressionShapeV1 as Shape, BodyMeReceiverV1};
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        plan.owner() == self.receiver.owner()
            && plan.nodes().iter().filter(|node| node.site() == &self.field_site
                && matches!(node.source(), Node::Shape(Shape::FieldAccess { object, field, .. })
                    if object == &self.receiver_site && field == &self.name)).count() == 1
            && plan.nodes().iter().filter(|node| node.site() == &self.receiver_site
                && matches!(node.source(), Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                    | Node::Shape(Shape::Me { receiver: BodyMeReceiverV1::Lexical(binding), .. })
                    if *binding == self.receiver)).count() == 1
    }
}
