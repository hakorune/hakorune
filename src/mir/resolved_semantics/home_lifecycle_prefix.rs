//! Mandatory lifecycle source membership from the canonical Home flow.
use super::*;

/// Original source membership of one lifecycle prefix, before any physical
/// observations exist. A Print remains mandatory even with no calls or owners.
#[derive(Debug, Clone)]
pub(crate) enum LifecyclePrefixSourceV1 {
    Call { site: OwnedExprSiteV1, statement: SourceStmtSiteV1 },
    FieldArrayAppend {
        site: OwnedExprSiteV1,
        proof: std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>,
        index: usize,
    },
    OwnedArrayAppend {
        site: OwnedExprSiteV1,
        statement: SourceStmtSiteV1,
        holder: crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
        candidate: OwnedExprSiteV1,
    },
    BoolCondition {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<CheckedBoolConditionFrameV1>,
    },
    InitializerExpression { site: OwnedExprSiteV1, frame: std::rc::Rc<CheckedInitializerExpressionFrameV1> },
    PrintExpression {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<CheckedPrintExpressionFrameV1>,
    },
    BorrowedFieldWrite {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<CheckedFieldWriteFrameV1>,
    },
}
impl LifecyclePrefixSourceV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match self { Self::InitializerExpression { site, .. } | Self::BorrowedFieldWrite { site, .. } | Self::Call { site, .. } | Self::FieldArrayAppend { site, .. } | Self::BoolCondition { site, .. } | Self::PrintExpression { site, .. } | Self::OwnedArrayAppend { site, .. } => site }
    }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        match self {
            Self::Call { statement, .. } | Self::OwnedArrayAppend { statement, .. } => statement,
            Self::FieldArrayAppend { proof, .. } => proof.site(),
            Self::InitializerExpression { frame, .. } => frame.source().root().statement(),
            Self::PrintExpression { frame, .. } => frame.source().root().statement(),
            Self::BoolCondition { frame, .. } => frame.source().root().statement(),
            Self::BorrowedFieldWrite { frame, .. } => frame.source().root().statement(),
        }
    }
    pub(crate) fn covered_by(&self, exit: &RootHomeExitV1) -> bool {
        match self {
            Self::Call { site, .. } => exit.covered_calls().contains(site),
            Self::InitializerExpression { .. } | Self::BorrowedFieldWrite { .. } | Self::FieldArrayAppend { .. } | Self::BoolCondition { .. } | Self::PrintExpression { .. } | Self::OwnedArrayAppend { .. } => exit.covers_statement(self.statement()),
        }
    }
}

impl RootHomeFlow {
    /// Preserve the existing call postorder within a statement. Only enclosing
    /// statement paths order separate prefixes; expression-path ordering would
    /// incorrectly put an outer call before its evaluated arguments.
    pub(crate) fn initializer_scalar_call_dependency_v1(&self, site: &OwnedExprSiteV1) -> bool {
        self.expression_plans.iter().filter_map(HomeExpressionPlanV1::initializer_frame).any(|frame|
            frame.evaluation().references().values().any(|reference| match reference {
                super::CheckedExpressionReferenceV1::Scalar(read) => read.original_read().is_some_and(|read|
                    matches!(read.producer(), super::CurrentScalarProducerV1::Call(call) if call.site() == site)),
                _ => false,
            }))
    }
    pub(crate) fn lifecycle_prefixes(&self, routed: &[OwnedExprSiteV1]) -> Result<Vec<LifecyclePrefixSourceV1>, &'static str> {
        let frames: Vec<_> = self.expression_plans.iter().filter_map(HomeExpressionPlanV1::print_frame).collect();
        let initializers: Vec<_> = self.expression_plans.iter().filter_map(HomeExpressionPlanV1::initializer_frame).collect();
        let conditions: Vec<_> = self.expression_plans.iter().filter_map(HomeExpressionPlanV1::condition_frame).collect();
        let mut prefixes: Vec<_> = self.local_calls.iter().filter(|call| {
            matches!(call.result(), super::LocalCallResultClassV1::Unit | super::LocalCallResultClassV1::I64
                | super::LocalCallResultClassV1::Bool
                | super::LocalCallResultClassV1::Handle | super::LocalCallResultClassV1::Nullable
                | super::LocalCallResultClassV1::BorrowedNullable | super::LocalCallResultClassV1::MixedObject)
                && (routed.contains(call.site()) || self.initializer_scalar_call_dependency_v1(call.site()))
                && !frames.iter().any(|frame| frame.calls().iter().any(|nested| nested.site() == call.site()))
                && !initializers.iter().any(|frame| frame.evaluation().calls().iter().any(|nested| nested.site() == call.site()))
                && !conditions.iter().any(|frame| frame.calls().iter().any(|nested| nested.site() == call.site()))
        }).map(|call| LifecyclePrefixSourceV1::Call {
            site: call.site().clone(), statement: call.statement().clone(),
        }).collect();
        if prefixes.windows(2).any(|pair| pair[0].statement() > pair[1].statement())
            || frames.windows(2).any(|pair| pair[0].source().root().statement() > pair[1].source().root().statement())
            || conditions.windows(2).any(|pair| pair[0].source().root().statement() > pair[1].source().root().statement()) {
            return Err("lifecycle-prefix/source-order");
        }
        prefixes.extend(frames.into_iter().map(|frame| LifecyclePrefixSourceV1::PrintExpression {
            site: OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone()),
            frame: std::rc::Rc::clone(frame),
        }));
        prefixes.extend(initializers.into_iter().map(|frame| LifecyclePrefixSourceV1::InitializerExpression {
            site: OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone()), frame: std::rc::Rc::clone(frame),
        }));
        prefixes.extend(conditions.into_iter().map(|frame| LifecyclePrefixSourceV1::BoolCondition {
            site: OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone()),
            frame: std::rc::Rc::clone(frame),
        }));
        let writes: Vec<_> = self.expression_plans.iter()
            .filter_map(HomeExpressionPlanV1::field_write_frame).collect();
        if writes.windows(2).any(|pair|
            pair[0].source().root().statement() > pair[1].source().root().statement()) {
            return Err("lifecycle-prefix/source-order");
        }
        prefixes.extend(writes.into_iter().map(|frame| LifecyclePrefixSourceV1::BorrowedFieldWrite {
            site: OwnedExprSiteV1::new(frame.source().owner(), frame.source().nodes()[0].site().clone()),
            frame: std::rc::Rc::clone(frame),
        }));
        for observation in &self.named_arrays {
            if let super::NamedArrayHomeObservationV1::Append { source, call, candidate, availability, .. } = observation {
                availability.as_ref().map_err(|_| "lifecycle-prefix/array-append-unavailable")?;
                prefixes.push(LifecyclePrefixSourceV1::OwnedArrayAppend {
                    site: call.clone(), statement: SourceStmtSiteV1::from_node(call.site().node().clone()),
                    holder: source.clone(), candidate: candidate.clone(),
                });
            }
        }
        for proof in self.field_array_loop_sources() {
            for (index, append) in proof.appends().iter().enumerate() {
                prefixes.push(LifecyclePrefixSourceV1::FieldArrayAppend {
                    site: append.fault().call().clone(), proof: std::rc::Rc::clone(proof), index,
                });
            }
        }
        prefixes.sort_by(|a, b| a.statement().cmp(b.statement()));
        if prefixes.windows(2).any(|pair| pair[0].site() == pair[1].site()) {
            return Err("lifecycle-prefix/source-duplicate");
        }
        Ok(prefixes)
    }
}
