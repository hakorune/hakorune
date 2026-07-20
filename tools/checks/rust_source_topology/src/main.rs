use std::path::PathBuf;

use rust_source_topology_check::extract_single_file_source;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage());
    };
    if command != "single-file" {
        return Err(format!(
            "[rust-source-topology/unknown-command] {command}\n{}",
            usage()
        ));
    }
    let Some(input) = args.next() else {
        return Err(usage());
    };
    let mut module_syntax_path = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--module-syntax-path" => {
                module_syntax_path = args.next();
                if module_syntax_path.is_none() {
                    return Err(usage());
                }
            }
            _ => {
                return Err(format!(
                    "[rust-source-topology/unknown-argument] {flag}\n{}",
                    usage()
                ));
            }
        }
    }
    let module_syntax_path = module_syntax_path.ok_or_else(usage)?;
    let input_path = PathBuf::from(&input);
    let source = std::fs::read_to_string(&input_path).map_err(|error| {
        format!(
            "[rust-source-topology/read-failed] path={} detail={error}",
            input_path.display()
        )
    })?;
    let topology = extract_single_file_source(&input, &module_syntax_path, &source)
        .map_err(|error| error.to_string())?;
    let mut output = serde_json::to_string_pretty(&topology)
        .map_err(|error| format!("[rust-source-topology/json-failed] {error}"))?;
    output.push('\n');
    print!("{output}");
    Ok(())
}

fn usage() -> String {
    "usage: rust-source-topology-check single-file <path> --module-syntax-path <syntax-path>"
        .to_string()
}
