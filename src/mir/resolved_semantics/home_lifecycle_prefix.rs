//! Mandatory lifecycle source membership from the canonical Home flow.
use super::*;

/// Original source membership of one lifecycle prefix, before any physical
/// observations exist. A Print remains mandatory even with no calls or owners.
#[derive(Debug, Clone)]
pub(crate) enum LifecyclePrefixSourceV1 {
    Loop {
        site: OwnedExprSiteV1,
        product: std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    },
    FieldMapInstall { site: OwnedExprSiteV1, frame: std::rc::Rc<MapFieldInstallV1> },
    StoredMapGet {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<MapFieldGetLocalV1>,
    },
    Call {
        site: OwnedExprSiteV1,
        statement: SourceStmtSiteV1,
    },
    FieldTextAppend {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<super::CheckedFieldArrayTextAppendV1>,
    },
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
    BindingRebind {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<CheckedBindingRebindFrameV1>,
    },
    InitializerExpression {
        site: OwnedExprSiteV1,
        frame: std::rc::Rc<CheckedInitializerExpressionFrameV1>,
    },
    SharedInitializer {
        site: OwnedExprSiteV1,
        source: std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    },
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
        match self {
            Self::FieldMapInstall { site, .. }
            | Self::StoredMapGet { site, .. }
            | Self::Loop { site, .. }
            | Self::BindingRebind { site, .. }
            | Self::InitializerExpression { site, .. }
            | Self::SharedInitializer { site, .. }
            | Self::BorrowedFieldWrite { site, .. }
            | Self::Call { site, .. }
            | Self::FieldArrayAppend { site, .. }
            | Self::FieldTextAppend { site, .. }
            | Self::BoolCondition { site, .. }
            | Self::PrintExpression { site, .. }
            | Self::OwnedArrayAppend { site, .. } => site,
        }
    }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        match self {
            Self::FieldMapInstall { frame, .. } => frame.statement(),
            Self::StoredMapGet { frame, .. } => frame.statement(),
            Self::Loop { product, .. } => product.source.source.site(),
            Self::Call { statement, .. } | Self::OwnedArrayAppend { statement, .. } => statement,
            Self::FieldArrayAppend { proof, .. } => proof.site(),
            Self::FieldTextAppend { frame, .. } => frame.statement(),
            Self::BindingRebind { frame, .. } => frame.source().root().statement(),
            Self::InitializerExpression { frame, .. } => frame.source().root().statement(),
            Self::SharedInitializer { source, .. } => source.root().statement(),
            Self::PrintExpression { frame, .. } => frame.source().root().statement(),
            Self::BoolCondition { frame, .. } => frame.source().root().statement(),
            Self::BorrowedFieldWrite { frame, .. } => frame.source().root().statement(),
        }
    }
    pub(crate) fn covered_by(&self, exit: &RootHomeExitV1) -> bool {
        match self {
            Self::Call { site, .. } => exit.covered_calls().contains(site),
            Self::FieldMapInstall { .. }
            | Self::StoredMapGet { .. }
            | Self::Loop { .. }
            | Self::BindingRebind { .. }
            | Self::InitializerExpression { .. }
            | Self::SharedInitializer { .. }
            | Self::BorrowedFieldWrite { .. }
            | Self::FieldArrayAppend { .. }
            | Self::FieldTextAppend { .. }
            | Self::BoolCondition { .. }
            | Self::PrintExpression { .. }
            | Self::OwnedArrayAppend { .. } => exit.covers_statement(self.statement()),
        }
    }
}

impl RootHomeFlow {
    /// Preserve the existing call postorder within a statement. Only enclosing
    /// statement paths order separate prefixes; expression-path ordering would
    /// incorrectly put an outer call before its evaluated arguments.
    pub(crate) fn initializer_scalar_call_dependency_v1(&self, site: &OwnedExprSiteV1) -> bool {
        self.expression_plans.iter().filter_map(HomeExpressionPlanV1::initializer_frame).map(|frame| frame.evaluation())
            .chain(self.expression_plans.iter().filter_map(HomeExpressionPlanV1::rebind_frame).map(|frame| frame.evaluation())).any(|evaluation|
            evaluation.references().values().any(|reference| match reference {
                super::CheckedExpressionReferenceV1::Scalar(read) => read.original_read().is_some_and(|read|
                    matches!(read.producer(), super::CurrentScalarProducerV1::Call(call) if call.site() == site)),
                _ => false,
            }))
    }
    pub(crate) fn lifecycle_prefixes(
        &self,
        routed: &[OwnedExprSiteV1],
    ) -> Result<Vec<LifecyclePrefixSourceV1>, &'static str> {
        // The Loop owns its conditional body, not a sequence of always-run
        // root prefixes. Membership is the checked tree's SAME source Rc.
        let all_loops: Vec<_> = self
            .expression_plans
            .iter()
            .filter_map(HomeExpressionPlanV1::loop_recipe_v2)
            .collect();
        // A child Loop is a lifecycle group of its parent Loop, not of the root.
        let loops: Vec<_> = all_loops.iter().copied()
            .filter(|product| !all_loops.iter().any(|parent| parent.owns_child_loop_v2(product)))
            .collect();
        let standalone = |plan: &&HomeExpressionPlanV1| {
            !loops
                .iter()
                .any(|product| product.owns_expression_v2(plan.source()))
        };
        let frames: Vec<_> = self
            .expression_plans
            .iter()
            .filter(standalone)
            .filter_map(HomeExpressionPlanV1::print_frame)
            .collect();
        let initializers: Vec<_> = self
            .expression_plans
            .iter()
            .filter(standalone)
            .filter_map(HomeExpressionPlanV1::initializer_frame)
            .collect();
        let rebinds: Vec<_> = self
            .expression_plans
            .iter()
            .filter(standalone)
            .filter_map(HomeExpressionPlanV1::rebind_frame)
            .collect();
        let conditions: Vec<_> = self
            .expression_plans
            .iter()
            .filter(standalone)
            .filter_map(HomeExpressionPlanV1::condition_frame)
            .collect();
        let mut prefixes: Vec<_> = self
            .local_calls
            .iter()
            .filter(|call| {
                matches!(
                    call.result(),
                    super::LocalCallResultClassV1::Unit
                        | super::LocalCallResultClassV1::I64
                        | super::LocalCallResultClassV1::Bool
                        | super::LocalCallResultClassV1::Handle
                        | super::LocalCallResultClassV1::Nullable
                        | super::LocalCallResultClassV1::BorrowedNullable
                        | super::LocalCallResultClassV1::MixedObject
                ) && !loops
                    .iter()
                    .any(|product| product.owns_call_v2(call.site()))
                    && (routed.contains(call.site())
                        || self.initializer_scalar_call_dependency_v1(call.site()))
                    && !frames.iter().any(|frame| {
                        frame
                            .calls()
                            .iter()
                            .any(|nested| nested.site() == call.site())
                    })
                    && !rebinds.iter().any(|frame| {
                        frame
                            .evaluation()
                            .calls()
                            .iter()
                            .any(|nested| nested.site() == call.site())
                    })
                    && !initializers.iter().any(|frame| {
                        frame
                            .evaluation()
                            .calls()
                            .iter()
                            .any(|nested| nested.site() == call.site())
                    })
                    && !conditions.iter().any(|frame| {
                        frame
                            .calls()
                            .iter()
                            .any(|nested| nested.site() == call.site())
                    })
            })
            .map(|call| LifecyclePrefixSourceV1::Call {
                site: call.site().clone(),
                statement: call.statement().clone(),
            })
            .collect();
        if prefixes
            .windows(2)
            .any(|pair| pair[0].statement() > pair[1].statement())
            || frames.windows(2).any(|pair| {
                pair[0].source().root().statement() > pair[1].source().root().statement()
            })
            || conditions.windows(2).any(|pair| {
                pair[0].source().root().statement() > pair[1].source().root().statement()
            })
        {
            return Err("lifecycle-prefix/source-order");
        }
        prefixes.extend(
            frames
                .into_iter()
                .map(|frame| LifecyclePrefixSourceV1::PrintExpression {
                    site: OwnedExprSiteV1::new(
                        frame.source().owner(),
                        frame.source().nodes()[0].site().clone(),
                    ),
                    frame: std::rc::Rc::clone(frame),
                }),
        );
        prefixes.extend(initializers.into_iter().map(|frame| {
            LifecyclePrefixSourceV1::InitializerExpression {
                site: OwnedExprSiteV1::new(
                    frame.source().owner(),
                    frame.source().nodes()[0].site().clone(),
                ),
                frame: std::rc::Rc::clone(frame),
            }
        }));
        prefixes.extend(self.expression_plans.iter().filter(standalone)
            .filter_map(HomeExpressionPlanV1::shared_initializer_source_v1)
            .map(|source| LifecyclePrefixSourceV1::SharedInitializer {
                site: OwnedExprSiteV1::new(source.owner(), source.nodes()[0].site().clone()),
                source: std::rc::Rc::clone(source),
            }));
        prefixes.extend(
            rebinds
                .into_iter()
                .map(|frame| LifecyclePrefixSourceV1::BindingRebind {
                    site: OwnedExprSiteV1::new(
                        frame.source().owner(),
                        frame.source().nodes()[0].site().clone(),
                    ),
                    frame: std::rc::Rc::clone(frame),
                }),
        );
        prefixes.extend(conditions.into_iter().map(|frame| {
            LifecyclePrefixSourceV1::BoolCondition {
                site: OwnedExprSiteV1::new(
                    frame.source().owner(),
                    frame.source().nodes()[0].site().clone(),
                ),
                frame: std::rc::Rc::clone(frame),
            }
        }));
        let writes: Vec<_> = self
            .expression_plans
            .iter()
            .filter(standalone)
            .filter_map(HomeExpressionPlanV1::field_write_frame)
            .collect();
        if writes
            .windows(2)
            .any(|pair| pair[0].source().root().statement() > pair[1].source().root().statement())
        {
            return Err("lifecycle-prefix/source-order");
        }
        prefixes.extend(writes.into_iter().map(|frame| {
            LifecyclePrefixSourceV1::BorrowedFieldWrite {
                site: OwnedExprSiteV1::new(
                    frame.source().owner(),
                    frame.source().nodes()[0].site().clone(),
                ),
                frame: std::rc::Rc::clone(frame),
            }
        }));
        for observation in &self.named_arrays {
            if let super::NamedArrayHomeObservationV1::Append {
                source,
                call,
                candidate,
                availability,
                ..
            } = observation
            {
                availability
                    .as_ref()
                    .map_err(|_| "lifecycle-prefix/array-append-unavailable")?;
                if loops
                    .iter()
                    .any(|product| product.owned_array_append_v2(call).is_some())
                {
                    continue;
                }
                prefixes.push(LifecyclePrefixSourceV1::OwnedArrayAppend {
                    site: call.clone(),
                    statement: SourceStmtSiteV1::from_node(call.site().node().clone()),
                    holder: source.clone(),
                    candidate: candidate.clone(),
                });
            }
        }
        for observation in self.named_arrays() {
            if let super::NamedArrayHomeObservationV1::FieldTextAppend(frame) = observation {
                prefixes.push(LifecyclePrefixSourceV1::FieldTextAppend {
                    site: frame.source().call().clone(),
                    frame: std::rc::Rc::clone(frame),
                });
            }
        }
        for proof in self.field_array_loop_sources() {
            for (index, append) in proof.appends().iter().enumerate() {
                prefixes.push(LifecyclePrefixSourceV1::FieldArrayAppend {
                    site: append.fault().call().clone(),
                    proof: std::rc::Rc::clone(proof),
                    index,
                });
            }
        }
        prefixes.extend(
            loops
                .into_iter()
                .map(|product| LifecyclePrefixSourceV1::Loop {
                    site: OwnedExprSiteV1::new(
                        product.source.source.owner(),
                        product.source.condition.source().nodes()[0].site().clone(),
                    ),
                    product: std::rc::Rc::clone(product),
                }),
        );
        for observation in &self.maps {
            if let MapHomeObservation::FieldInstall(frame) = observation {
                prefixes.push(LifecyclePrefixSourceV1::FieldMapInstall {
                    site: frame.site().clone(), frame: std::rc::Rc::clone(frame),
                });
            }
            if let MapHomeObservation::FieldGet(frame) = observation {
                prefixes.push(LifecyclePrefixSourceV1::StoredMapGet {
                    site: frame.site().clone(),
                    frame: std::rc::Rc::clone(frame),
                });
            }
        }
        prefixes.sort_by(|a, b| a.statement().cmp(b.statement()));
        if prefixes
            .windows(2)
            .any(|pair| pair[0].site() == pair[1].site())
        {
            return Err("lifecycle-prefix/source-duplicate");
        }
        Ok(prefixes)
    }
}
