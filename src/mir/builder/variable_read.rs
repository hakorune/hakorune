//! Variable reads and undefined-variable diagnostics.

use super::{MirBuilder, ValueId};

impl MirBuilder {
    /// Build variable access.
    pub(in crate::mir::builder) fn build_variable_access(
        &mut self,
        name: String,
    ) -> Result<ValueId, String> {
        // Step 5-5-G: __pin$ variables should NEVER be accessed from variable_map.
        if name.starts_with("__pin$") {
            return Err(format!(
                "COMPILER BUG: Attempt to access __pin$ temporary '{}' from variable_map. \
                 __pin$ variables should only exist as direct SSA values, not as named variables.",
                name
            ));
        }

        if let Some(&value_id) = self.function_state.variable_ctx.variable_map.get(&name) {
            self.fail_if_record_value_escape_by_name(&name, value_id)?;
            // Debug-only observation: check if variable_map value is defined.
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some(func) = self.function_state.current_function.as_ref() {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                    if !def_blocks.contains_key(&value_id) {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[call/arg_build:undefined_value] fn={} bb={:?} var_name={} v=%{} ast=Variable span=n/a next={}",
                            func.signature.name,
                            self.function_state.current_block,
                            name,
                            value_id.0,
                            func.next_value_id
                        ));
                    }
                }
            }
            Ok(value_id)
        } else {
            Err(self.undefined_variable_message(&name))
        }
    }

    pub(in crate::mir::builder) fn undefined_variable_message(&self, name: &str) -> String {
        let mut msg = format!("Undefined variable: {}", name);

        if name == "local" && !crate::config::env::parser_stage3_enabled() {
            msg.push_str("\nHint: 'local' is a syntax-3 keyword. Prefer NYASH_FEATURES=stage3 (legacy: NYASH_PARSER_STAGE3=1 / HAKO_PARSER_STAGE3=1 for mode-B compatibility routes).");
            msg.push_str(
                "\nFor AotPrep verification, use tools/hakorune_emit_mir.sh which sets these automatically.",
            );
        } else if (name == "flow" || name == "try" || name == "catch" || name == "throw")
            && !crate::config::env::parser_stage3_enabled()
        {
            msg.push_str(&format!(
                "\nHint: '{}' is a syntax-3 keyword. Prefer NYASH_FEATURES=stage3 (legacy: NYASH_PARSER_STAGE3=1 / HAKO_PARSER_STAGE3=1 for mode-B compatibility routes).",
                name
            ));
        }

        let suggest = crate::using::simple_registry::suggest_using_for_symbol(name);
        if !suggest.is_empty() {
            msg.push_str("\nHint: symbol appears in using module(s): ");
            msg.push_str(&suggest.join(", "));
            msg.push_str(
                "\nConsider adding 'using <module> [as Alias]' or check nyash.toml [using].",
            );
        }

        msg
    }
}
