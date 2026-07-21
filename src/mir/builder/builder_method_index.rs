use super::function_signature_lookup::FunctionSignatureLookupV1;
use super::MirBuilder;

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

    /// Port-aware sibling for method-tail index projection.  It consumes only
    /// the explicit completed-header inventory and never reaches through a
    /// module storage fallback.
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn rebuild_method_tail_index_with_headers(
        &mut self,
        headers: &dyn FunctionSignatureLookupV1,
    ) {
        let mut names = Vec::with_capacity(headers.symbol_count());
        headers.visit_symbols(&mut |symbol| names.push(symbol.to_owned()));
        self.rebuild_method_tail_index_from_names(names, headers.symbol_count());
    }

    fn rebuild_method_tail_index_from_names(&mut self, names: Vec<String>, source_len: usize) {
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
        let tail = format!(".{}{}", method, format!("/{}", arity));
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
