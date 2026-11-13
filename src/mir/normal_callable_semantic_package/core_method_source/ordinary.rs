//! Original ordinary Core source issuance; no Loop or physical capability.
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ClosedOriginalCoreSourceV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
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
    pub(in crate::mir::normal_callable_semantic_package) fn is_array_dynamic_get_v1(&self) -> bool {
        self.source.target().schema()
            == crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::ArrayDynamicRead
            && self.source.target().row().row().op
                == crate::mir::core_method_op::CoreMethodOp::ArrayGet
            && self
                .operands
                .field_array_access_v1()
                .is_some_and(|access| access.is_dynamic_get())
    }
    /// A payload class only; the physical Get result remains an owned carrier.
    pub(in crate::mir::normal_callable_semantic_package) fn proves_text_result_v1(&self) -> bool {
        self.operands.result_sum_v1().text_only()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn source(
        &self,
    ) -> &VerifiedResolverOrdinaryCoreMethodSourceV1 {
        &self.source
    }
    /// Fault handling: the read's law has a Fault, so the Home owner cleans up.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_fault_read_v1(&self) -> bool {
        issues_checked_fault_read_v1(&self.operands, self.source.target())
    }
    /// Receiver reading: the receiver is a live named-local Map, borrowed by one
    /// shared path read, never read as an ordinary receiver value. An array
    /// alias receiver is also path-read (`path_read_receiver_v1`) but is not
    /// answered here: the comparison nodes keep their own array-receiver
    /// handling (`direct_array_receiver_v1`), and asking for both kinds changes
    /// the diagnostic of an alias-length condition.
    pub(in crate::mir::normal_callable_semantic_package) fn borrows_local_map_receiver_v1(
        &self,
    ) -> bool {
        self.operands.local_map().is_some()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn operands(
        &self,
    ) -> &Rc<ClosedOriginalCoreSourceV1> {
        &self.operands
    }
    /// Original source/argument correspondence only. A Dynamic result can
    /// lend an unannotated input, never a nominal class or executable carrier.
    pub(in crate::mir::normal_callable_semantic_package) fn matches_original_argument_v1(
        &self,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
        formal: &crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
    ) -> bool {
        use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
        !(method.owner() != site.owner()
            || method.site() != site.site()
            || !std::sync::Arc::ptr_eq(method, self.source().source())
            || !(self.source().target().schema()
                == crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::StringBoxText
                || self.source().target().schema()
                    == crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::ArrayDynamicRead
                    && self
                        .operands()
                        .field_array_access_v1()
                        .is_some_and(|source| source.is_dynamic_get()))
            || !match (formal, self.source().target().result()) {
                (
                    kind,
                    crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller,
                ) => kind.is_ordinary_borrowed_handle(),
                (
                    kind,
                    crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller,
                ) => {
                    matches!(kind, CallableParameterContractKindV1::OpaqueHandle)
                        || matches!(kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64())
                }
                (
                    CallableParameterContractKindV1::OpaqueHandle,
                    crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::DynamicToCaller,
                ) => true,
                _ => false,
            })
    }
}

/// The one place that decides "this Core call is a Map get": receiver evidence
/// plus the row's law, a live named-local Map read by the checked `get` whose
/// result is an i64 or a Fault (114 absent, 104 non-i64). Users ask the two
/// properties (Fault handling, receiver reading), never this name.
fn issues_checked_fault_read_v1(
    operands: &ClosedOriginalCoreSourceV1,
    target: &crate::mir::resolved_semantics::VerifiedCoreMethodInstanceTargetV1,
) -> bool {
    operands.local_map().is_some() && target.checked_map_get_law_v1()
}

/// The value class a checked Fault read's law returns on its Normal path.
fn checked_read_result_class_v1(
    target: &crate::mir::resolved_semantics::VerifiedCoreMethodInstanceTargetV1,
) -> Option<crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1> {
    use crate::mir::core_method_result_kind::CoreMethodSemanticLawV2 as Law;
    match target.row().semantic_law_for_arity()? {
        Law::MapTextGetPresentOrFault => {
            Some(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64)
        }
        _ => None,
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
    // Each Map row takes exactly the receiver evidence its operation reads:
    // a field Map for `has`/`keys`, a named-local Map for the checked `get`.
    let evidence_fits = match op {
        Op::MapHas | Op::MapKeys => operands.field_map().is_some(),
        Op::MapGet => operands.local_map().is_some() && operands.field_map().is_none(),
        _ => operands.field_map().is_none() && operands.local_map().is_none(),
    };
    if !evidence_fits {
        return Err(reject());
    }
    let access = operands.field_array_access_v1();
    let receiver_valid = match op {
        Op::ArrayLen => operands.receiver_core().is_some() ^ access.is_some(),
        Op::ArrayGet => {
            operands.receiver_core().is_none()
                && access.is_some_and(|source| source.is_dynamic_get())
        }
        _ => operands.receiver_core().is_none() && access.is_none(),
    };
    if !receiver_valid
        || operands.receiver_core().is_some_and(|inner| {
            inner.manifest().row().op != Op::MapKeys
                || inner.field_map().is_none()
                || inner.method().owner() != method.owner()
                || inner.method().result_site() != method.receiver_site()
                || !input
                    .function()
                    .method_call_source_v1(inner.method().site())
                    .is_some_and(|original| std::sync::Arc::ptr_eq(original, inner.method()))
        })
    {
        return Err(reject());
    }
    if let Some(access) = access {
        if !(op == Op::ArrayLen && access.is_count()
            || op == Op::ArrayGet && access.is_dynamic_get())
            || !access.matches_original_call_v1(input)
            || access.call().owner() != method.owner()
            || access.call().site() != method.site()
            || access.receiver().site() != method.receiver_site()
            || access.read_manifest_v1().is_none_or(|original| {
                !std::ptr::eq(original.row(), operands.manifest().row())
                    || original.arity() != operands.manifest().arity()
            })
        {
            return Err(reject());
        }
        access.recheck_census(input.owner(), &std::collections::BTreeMap::new())?;
    }
    let mut issuer = match op {
        Op::MapHas => {
            CoreMethodInstanceTargetIssuerV1::map_text_presence(operands.manifest().brand())
        }
        Op::MapKeys => CoreMethodInstanceTargetIssuerV1::map_text_keys(operands.manifest().brand()),
        Op::MapGet => CoreMethodInstanceTargetIssuerV1::map_text_get(operands.manifest().brand()),
        Op::MapSet => CoreMethodInstanceTargetIssuerV1::map_text_set(operands.manifest().brand()),
        Op::MapDelete => {
            CoreMethodInstanceTargetIssuerV1::map_text_delete(operands.manifest().brand())
        }
        Op::MapLen => {
            CoreMethodInstanceTargetIssuerV1::map_entry_count(operands.manifest().brand())
        }
        Op::ArrayLen => CoreMethodInstanceTargetIssuerV1::array_count(operands.manifest().brand()),
        Op::ArrayGet => {
            CoreMethodInstanceTargetIssuerV1::array_dynamic_read(operands.manifest().brand())
        }
        _ => CoreMethodInstanceTargetIssuerV1::string_box_text(operands.manifest().brand()),
    }
    .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    let target = issuer
        .issue(operands.manifest())
        .map_err(|error| format!("[freeze:contract][ordinary-core/target] {error:?}"))?;
    if target.checked_code_point_law_v1().is_none()
        && !target.checked_map_presence_law_v1()
        && !(op == Op::MapGet && issues_checked_fault_read_v1(operands, &target))
        && !target.checked_text_keys_law_v1()
        && !target.checked_array_count_law_v1()
        && !(op == Op::ArrayGet
            && access.is_some_and(|source| source.is_dynamic_get())
            && target.schema()
                == crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::ArrayDynamicRead)
    {
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
    pub(in crate::mir::normal_callable_semantic_package) fn borrows_original_v1(
        &self,
        original: &Rc<OrdinaryCoreSourceCallV1>,
    ) -> bool {
        Rc::ptr_eq(&self.original, original)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_receiver_result_v1(
        &self,
        inner: &Self,
    ) -> bool {
        Rc::ptr_eq(&self.plan, &inner.plan)
            && self.source().source().receiver_site() == inner.source().source().result_site()
            && self
                .original
                .operands
                .receiver_core()
                .is_some_and(|original| Rc::ptr_eq(original, inner.original.operands()))
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
                || self.checked_fault_read_v1()
                || self.field_map().is_some_and(|field| {
                    (self.source().target().checked_map_presence_law_v1()
                        || self.source().target().checked_text_keys_law_v1())
                        && field.matches_plan(plan)
                })
                || ((self.source().target().checked_array_count_law_v1()
                    || self.source().target().schema()
                        == crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::ArrayDynamicRead)
                    && self
                        .field_array_access_v1()
                        .is_some_and(|access| access.matches_expression_receiver_v1(plan)))
                || (self.source().target().checked_array_count_law_v1()
                    && self.original.operands.receiver_core().is_some_and(|inner| {
                        inner.method().result_site() == self.source().source().receiver_site()
                            && plan
                                .nodes()
                                .iter()
                                .filter(|node| {
                                    node.site() == inner.method().site()
                                        && matches!(node.source(), Node::MethodCall)
                                })
                                .count()
                                == 1
                            && inner
                                .field_map()
                                .is_some_and(|field| field.matches_plan(plan))
                    })))
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
        if self.is_array_dynamic_get_v1() && self.proves_text_result_v1() {
            return Some(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String);
        }
        // The result class of a checked read comes from its law, not from the
        // read having a Fault: `CheckedGetI64` (law `MapTextGetPresentOrFault`)
        // is an i64 or Fault 114 (absent key) / 104 (present non-i64 value).
        // It does not come from which values the Map was written with. A Fault
        // read of another law answers for itself.
        if self.checked_fault_read_v1() {
            if let Some(class) = checked_read_result_class_v1(self.source().target()) {
                return Some(class);
            }
        }
        self.source().target().expression_result_class_v1()
    }
    /// Admission and Normal result of the shared scalar read evaluation.
    /// This does not decide whether evaluation or cleanup needs a Fault frame.
    pub(crate) fn scalar_read_result_class_v1(
        &self,
    ) -> Option<crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1> {
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as Class;
        if self.owns_result()
            || !(self.source().target().frame_free_read_law_v1().is_some()
                || self.checked_fault_read_v1())
        {
            return None;
        }
        match self.result_class()? {
            class @ (Class::I64 | Class::Bool) => Some(class),
            _ => None,
        }
    }
    /// Fault handling: a checked read whose law has a Fault (the Home owner
    /// cleans up), as opposed to a frame-free read (`frame_free_read_law_v1`).
    pub(crate) fn checked_fault_read_v1(&self) -> bool {
        self.original.checked_fault_read_v1()
    }
    /// Receiver reading: a live named-local Map receiver, borrowed by the shared
    /// path read rather than read as an ordinary receiver value.
    pub(crate) fn borrows_local_map_receiver_v1(&self) -> bool {
        self.original.borrows_local_map_receiver_v1()
    }
    /// The receiver this Core call reads once through a shared path read: an
    /// array alias or a live named-local Map. Users prepare that one read
    /// without choosing between the two kinds.
    pub(crate) fn path_read_receiver_v1(&self) -> Option<(OwnedExprSiteV1, BindingRefV1)> {
        if let Some(access) = self.field_array_access_v1() {
            return Some((
                access.receiver().clone(),
                access.receiver_alias_v1()?.binding(),
            ));
        }
        let source = self.source().source();
        let site = OwnedExprSiteV1::new(source.owner(), source.receiver_site().clone());
        Some((site, self.local_map()?))
    }
    /// Receiver evidence only: the live named-local Map this checked `get`
    /// borrows. It says nothing about the type of the stored values.
    pub(crate) fn local_map(&self) -> Option<crate::mir::resolved_semantics::BindingRefV1> {
        self.original.operands.local_map()
    }
    /// A payload class only; the physical Get result remains an owned carrier.
    pub(crate) fn proves_text_result_v1(&self) -> bool {
        self.original.proves_text_result_v1()
    }
    pub(crate) fn field_array_access_v1(
        &self,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>
    {
        self.original.operands.field_array_access_v1()
    }
    pub(crate) fn field_map(&self) -> Option<&FieldMapReceiverSourceV1> {
        self.original.operands.field_map().map(Rc::as_ref)
    }
    pub(crate) fn owns_result(&self) -> bool {
        use crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1 as Result;
        self.source().target().result() == Result::TextToCaller
            || self.source().target().result() == Result::DynamicToCaller
                && self.is_array_dynamic_get_v1()
    }
    pub(crate) fn is_array_dynamic_get_v1(&self) -> bool {
        self.original.is_array_dynamic_get_v1()
    }
}

/// The one receiver evidence a Map Core call carries. It says which Map the
/// read borrows, never what the Map holds: the result class comes from the
/// operation's own checked contract.
#[derive(Debug, Clone)]
pub(crate) enum MapReceiverEvidenceV1 {
    /// A Map field of `me`: the receiver read travels with the scalar invocation.
    Field(Rc<FieldMapReceiverSourceV1>),
    /// A named local whose sole initializer is a sealed Map construction. The
    /// live Home is checked later; the read is one shared path read.
    Local(crate::mir::resolved_semantics::BindingRefV1),
}
impl MapReceiverEvidenceV1 {
    pub(crate) fn field(&self) -> Option<&Rc<FieldMapReceiverSourceV1>> {
        match self {
            Self::Field(field) => Some(field),
            Self::Local(_) => None,
        }
    }
    pub(crate) fn local(&self) -> Option<crate::mir::resolved_semantics::BindingRefV1> {
        match self {
            Self::Local(binding) => Some(*binding),
            Self::Field(_) => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Field(left), Self::Field(right)) => left.matches_source(right),
            (Self::Local(left), Self::Local(right)) => left == right,
            _ => false,
        }
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
        let (writer, store) = source
            .map_birth_provider_v1(constructors)
            .ok_or_else(reject)?;
        let root = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ExpressionFieldRootSourceV1::issue_entry_v1(
            constructors, receiver_proof, entry, input.owner(), *receiver, source.field())?;
        let birth_key = constructors
            .published_birth_key(writer.birth_source().ok_or_else(reject)?)
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
            birth_owner: writer.owner(),
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
        &self,
        original: &Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::DeclaredFieldValueSourceV1>,
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
