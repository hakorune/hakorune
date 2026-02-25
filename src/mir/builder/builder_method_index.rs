use super::MirBuilder;

impl MirBuilder {
    // ----------------------
    // Method tail index (performance helper)
    // ----------------------
    fn rebuild_method_tail_index(&mut self) {
        self.comp_ctx.method_tail_index.clear();
        if let Some(ref module) = self.current_module {
            for name in module.functions.keys() {
                if let (Some(dot), Some(slash)) = (name.rfind('.'), name.rfind('/')) {
                    if slash > dot {
                        let tail = &name[dot..];
                        self.comp_ctx
                            .method_tail_index
                            .entry(tail.to_string())
                            .or_insert_with(Vec::new)
                            .push(name.clone());
                    }
                }
            }
            self.comp_ctx.method_tail_index_source_len = module.functions.len();
        } else {
            self.comp_ctx.method_tail_index_source_len = 0;
        }
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
