mod cli;
mod exprs;
mod functions;
mod items;
mod names;
mod stmts;
mod types;

use crate::cli::{fail, parse_args, write_output};
use crate::items::file_to_json;

fn main() {
    let args = parse_args();
    let source = std::fs::read_to_string(&args.input)
        .unwrap_or_else(|err| fail(format!("failed to read {}: {err}", args.input.display())));
    let file = syn::parse_file(&source)
        .unwrap_or_else(|err| fail(format!("failed to parse {}: {err}", args.input.display())));

    let mut text = serde_json::to_string_pretty(&file_to_json(&file, args.module))
        .unwrap_or_else(|err| fail(format!("failed to serialize adapter JSON: {err}")));
    text.push('\n');

    write_output(args.output, text);
}
