use std::path::PathBuf;

pub(crate) struct Args {
    pub(crate) input: PathBuf,
    pub(crate) output: Option<PathBuf>,
    pub(crate) module: String,
}

pub(crate) fn fail(msg: impl AsRef<str>) -> ! {
    eprintln!("[rust-subset-syn-adapter] ERROR: {}", msg.as_ref());
    std::process::exit(1);
}

pub(crate) fn parse_args() -> Args {
    let mut input = None;
    let mut output = None;
    let mut module = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => output = args.next().map(PathBuf::from),
            "--module" => module = args.next(),
            "-h" | "--help" => {
                println!(
                    "usage: rust-subset-syn-adapter <input.rs> [-o output.json] [--module name]"
                );
                std::process::exit(0);
            }
            _ if input.is_none() => input = Some(PathBuf::from(arg)),
            _ => fail(format!("unexpected argument: {arg}")),
        }
    }

    let input = input.unwrap_or_else(|| fail("missing input .rs path"));
    let module = module.unwrap_or_else(|| {
        input
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("module")
            .to_string()
    });

    Args {
        input,
        output,
        module,
    }
}

pub(crate) fn write_output(output: Option<PathBuf>, text: String) {
    if let Some(output) = output {
        std::fs::write(&output, text)
            .unwrap_or_else(|err| fail(format!("failed to write {}: {err}", output.display())));
    } else {
        println!("{text}");
    }
}
