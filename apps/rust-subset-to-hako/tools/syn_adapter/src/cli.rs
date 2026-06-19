use std::path::PathBuf;

pub(crate) enum Command {
    SingleFile {
        input: PathBuf,
        output: Option<PathBuf>,
        module: String,
    },
    Crate {
        crate_root: PathBuf,
        out_dir: PathBuf,
        crate_name: String,
        target_kind: String,
        target_name: String,
    },
}

pub(crate) fn fail(msg: impl AsRef<str>) -> ! {
    eprintln!("[rust-subset-syn-adapter] ERROR: {}", msg.as_ref());
    std::process::exit(1);
}

pub(crate) fn parse_args() -> Command {
    let mut input = None;
    let mut output = None;
    let mut module = None;
    let mut crate_root = None;
    let mut out_dir = None;
    let mut crate_name = None;
    let mut target_kind = None;
    let mut target_name = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" | "--output" => output = args.next().map(PathBuf::from),
            "--module" => module = args.next(),
            "--crate-root" => crate_root = args.next().map(PathBuf::from),
            "--out-dir" => out_dir = args.next().map(PathBuf::from),
            "--crate-name" => crate_name = args.next(),
            "--target-kind" => target_kind = args.next(),
            "--target-name" => target_name = args.next(),
            "-h" | "--help" => {
                println!(
                    "usage:\n  rust-subset-syn-adapter <input.rs> [-o output.json] [--module name]\n  rust-subset-syn-adapter --crate-root DIR --out-dir DIR [--crate-name name] [--target-kind lib|bin] [--target-name name]"
                );
                std::process::exit(0);
            }
            _ if input.is_none() => input = Some(PathBuf::from(arg)),
            _ => fail(format!("unexpected argument: {arg}")),
        }
    }

    if crate_root.is_some() || out_dir.is_some() {
        let crate_root = crate_root.unwrap_or_else(|| fail("missing --crate-root for crate mode"));
        let out_dir = out_dir.unwrap_or_else(|| fail("missing --out-dir for crate mode"));
        if input.is_some() || output.is_some() || module.is_some() {
            fail("crate mode must not use input file, -o, or --module");
        }
        let crate_name = crate_name.unwrap_or_else(|| {
            crate_root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("crate")
                .to_string()
        });
        let target_kind = target_kind.unwrap_or_else(|| "lib".to_string());
        let target_name = target_name.unwrap_or_else(|| crate_name.clone());
        return Command::Crate {
            crate_root,
            out_dir,
            crate_name,
            target_kind,
            target_name,
        };
    }

    let input = input.unwrap_or_else(|| fail("missing input .rs path"));
    let module = module.unwrap_or_else(|| {
        input
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("module")
            .to_string()
    });

    Command::SingleFile {
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
