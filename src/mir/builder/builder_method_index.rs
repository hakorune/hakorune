use super::function_signature_lookup::FunctionSignatureLookupV1;
use super::MirBuilder;

fn method_tail_key(method: &str, arity: usize) -> String {
    format!(".{method}/{arity}")
}

/// Project a deterministic method candidate list from an explicit symbol
/// inventory.  This is the shared tail policy for both the legacy index and
/// header-port readers; it does not inspect module storage or mutate caches.
pub(in crate::mir::builder) fn method_candidates_from_symbols(
    symbols: impl IntoIterator<Item = String>,
    method: &str,
    arity: usize,
) -> Vec<String> {
    let tail = method_tail_key(method, arity);
    let mut candidates = symbols
        .into_iter()
        .filter(|symbol| symbol.contains('.') && symbol.ends_with(&tail))
        .collect::<Vec<_>>();
    candidates.sort();
    candidates
}

/// Header-port projection of the same method-tail policy used by the legacy
/// index.  The lookup remains borrowed for this call only.
pub(in crate::mir::builder) fn method_candidates_from_headers(
    headers: &dyn FunctionSignatureLookupV1,
    method: &str,
    arity: usize,
) -> Vec<String> {
    let mut symbols = Vec::with_capacity(headers.symbol_count());
    headers.visit_symbols(&mut |symbol| symbols.push(symbol.to_owned()));
    method_candidates_from_symbols(symbols, method, arity)
}

impl MirBuilder {
    // ----------------------
    // Method tail index (performance helper)
    // ----------------------
    fn rebuild_method_tail_index(&mut self) {
        self.comp_ctx.method_tail_index.clear();
        let Some(module) = self.current_module.as_ref() else {
            self.comp_ctx.method_tail_index_source_len = 0;
            return;
        };
        let source_len = module.functions.len();
        let mut names = Vec::with_capacity(source_len);
        names.extend(module.functions.keys().cloned());
        self.rebuild_method_tail_index_from_names(names, source_len);
    }

    fn rebuild_method_tail_index_from_names(&mut self, mut names: Vec<String>, source_len: usize) {
        names.sort();
        for name in names {
            if let (Some(dot), Some(slash)) = (name.rfind('.'), name.rfind('/')) {
                if slash > dot {
                    let tail = &name[dot..];
                    self.comp_ctx
                        .method_tail_index
                        .entry(tail.to_string())
                        .or_insert_with(Vec::new)
                        .push(name);
                }
            }
        }
        for candidates in self.comp_ctx.method_tail_index.values_mut() {
            candidates.sort();
        }
        self.comp_ctx.method_tail_index_source_len = source_len;
    }

    fn ensure_method_tail_index(&mut self) {
        let need_rebuild = match self.current_module {
            Some(ref refmod) => {
                self.comp_ctx.method_tail_index_source_len != refmod.functions.len()
            }
            None => self.comp_ctx.method_tail_index_source_len != 0,
        };
        if need_rebuild {
            self.rebuild_method_tail_index();
        }
    }

    pub(super) fn method_candidates(&mut self, method: &str, arity: usize) -> Vec<String> {
        self.ensure_method_tail_index();
        let tail = method_tail_key(method, arity);
        self.comp_ctx
            .method_tail_index
            .get(&tail)
            .cloned()
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    pub(super) fn method_candidates_tail<S: AsRef<str>>(&mut self, tail: S) -> Vec<String> {
        self.ensure_method_tail_index();
        self.comp_ctx
            .method_tail_index
            .get(tail.as_ref())
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::method_candidates_from_symbols;

    #[test]
    fn explicit_projection_is_sorted_and_ignores_non_methods() {
        let candidates = method_candidates_from_symbols(
            vec![
                "Zulu.run/1".to_owned(),
                "not-a-method".to_owned(),
                "Alpha.run/1".to_owned(),
                "Alpha.run/2".to_owned(),
            ],
            "run",
            1,
        );
        assert_eq!(candidates, vec!["Alpha.run/1", "Zulu.run/1"]);
    }
}
