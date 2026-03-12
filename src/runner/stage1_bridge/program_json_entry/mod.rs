use crate::cli::CliGroups;

mod exit;
mod request;

pub(in crate::runner) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    request::emit_program_json_v0_requested(groups)
}

pub(in crate::runner) fn emit_program_json_v0_and_exit(groups: &CliGroups) -> ! {
    let request = match request::build_emit_request(groups) {
        Ok(request) => request,
        Err(error) => exit::exit_with_emit_program_json_error(&error),
    };
    exit::exit_with_emit_program_json_result(
        &request.out_path,
        super::program_json::emit_program_json_v0(&request.source_path, &request.out_path),
    )
}
