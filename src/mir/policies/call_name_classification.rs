//! Neutral call-name classification shared by Raw admission and Callee resolution.
//!
//! The two facts intentionally remain independent. Raw unified admission and
//! Callee resolution have different historical name sets, and collapsing them
//! would change production routing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallNameCalleeClassV1 {
    BuiltinGlobal,
    Extern,
    Ordinary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CallNameClassificationV1 {
    raw_unified_admission: bool,
    callee_class: CallNameCalleeClassV1,
}

impl CallNameClassificationV1 {
    pub(crate) fn raw_unified_admission(self) -> bool {
        self.raw_unified_admission
    }

    pub(crate) fn callee_class(self) -> CallNameCalleeClassV1 {
        self.callee_class
    }
}

pub(crate) fn classify_call_name_v1(name: &str) -> CallNameClassificationV1 {
    let (raw_unified_admission, callee_class) = match name {
        "print" | "error" | "panic" | "exit" | "now" | "abs" | "min" | "max" => {
            (true, CallNameCalleeClassV1::BuiltinGlobal)
        }
        "isType" | "asType" => (true, CallNameCalleeClassV1::Ordinary),
        "gc_collect" | "gc_stats" | "sin" | "cos" => (false, CallNameCalleeClassV1::BuiltinGlobal),
        name if name.starts_with("nyash.") => (true, CallNameCalleeClassV1::Extern),
        name if name.starts_with("env.") || name.starts_with("system.") => {
            (false, CallNameCalleeClassV1::Extern)
        }
        _ => (false, CallNameCalleeClassV1::Ordinary),
    };

    CallNameClassificationV1 {
        raw_unified_admission,
        callee_class,
    }
}

pub(crate) fn is_math_function(name: &str) -> bool {
    matches!(
        name,
        "sin" | "cos" | "abs" | "min" | "max" | "sqrt" | "pow" | "floor" | "ceil"
    )
}

pub(crate) fn is_parked_file_special_form_v1(name: &str) -> bool {
    name == "panic"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsupportedBareProviderV1 {
    Error,
    Now,
}

impl UnsupportedBareProviderV1 {
    pub(crate) fn diagnostic(self, arity: usize) -> String {
        let name = match self {
            Self::Error => "error",
            Self::Now => "now",
        };
        format!("[freeze:contract][direct-call/bare-{name}-unsupported] name={name} arity={arity}")
    }
}

pub(crate) fn unsupported_bare_provider_v1(name: &str) -> Option<UnsupportedBareProviderV1> {
    match name {
        "error" => Some(UnsupportedBareProviderV1::Error),
        "now" => Some(UnsupportedBareProviderV1::Now),
        _ => None,
    }
}

/// File-mode reservations. Print syntax and FastMemory receiver policy have
/// their own AST forms. The legacy builtin names below issue no file target.
pub(crate) fn is_reserved_source_function_call_v1(name: &str) -> bool {
    matches!(name, "print" | "panic" | "gc_collect" | "gc_stats" | "isType" | "asType")
        || unsupported_bare_provider_v1(name).is_some()
        || name.starts_with("nyash.")
        || name.starts_with("env.")
        || name.starts_with("system.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_the_two_existing_call_name_dimensions() {
        let cases = [
            ("print", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("error", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("panic", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("exit", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("now", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("abs", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("min", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("max", true, CallNameCalleeClassV1::BuiltinGlobal),
            ("isType", true, CallNameCalleeClassV1::Ordinary),
            ("asType", true, CallNameCalleeClassV1::Ordinary),
            ("gc_collect", false, CallNameCalleeClassV1::BuiltinGlobal),
            ("gc_stats", false, CallNameCalleeClassV1::BuiltinGlobal),
            ("sin", false, CallNameCalleeClassV1::BuiltinGlobal),
            ("cos", false, CallNameCalleeClassV1::BuiltinGlobal),
            ("nyash.", true, CallNameCalleeClassV1::Extern),
            ("nyash.fs.read", true, CallNameCalleeClassV1::Extern),
            ("nyashx", false, CallNameCalleeClassV1::Ordinary),
            ("env.", false, CallNameCalleeClassV1::Extern),
            ("env.console.log", false, CallNameCalleeClassV1::Extern),
            ("envx", false, CallNameCalleeClassV1::Ordinary),
            ("system.", false, CallNameCalleeClassV1::Extern),
            ("system.exit", false, CallNameCalleeClassV1::Extern),
            ("systemx", false, CallNameCalleeClassV1::Ordinary),
            ("ordinary", false, CallNameCalleeClassV1::Ordinary),
            ("", false, CallNameCalleeClassV1::Ordinary),
        ];

        for (name, raw_unified_admission, callee_class) in cases {
            let actual = classify_call_name_v1(name);
            assert_eq!(
                actual.raw_unified_admission(),
                raw_unified_admission,
                "Raw unified admission drift for {name}"
            );
            assert_eq!(
                actual.callee_class(),
                callee_class,
                "Callee class drift for {name}"
            );
        }
    }
}
