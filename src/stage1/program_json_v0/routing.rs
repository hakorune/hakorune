//! Source-shape / build-route SSOT for Stage1 Program(JSON v0).

const RELAXED_KEEP_REASON_DEV_LOCAL_ALIAS_SUGAR: &str = "dev-local-alias-sugar";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProgramJsonV0SourceShape {
    StrictSafe,
    NeedsRelaxedCompat(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ProgramJsonV0SourceShapeInfo {
    label: &'static str,
    relaxed_reason: Option<&'static str>,
}

impl ProgramJsonV0SourceShapeInfo {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn relaxed_reason(&self) -> Option<&'static str> {
        self.relaxed_reason
    }

    pub fn strict_authority_rejection(&self, consumer: &str) -> Option<String> {
        self.relaxed_reason().map(|reason| {
            format!(
                "{} rejects compat-only {} source shape ({})",
                consumer,
                self.label(),
                reason
            )
        })
    }
}

impl ProgramJsonV0SourceShape {
    pub fn label(&self) -> &'static str {
        match self {
            Self::StrictSafe => "strict-safe",
            Self::NeedsRelaxedCompat(_) => "relaxed-compat",
        }
    }

    pub fn relaxed_reason(&self) -> Option<&'static str> {
        match self {
            Self::NeedsRelaxedCompat(reason) => Some(*reason),
            Self::StrictSafe => None,
        }
    }

    fn info(&self) -> ProgramJsonV0SourceShapeInfo {
        ProgramJsonV0SourceShapeInfo {
            label: self.label(),
            relaxed_reason: self.relaxed_reason(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProgramJsonV0BuildRoute {
    StrictAuthority,
    StrictDefault,
    RelaxedCompat(&'static str),
}

impl ProgramJsonV0BuildRoute {
    #[cfg(test)]
    pub fn label(&self) -> &'static str {
        match self {
            Self::StrictAuthority => "strict-authority",
            Self::StrictDefault => "strict-default",
            Self::RelaxedCompat(_) => "relaxed-compat",
        }
    }

    #[cfg(test)]
    pub fn relaxed_reason(&self) -> Option<&'static str> {
        match self {
            Self::RelaxedCompat(reason) => Some(*reason),
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn trace_relaxed_reason(&self) -> &'static str {
        self.relaxed_reason().unwrap_or("none")
    }

    #[cfg(test)]
    pub fn trace_summary(&self) -> String {
        format!(
            "route={} relaxed_reason={}",
            self.label(),
            self.trace_relaxed_reason()
        )
    }

    fn emit_program_json(&self, source_text: &str) -> Result<String, String> {
        match self {
            Self::StrictAuthority | Self::StrictDefault => {
                super::source_to_program_json_v0_strict(source_text)
            }
            Self::RelaxedCompat(_) => super::source_to_program_json_v0_relaxed(source_text),
        }
    }
}

pub(super) struct ProgramJsonV0BuildEmission {
    #[cfg(test)]
    route: ProgramJsonV0BuildRoute,
    program_json: String,
}

impl ProgramJsonV0BuildEmission {
    fn from_route(route: ProgramJsonV0BuildRoute, program_json: String) -> Self {
        #[cfg(not(test))]
        let _ = route;
        Self {
            #[cfg(test)]
            route,
            program_json,
        }
    }

    pub(super) fn into_program_json(self) -> String {
        self.program_json
    }

    #[cfg(test)]
    pub(super) fn trace_summary(&self) -> String {
        self.route.trace_summary()
    }
}

fn classify_program_json_v0_source_shape_internal(source_text: &str) -> ProgramJsonV0SourceShape {
    if super::extract::has_dev_local_alias_sugar(source_text) {
        ProgramJsonV0SourceShape::NeedsRelaxedCompat(RELAXED_KEEP_REASON_DEV_LOCAL_ALIAS_SUGAR)
    } else {
        ProgramJsonV0SourceShape::StrictSafe
    }
}

pub(super) fn classify_program_json_v0_source_shape(
    source_text: &str,
) -> ProgramJsonV0SourceShapeInfo {
    classify_program_json_v0_source_shape_internal(source_text).info()
}

pub(super) fn strict_authority_program_json_v0_source_rejection(
    source_text: &str,
    consumer: &str,
) -> Option<String> {
    classify_program_json_v0_source_shape(source_text).strict_authority_rejection(consumer)
}

fn select_program_json_v0_build_route(
    source_text: &str,
    strict_authority_mode: bool,
) -> ProgramJsonV0BuildRoute {
    if strict_authority_mode {
        ProgramJsonV0BuildRoute::StrictAuthority
    } else {
        match classify_program_json_v0_source_shape_internal(source_text) {
            ProgramJsonV0SourceShape::StrictSafe => ProgramJsonV0BuildRoute::StrictDefault,
            ProgramJsonV0SourceShape::NeedsRelaxedCompat(reason) => {
                ProgramJsonV0BuildRoute::RelaxedCompat(reason)
            }
        }
    }
}

fn emit_program_json_v0_for_build_route(
    source_text: &str,
    strict_authority_mode: bool,
) -> Result<ProgramJsonV0BuildEmission, String> {
    let route = select_program_json_v0_build_route(source_text, strict_authority_mode);
    let program_json = route.emit_program_json(source_text)?;
    Ok(ProgramJsonV0BuildEmission::from_route(route, program_json))
}

pub(super) fn emit_stage1_build_box_program_json(
    source_text: &str,
    strict_authority_mode: bool,
) -> Result<ProgramJsonV0BuildEmission, String> {
    if strict_authority_mode {
        let program_json = super::emit_program_json_v0_for_strict_authority_source(source_text)?;
        return Ok(ProgramJsonV0BuildEmission::from_route(
            ProgramJsonV0BuildRoute::StrictAuthority,
            program_json,
        ));
    }
    emit_program_json_v0_for_build_route(source_text, false)
}
