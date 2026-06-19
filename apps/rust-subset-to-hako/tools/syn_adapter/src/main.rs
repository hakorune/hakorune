mod cli;
mod crate_mode;
mod exprs;
mod functions;
mod items;
mod names;
mod stmts;
mod types;

use crate::cli::{fail, parse_args, write_output, Command};
use crate::crate_mode::write_crate_bundle;
use crate::items::file_to_json;

fn main() {
    match parse_args() {
        Command::SingleFile {
            input,
            output,
            module,
        } => write_single_file(&input, output, module),
        Command::Crate {
            crate_root,
            out_dir,
            crate_name,
            target_kind,
            target_name,
        } => write_crate_bundle(
            &crate_root,
            &out_dir,
            &crate_name,
            &target_kind,
            &target_name,
        ),
    }
}

fn write_single_file(input: &std::path::Path, output: Option<std::path::PathBuf>, module: String) {
    let source = std::fs::read_to_string(input)
        .unwrap_or_else(|err| fail(format!("failed to read {}: {err}", input.display())));
    let file = syn::parse_file(&source)
        .unwrap_or_else(|err| fail(format!("failed to parse {}: {err}", input.display())));

    let mut text = serde_json::to_string_pretty(&file_to_json(&file, module))
        .unwrap_or_else(|err| fail(format!("failed to serialize adapter JSON: {err}")));
    text.push('\n');

    write_output(output, text);
}
