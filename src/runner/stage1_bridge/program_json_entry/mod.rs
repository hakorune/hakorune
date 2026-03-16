use crate::cli::CliGroups;

mod exit;
mod request;

pub(in crate::runner) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    request::emit_program_json_v0_requested(groups)
}

pub(in crate::runner) fn emit_program_json_v0_and_exit(groups: &CliGroups) -> ! {
    let request = match request::ProgramJsonEmitRequest::build(groups) {
        Ok(request) => request,
        Err(error) => exit::exit_with_emit_program_json_error(&error),
    };
    let out_path = request.out_path.clone();
    exit::exit_with_emit_program_json_result(&out_path, request.emit_program_json_v0())
}
