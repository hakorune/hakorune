use super::{
    failfast_error, module_to_mir_json, with_phase0_mir_json_env, Stage1UserBoxDecls, FAILFAST_TAG,
};

pub(super) struct Stage1ProgramJsonInput<'a> {
    program_json: &'a str,
}

pub(super) struct Stage1ProgramJsonValue {
    program_value: serde_json::Value,
}

pub(super) struct Stage1ProgramJsonModuleHandoff {
    module: crate::mir::MirModule,
    user_box_decls: Stage1UserBoxDecls,
}

impl<'a> Stage1ProgramJsonInput<'a> {
    pub(super) fn new(program_json: &'a str) -> Self {
        Self { program_json }
    }

    pub(super) fn into_module_handoff(self) -> Result<Stage1ProgramJsonModuleHandoff, String> {
        Ok(Stage1ProgramJsonModuleHandoff {
            module: self.parse_module()?,
            user_box_decls: self.parse_value()?.resolve_user_box_decls(),
        })
    }

    pub(super) fn parse_value(&self) -> Result<Stage1ProgramJsonValue, String> {
        Stage1ProgramJsonValue::parse(self.program_json)
    }

    fn parse_module(&self) -> Result<crate::mir::MirModule, String> {
        crate::runner::json_v0_bridge::parse_json_v0_to_module(self.program_json)
            .map_err(failfast_error)
    }
}

impl Stage1ProgramJsonValue {
    fn parse(program_json: &str) -> Result<Self, String> {
        serde_json::from_str(program_json)
            .map(|program_value| Self { program_value })
            .map_err(|error| format!("program json parse error: {}", error))
    }

    pub(super) fn resolve_user_box_decls(&self) -> Stage1UserBoxDecls {
        Stage1UserBoxDecls::from_program_value(&self.program_value)
    }
}

impl Stage1ProgramJsonModuleHandoff {
    pub(super) fn parse(program_json: &str) -> Result<Self, String> {
        Stage1ProgramJsonInput::new(program_json).into_module_handoff()
    }

    pub(super) fn emit_mir_json(self) -> Result<String, String> {
        module_to_mir_json(&self.into_module_with_user_box_decls())
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        with_phase0_mir_json_env(|| self.emit_mir_json())
    }

    fn into_module_with_user_box_decls(self) -> crate::mir::MirModule {
        let mut module = self.module;
        module.metadata.user_box_decls = self.user_box_decls.into_metadata_map();
        module
    }
}

pub(super) struct SourceProgramJsonHandoff {
    program_json: String,
}

impl SourceProgramJsonHandoff {
    pub(super) fn for_source(source_text: &str) -> Result<Self, String> {
        Ok(Self {
            program_json: Self::emit_strict_program_json(source_text)?,
        })
    }

    pub(super) fn emit_guarded_program_and_mir_json(self) -> Result<(String, String), String> {
        let mir_json =
            Stage1ProgramJsonModuleHandoff::parse(&self.program_json)?.emit_guarded_mir_json()?;
        Ok((self.program_json, mir_json))
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        self.emit_guarded_program_and_mir_json()
            .map(|(_, mir_json)| mir_json)
    }

    #[cfg(test)]
    pub(super) fn emit_plain_program_and_mir_json(self) -> Result<(String, String), String> {
        let mir_json = super::lowering::program_json_to_mir_json(&self.program_json)?;
        Ok((self.program_json, mir_json))
    }

    fn emit_strict_program_json(source_text: &str) -> Result<String, String> {
        crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(
            source_text,
        )
        .map_err(|error| format!("{FAILFAST_TAG} {}", error))
    }
}
