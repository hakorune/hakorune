/*!
 * Call Target Types
 *
 * Type-safe call target specification for unified call system
 * Part of Phase 15.5 MIR Call unification
 */

use crate::mir::ValueId;
use hakorune_mir_defs::CanonicalGlobalTargetV1;

/// Carry a symbol that an older, already-selected route has produced into
/// the typed carrier.  This is a compatibility adapter only: it performs no
/// catalog lookup, fallback, or retry.  Source-backed issuers should use the
/// declaration-key constructors directly and the adapter is slated to leave
/// with the legacy routes.
pub(crate) fn typed_global_target_from_selected_symbol(
    symbol: &str,
    fallback_arity: usize,
) -> Result<CanonicalGlobalTargetV1, String> {
    if symbol == "print" {
        return Ok(CanonicalGlobalTargetV1::builtin_print());
    }
    let (qualified, encoded_arity) = match symbol.rsplit_once('/') {
        Some((qualified, encoded)) => (
            qualified,
            encoded.parse::<u32>().map_err(|_| {
                format!("[freeze:contract][global-target/malformed-arity] {symbol}")
            })?,
        ),
        None => (
            symbol,
            u32::try_from(fallback_arity)
                .map_err(|_| format!("[freeze:contract][global-target/arity-overflow] {symbol}"))?,
        ),
    };
    if let Some((owner, method)) = qualified.rsplit_once('.') {
        return CanonicalGlobalTargetV1::new_static_box_method(
            owner.into(),
            method.into(),
            encoded_arity,
        )
        .map_err(|error| format!("[freeze:contract][global-target/{error:?}]"));
    }
    CanonicalGlobalTargetV1::new_free_function(qualified.into(), encoded_arity)
        .map_err(|error| format!("[freeze:contract][global-target/{error:?}]"))
}

/// Call target specification for emit_unified_call
/// Provides type-safe target resolution at the builder level
#[derive(Debug, Clone)]
pub enum CallTarget {
    /// Global function selected by the declaration-backed call authority.
    Global(CanonicalGlobalTargetV1),

    /// Method call (box.method)
    Method {
        box_type: Option<String>, // None = infer from value
        method: String,
        receiver: ValueId,
    },

    /// Constructor (new BoxType)
    Constructor(String),

    /// External function (nyash.*)
    Extern(String),

    /// Dynamic function value
    Value(ValueId),

    /// Closure creation
    Closure {
        params: Vec<String>,
        captures: Vec<(String, ValueId)>,
        me_capture: Option<ValueId>,
    },
}

impl CallTarget {
    /// Check if this target is a constructor
    pub fn is_constructor(&self) -> bool {
        matches!(self, CallTarget::Constructor(_))
    }

    /// Get the name of the target for debugging
    pub fn name(&self) -> String {
        match self {
            CallTarget::Global(target) => target.display_name(),
            CallTarget::Method { method, .. } => method.clone(),
            CallTarget::Constructor(box_type) => format!("new {}", box_type),
            CallTarget::Extern(name) => name.clone(),
            CallTarget::Value(_) => "<dynamic>".to_string(),
            CallTarget::Closure { .. } => "<closure>".to_string(),
        }
    }
}
