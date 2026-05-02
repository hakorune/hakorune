use super::populate_from_toml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

fn test_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "hakorune_using_resolver_{}_{}_{}",
        label,
        std::process::id(),
        nanos
    ))
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).expect("write file");
}

fn restore_env_and_cwd(original_root: Option<String>, original_dir: PathBuf) {
    if let Some(value) = original_root {
        std::env::set_var("NYASH_ROOT", value);
    } else {
        std::env::remove_var("NYASH_ROOT");
    }
    std::env::set_current_dir(original_dir).expect("restore cwd");
}

#[test]
fn populate_from_toml_merges_root_modules_into_local_manifest() {
    let _guard = test_guard().lock().expect("lock");
    let original_dir = std::env::current_dir().expect("cwd");
    let original_root = std::env::var("NYASH_ROOT").ok();

    let root_dir = unique_temp_dir("root");
    let local_dir = unique_temp_dir("local");
    std::fs::create_dir_all(&root_dir).expect("root dir");
    std::fs::create_dir_all(&local_dir).expect("local dir");

    write_file(
        &root_dir.join("hako.toml"),
        "[modules]\n\"selfhost.vm.entry_s0\" = \"lang/src/vm/boxes/mini_vm_s0_entry.hako\"\n",
    );
    write_file(
        &root_dir.join("lang/src/vm/boxes/mini_vm_s0_entry.hako"),
        "static box MiniVmS0EntryBox {}\n",
    );
    write_file(
        &local_dir.join("nyash.toml"),
        "[using]\npaths = [\"lib\"]\n",
    );
    std::fs::create_dir_all(local_dir.join("lib")).expect("lib dir");

    std::env::set_current_dir(&local_dir).expect("set cwd");
    std::env::set_var("NYASH_ROOT", &root_dir);

    let mut using_paths = Vec::new();
    let mut pending_modules = Vec::new();
    let mut aliases = HashMap::new();
    let mut packages = HashMap::new();
    let mut module_roots = Vec::new();
    let result = populate_from_toml(
        &mut using_paths,
        &mut pending_modules,
        &mut aliases,
        &mut packages,
        &mut module_roots,
    );

    restore_env_and_cwd(original_root, original_dir);

    assert!(
        result.is_ok(),
        "populate_from_toml should succeed: {result:?}"
    );
    let expected_module = root_dir
        .join("lang/src/vm/boxes/mini_vm_s0_entry.hako")
        .to_string_lossy()
        .to_string();
    assert!(pending_modules
        .iter()
        .any(|(name, path)| { name == "selfhost.vm.entry_s0" && path == &expected_module }));
    let expected_using_path = local_dir.join("lib").to_string_lossy().to_string();
    assert!(using_paths.iter().any(|path| path == &expected_using_path));

    let _ = std::fs::remove_dir_all(&root_dir);
    let _ = std::fs::remove_dir_all(&local_dir);
}

#[test]
fn populate_from_toml_prefers_local_module_override_over_root_manifest() {
    let _guard = test_guard().lock().expect("lock");
    let original_dir = std::env::current_dir().expect("cwd");
    let original_root = std::env::var("NYASH_ROOT").ok();

    let root_dir = unique_temp_dir("override_root");
    let local_dir = unique_temp_dir("override_local");
    std::fs::create_dir_all(&root_dir).expect("root dir");
    std::fs::create_dir_all(&local_dir).expect("local dir");

    write_file(
        &root_dir.join("hako.toml"),
        "[modules]\nfoo.bar = \"root/foo/bar.hako\"\n",
    );
    write_file(
        &root_dir.join("root/foo/bar.hako"),
        "static box RootBar {}\n",
    );
    write_file(
        &local_dir.join("nyash.toml"),
        "[modules]\nfoo.bar = \"local/foo/bar.hako\"\n",
    );
    write_file(
        &local_dir.join("local/foo/bar.hako"),
        "static box LocalBar {}\n",
    );

    std::env::set_current_dir(&local_dir).expect("set cwd");
    std::env::set_var("NYASH_ROOT", &root_dir);

    let mut using_paths = Vec::new();
    let mut pending_modules = Vec::new();
    let mut aliases = HashMap::new();
    let mut packages = HashMap::new();
    let mut module_roots = Vec::new();
    let result = populate_from_toml(
        &mut using_paths,
        &mut pending_modules,
        &mut aliases,
        &mut packages,
        &mut module_roots,
    );

    restore_env_and_cwd(original_root, original_dir);

    assert!(
        result.is_ok(),
        "populate_from_toml should succeed: {result:?}"
    );
    let expected_local = local_dir
        .join("local/foo/bar.hako")
        .to_string_lossy()
        .to_string();
    assert!(pending_modules
        .iter()
        .any(|(name, path)| name == "foo.bar" && path == &expected_local));

    let _ = std::fs::remove_dir_all(&root_dir);
    let _ = std::fs::remove_dir_all(&local_dir);
}
