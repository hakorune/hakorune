use super::super::ast::FuncDefV0;
use crate::mir::Callee;
use std::collections::{BTreeMap, BTreeSet};

/// Immutable source-bound target facts for generic Program JSON-v0 calls.
///
/// Imported alias calls are intentionally handled by their existing canonical
/// producers. This catalog only owns local definition membership and the
/// source name/arity relation for `ExprV0::Call`.
#[derive(Clone, Debug, Default)]
pub(super) struct ProgramCallTargetCatalog {
    by_short_arity: BTreeMap<(String, usize), Vec<String>>,
    qualified_names: BTreeSet<String>,
}

impl ProgramCallTargetCatalog {
    pub(super) fn from_defs(defs: &[FuncDefV0]) -> Result<Self, String> {
        let mut catalog = Self::default();
        for def in defs {
            if is_stageb_entry_def(def) {
                continue;
            }
            let qualified = qualified_function_name(def);
            if !catalog.qualified_names.insert(qualified.clone()) {
                return Err(format!(
                    "[json-v0/call-target/duplicate-definition] {qualified}"
                ));
            }
            catalog
                .by_short_arity
                .entry((def.name.clone(), def.params.len()))
                .or_default()
                .push(qualified);
        }
        Ok(catalog)
    }

    pub(super) fn resolve(&self, name: &str, args_len: usize) -> Result<Callee, String> {
        if name.is_empty() {
            return Err("[json-v0/call-target/empty-name]".to_string());
        }

        if is_extern_name(name) {
            return Ok(Callee::Extern(strip_extern_arity(name).to_string()));
        }

        if has_explicit_arity(name) {
            return Ok(Callee::Global(name.to_string()));
        }

        if name.contains('.') {
            let qualified = format!("{name}/{args_len}");
            if self.qualified_names.contains(&qualified) {
                return Ok(Callee::Global(qualified));
            }
            return Ok(Callee::Global(name.to_string()));
        }

        if let Some(candidates) = self.by_short_arity.get(&(name.to_string(), args_len)) {
            return match candidates.as_slice() {
                [qualified] => Ok(Callee::Global(qualified.clone())),
                _ => Err(format!(
                    "[json-v0/call-target/ambiguous-name] name={name} arity={args_len} candidates={}",
                    candidates.join(",")
                )),
            };
        }
        Ok(Callee::Global(name.to_string()))
    }
}

pub(super) fn is_stageb_entry_def(def: &FuncDefV0) -> bool {
    def.box_name == "Main" && def.name == "main"
}

pub(super) fn qualified_function_name(def: &FuncDefV0) -> String {
    format!("{}.{}/{}", def.box_name, def.name, def.params.len())
}

fn is_extern_name(name: &str) -> bool {
    name.starts_with("env.") || name.starts_with("nyash.")
}

fn strip_extern_arity(name: &str) -> &str {
    match name.rsplit_once('/') {
        Some((base, arity)) if arity.chars().all(|c| c.is_ascii_digit()) => base,
        _ => name,
    }
}

fn has_explicit_arity(name: &str) -> bool {
    matches!(
        name.rsplit_once('/'),
        Some((_base, arity)) if arity.chars().all(|c| c.is_ascii_digit())
    )
}
