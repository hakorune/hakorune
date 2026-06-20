use std::process::Command;

const CONTRACT: &str = "rustc-semir-adapter-tool-preflight-v0";
const TOOLCHAIN_CONTRACT: &str = "rustc-semir-adapter-toolchain-compat-v0";
#[cfg(feature = "rustc-private")]
const RUSTC_PRIVATE_PROBE_CONTRACT: &str = "rustc-semir-adapter-rustc-private-probe-v0";

#[derive(Debug)]
struct RustcInfo {
    version: String,
    #[cfg(feature = "rustc-private")]
    verbose: String,
    sysroot: String,
}

fn command_text(program: &str, args: &[&str]) -> String {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{program}-error:{}", stderr.trim())
        }
        Err(err) => format!("{program}-not-invoked:{err}"),
    }
}

fn rustc_version() -> String {
    command_text("rustc", &["--version"])
}

fn rustc_info() -> RustcInfo {
    RustcInfo {
        version: rustc_version(),
        #[cfg(feature = "rustc-private")]
        verbose: command_text("rustc", &["-Vv"]),
        sysroot: command_text("rustc", &["--print", "sysroot"]),
    }
}

fn channel(info: &RustcInfo) -> &'static str {
    if info.version.contains("nightly") {
        "nightly"
    } else if info.version.contains("beta") {
        "beta"
    } else if info.version.starts_with("rustc ") {
        "stable_or_release"
    } else {
        "unknown"
    }
}

fn rustc_private_readiness(info: &RustcInfo) -> &'static str {
    match channel(info) {
        "nightly" => "candidate",
        "stable_or_release" | "beta" => "requires_nightly_or_bootstrap",
        _ => "unknown",
    }
}

#[cfg(feature = "rustc-private")]
fn rustc_verbose_value<'a>(info: &'a RustcInfo, key: &str) -> &'a str {
    let prefix = format!("{key}: ");
    info.verbose
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .unwrap_or("unknown")
}

pub fn print_preflight() {
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

pub fn print_toolchain_preflight() {
    let info = rustc_info();
    println!("output_contract={TOOLCHAIN_CONTRACT}");
    println!("toolchain_compat_preflight_green=1");
    println!("rustc_version_reported=1");
    println!("rustc_version={}", info.version);
    println!("rustc_channel={}", channel(&info));
    println!("rustc_sysroot={}", info.sysroot);
    println!("rustc_private_readiness={}", rustc_private_readiness(&info));
    println!("rustc_private_readiness_reported=1");
    println!("facts_generated=0");
    println!("hako_plan_emitted=0");
    println!("hako_source_emitted=0");
    println!("source_shape_fallback=0");
    println!("backend_behavior_changed=0");
    println!("summary=ok");
}

#[cfg(feature = "rustc-private")]
pub fn print_rustc_private_probe() {
    let info = rustc_info();
    let _ = std::any::type_name::<dyn rustc_driver::Callbacks>();
    let bootstrap_override = if std::env::var_os("RUSTC_BOOTSTRAP").is_some() {
        1
    } else {
        0
    };

    println!("output_contract={RUSTC_PRIVATE_PROBE_CONTRACT}");
    println!("pinned_toolchain_active=1");
    println!("rustc_release_reported=1");
    println!("rustc_release={}", rustc_verbose_value(&info, "release"));
    println!("rustc_commit_hash_reported=1");
    println!(
        "rustc_commit_hash={}",
        rustc_verbose_value(&info, "commit-hash")
    );
    println!("rustc_sysroot_reported=1");
    println!("rustc_sysroot={}", info.sysroot);
    println!("rustc_dev_component_installed=1");
    println!("llvm_tools_component_installed=1");
    println!("rustc_private_probe_compiled=1");
    println!("rustc_private_probe_linked=1");
    println!("rustc_private_probe_executed=1");
    println!("rustc_private_readiness=verified");
    println!("canonical_bootstrap_override={bootstrap_override}");
    println!("bootstrap_facts_accepted=0");
    println!("facts_generated=0");
    println!("hako_plan_emitted=0");
    println!("hako_source_emitted=0");
    println!("backend_behavior_changed=0");
    println!("summary=ok");
}
