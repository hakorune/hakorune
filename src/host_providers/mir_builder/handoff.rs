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

pub(super) struct Stage1FinalizedMirModule {
    module: crate::mir::MirModule,
}

impl Stage1ProgramJsonModuleHandoff {
    pub(super) fn new(module: crate::mir::MirModule, user_box_decls: Stage1UserBoxDecls) -> Self {
        Self {
            module,
            user_box_decls,
        }
    }

    pub(super) fn parse(program_json: &str) -> Result<Self, String> {
        Stage1ProgramJsonInput::new(program_json).into_module_handoff()
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        self.into_finalized_module().emit_guarded_mir_json()
    }

    pub(super) fn into_finalized_module(self) -> Stage1FinalizedMirModule {
        let mut module = self.module;
        let (user_box_decls, user_box_field_decls) = self.user_box_decls.into_metadata_maps();
        module.metadata.user_box_decls = user_box_decls;
        module.metadata.user_box_field_decls = user_box_field_decls;
        Stage1FinalizedMirModule { module }
    }
}

pub(super) struct SourceProgramJsonOutputHandoff {
    program_json: String,
}

pub(super) struct SourceProgramJsonAuthority;

impl<'a> Stage1ProgramJsonInput<'a> {
    pub(super) fn new(program_json: &'a str) -> Self {
        Self { program_json }
    }

    pub(super) fn into_module_handoff(self) -> Result<Stage1ProgramJsonModuleHandoff, String> {
        let module = self.parse_module()?;
        let program_value = self.parse_value()?;
        Ok(program_value.into_module_handoff(module))
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

    fn into_module_handoff(self, module: crate::mir::MirModule) -> Stage1ProgramJsonModuleHandoff {
        let user_box_decls = self.resolve_user_box_decls();
        Stage1ProgramJsonModuleHandoff::new(module, user_box_decls)
    }
}

impl Stage1FinalizedMirModule {
    pub(super) fn emit_mir_json(self) -> Result<String, String> {
        module_to_mir_json(&self.module)
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        with_phase0_mir_json_env(|| self.emit_mir_json())
    }
}

impl SourceProgramJsonAuthority {
    pub(super) fn for_source(source_text: &str) -> Result<SourceProgramJsonOutputHandoff, String> {
        Ok(SourceProgramJsonOutputHandoff {
            program_json: Self::emit_strict_program_json(source_text)?,
        })
    }

    fn emit_strict_program_json(source_text: &str) -> Result<String, String> {
        crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(
            source_text,
        )
        .map_err(|error| format!("{FAILFAST_TAG} {}", error))
    }
}

impl SourceProgramJsonOutputHandoff {
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
}
