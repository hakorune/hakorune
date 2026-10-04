//! Existing final-view program selection and projection; no source reclassification.
use super::*;

impl<'module> PublishedMirBackendView<'module> {
    /// Issues the complete physical image selected by final lifecycle admission.
    ///
    /// The retained source handoff selects functions and ABI.  Final MIR supplies
    /// only their already-published physical bodies; names and JSON are never
    /// consulted to select, repair, or classify the program.
    pub(crate) fn issue_lifecycle_physical_program(
        &self,
    ) -> Result<PublishedLifecyclePhysicalProgramV1<'module>, String> {
        let handoff = self
            .retained_handoff
            .ok_or_else(|| fault("root-handoff-missing"))?;
        let root = self.retained_root().ok_or_else(|| fault("root-missing"))?;
        // Ordinary membership is transitive: every emitted function carries
        // its own sealed call rows, and callees discovered mid-walk join the
        // walk. The retained root selects the entry, not the edge set.
        // Birth units seed the walk too: a qualified-static provider
        // argument is a sealed `ExactI64` call row emitted inside the birth
        // caller, so its callee joins `ordinary_sites` through the same
        // membership proof and the birth carries its own call set.
        let births: &[crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1] =
            if handoff.script_array().is_none() {
                if self.route() != PublishedStaticMethodRouteV1::CanonicalTyped {
                    return Err(fault("not-final-lifecycle-view"));
                }
                handoff
                    .births()
                    .ok_or_else(|| fault("birth-handoff-missing"))?
            } else {
                &[][..]
            };
        let mut call_sets: std::collections::BTreeMap<&str, Vec<OrdinaryCallSite>> =
            std::collections::BTreeMap::new();
        let mut ordinary_sites: std::collections::BTreeMap<
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
            OrdinaryCallSite,
        > = std::collections::BTreeMap::new();
        if handoff.script_array().is_none() {
            let mut visited = BTreeSet::from([root.signature.name.as_str()]);
            let mut pending = std::collections::VecDeque::from([root]);
            for birth in births {
                let symbol = self
                    .module()
                    .canonical_callable_definition_symbol(birth.target())
                    .ok_or_else(|| fault("birth-definition-missing"))?;
                if visited.insert(symbol) {
                    pending.push_back(
                        self.module()
                            .functions
                            .get(symbol)
                            .ok_or_else(|| fault("birth-function-missing"))?,
                    );
                }
            }
            while let Some(function) = pending.pop_front() {
                let sites = collect_ordinary_calls(function)?;
                for site in &sites {
                    let key = ordinary_callable_key(&site.call.callee)?;
                    match ordinary_sites.entry(key.clone()) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(site.clone());
                        }
                        // One physical result contract per callee key; a key
                        // cannot be both an i64 callee and a Map callee.
                        std::collections::btree_map::Entry::Occupied(entry)
                            if entry.get().result == site.result => {}
                        _ => return Err(fault("ordinary-result-contract-drift")),
                    }
                    let symbol = self
                        .module()
                        .canonical_callable_definition_symbol(&key)
                        .ok_or_else(|| fault("ordinary-definition-missing"))?;
                    if visited.insert(symbol) {
                        pending.push_back(
                            self.module()
                                .functions
                                .get(symbol)
                                .ok_or_else(|| fault("ordinary-function-missing"))?,
                        );
                    }
                }
                call_sets.insert(function.signature.name.as_str(), sites);
            }
        }
        let root_result = if let Some(script) = handoff.script_array() {
            script.validate_root_binding(root)?;
            match script.root_result()? {
                crate::mir::builder::ScriptArrayRootResultV1::Integer { .. } => {
                    CompiledEntryRootResultV1::I64
                }
                crate::mir::builder::ScriptArrayRootResultV1::Unit => {
                    CompiledEntryRootResultV1::Unit
                }
            }
        } else {
            match handoff.root_result() {
                Some(result) => super::super::compiled_entry_contract::root_result_category(result),
                None => return Err(fault("root-result-missing")),
            }
        };
        let mut names = BTreeSet::new();
        let mut functions = Vec::with_capacity(births.len() + ordinary_sites.len() + 1);
        names.insert(root.signature.name.as_str());
        functions.push(issue_function_with_module(
            Some(self.module()),
            root,
            PublishedLifecyclePhysicalFunctionRoleV1::Root {
                result: root_result,
            },
            handoff.script_array().is_some(),
            call_sets
                .get(root.signature.name.as_str())
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        )?);
        for (key, site) in &ordinary_sites {
            let symbol = self
                .module()
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| fault("ordinary-definition-missing"))?;
            let function = self
                .module()
                .functions
                .get(symbol)
                .ok_or_else(|| fault("ordinary-function-missing"))?;
            let receiver = ordinary_call_receiver(&site.call.callee)?;
            let expected_arity = site.call.args.len() + usize::from(receiver.is_some());
            if function.signature.name != key.mir_symbol_projection()
                || function.signature.params.len() != expected_arity
                || !matches!(
                    (site.result, &function.signature.return_type),
                    (InvokeCallResultKind::I64, crate::mir::MirType::Integer)
                        | (
                            InvokeCallResultKind::Map
                                | InvokeCallResultKind::Handle
                                | InvokeCallResultKind::NullableHandle,
                            crate::mir::MirType::Box(_) | crate::mir::MirType::Unknown,
                        )
                )
                || !names.insert(symbol)
            {
                return Err(fault("ordinary-membership-drift"));
            }
            let receiver_object = object_identity::ordinary_receiver_object(self.module(), key)?;
            let role = match site.result {
                InvokeCallResultKind::I64 => {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 {
                        key: key.clone(),
                        receiver_object,
                    }
                }
                InvokeCallResultKind::Map => {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryMap {
                        key: key.clone(),
                        receiver_object,
                    }
                }
                InvokeCallResultKind::Handle => {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle {
                        key: key.clone(),
                        receiver_object,
                    }
                }
                InvokeCallResultKind::NullableHandle => {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryNullableHandle {
                        key: key.clone(),
                        receiver_object,
                    }
                }
                InvokeCallResultKind::Unit => {
                    return Err(fault("ordinary-result-contract-drift"));
                }
            };
            functions.push(issue_function_with_module(
                Some(self.module()),
                function,
                role,
                false,
                call_sets.get(symbol).map(Vec::as_slice).unwrap_or(&[]),
            )?);
        }
        for birth in births {
            let key = birth.target();
            if key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor {
                return Err(fault("birth-namespace"));
            }
            let symbol = self
                .module()
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| fault("birth-definition-missing"))?;
            let function = self
                .module()
                .functions
                .get(symbol)
                .ok_or_else(|| fault("birth-function-missing"))?;
            if function.signature.name != key.mir_symbol_projection()
                || function.params.len() != birth.abi().physical_arity()
                || !names.insert(symbol)
            {
                return Err(fault("birth-membership-drift"));
            }
            functions.push(issue_function_with_module(
                Some(self.module()),
                function,
                PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi: birth.clone() },
                false,
                call_sets.get(symbol).map(Vec::as_slice).unwrap_or(&[]),
            )?);
        }
        Ok(PublishedLifecyclePhysicalProgramV1 {
            functions: functions.into_boxed_slice(),
            handoff,
        })
    }
}
