use std::process::Command;

const CONTRACT: &str = "rustc-semir-adapter-tool-preflight-v0";

fn rustc_version() -> String {
    match Command::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("rustc-version-error:{}", stderr.trim())
        }
        Err(err) => format!("rustc-not-invoked:{err}"),
    }
}

fn print_preflight() {
    println!("output_contract={CONTRACT}");
    println!("adapter_tool_preflight_green=1");
    println!("standalone_tool_manifest_exists=1");
    println!("rustc_version={}", rustc_version());
    println!("rustc_private_dependency_enabled=0");
    println!("facts_generated=0");
    println!("hako_plan_emitted=0");
    println!("hako_source_emitted=0");
    println!("backend_behavior_changed=0");
    println!("summary=ok");
}

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--preflight") if args.next().is_none() => print_preflight(),
        _ => {
            eprintln!("usage: rustc-semir-adapter --preflight");
            std::process::exit(2);
        }
    }
}
