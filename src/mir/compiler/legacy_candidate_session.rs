//! Default Legacy compilation through a success-only Builder candidate.

use std::{collections::HashMap, time::Instant};

use crate::ast::ASTNode;
use crate::mir::builder::{BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1};

use super::{MirCompileResult, MirCompiler, MirFinishScheduleV1};

impl MirCompiler {
    pub(super) fn compile_legacy_candidate(
        &mut self,
        ast: ASTNode,
        source_file: Option<&str>,
        imports: HashMap<String, String>,
    ) -> Result<MirCompileResult, String> {
        let token = self
            .invocation_identity
            .issue_raw()
            .map_err(|error| error.to_string())?;
        let config = BuilderInvocationConfigV1::snapshot_for_raw_with_imports(
            &self.builder,
            source_file,
            imports,
        );
        let mut session =
            ModuleBuilderInvocationSessionV1::open_for_token(&token, &self.builder, config);

        let stage_start = Instant::now();
        let module = session.builder_mut().build_module(ast)?;
        super::super::compile_timing::trace_stage("build_module", stage_start.elapsed());

        let result = self.finish_built_module(module, MirFinishScheduleV1::Legacy)?;
        let prepared = session
            .prepare_external_commit()
            .map_err(|error| error.to_string())?;
        let _receipt = prepared.commit(&mut self.builder);
        Ok(result)
    }
}
