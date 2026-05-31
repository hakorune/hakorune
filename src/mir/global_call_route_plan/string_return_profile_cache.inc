impl GenericStringReturnProfileCache {
    pub(super) fn profile_for<'a>(
        &'a mut self,
        function: &MirFunction,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
    ) -> &'a GenericStringReturnProfile {
        let key = generic_string_return_profile_cache_key(function, targets);
        self.profiles
            .entry(key)
            .or_insert_with(|| build_generic_string_return_profile(function, targets))
    }
}

#[cfg(test)]
pub(super) fn generic_string_return_profile_test_cache_key(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> String {
    generic_string_return_profile_cache_key(function, targets)
}

fn generic_string_return_profile_cache_key(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> String {
    let mut key = function.signature.name.clone();
    let mut callees = BTreeSet::<&str>::new();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                ..
            } = instruction
            {
                callees.insert(name.as_str());
            }
        }
    }
    for callee in callees {
        key.push('|');
        key.push_str(callee);
        match lookup_global_call_target(callee, targets) {
            Some(target) => {
                let _ = write!(
                    key,
                    ":{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
                    target.shape(),
                    target.return_contract(),
                    target.proof(),
                    target.shape_reason(),
                    target.shape_blocker_symbol(),
                    target.shape_blocker_reason()
                );
            }
            None => key.push_str(":missing"),
        }
    }
    key
}
