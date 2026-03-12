/*!
 * Stage-1 bridge child env - runtime defaults section.
 *
 * Keeps bridge-local runtime defaults and mainline MIR builder locks out of the
 * top-level child-env facade.
 */

use std::process::Command;

pub(super) fn apply(cmd: &mut Command) {
    if std::env::var("NYASH_NYRT_SILENT_RESULT").is_err() {
        cmd.env("NYASH_NYRT_SILENT_RESULT", "1");
    }
    if std::env::var("NYASH_DISABLE_PLUGINS").is_err() {
        cmd.env("NYASH_DISABLE_PLUGINS", "0");
    }
    if std::env::var("NYASH_FILEBOX_MODE").is_err() {
        cmd.env("NYASH_FILEBOX_MODE", "auto");
    }
    if std::env::var("NYASH_BOX_FACTORY_POLICY").is_err() {
        cmd.env("NYASH_BOX_FACTORY_POLICY", "builtin_first");
    }
    if std::env::var("HAKO_MIR_BUILDER_METHODIZE").is_err() {
        cmd.env("HAKO_MIR_BUILDER_METHODIZE", "0");
    }

    // Mainline lock: keep MirBuilder on internal-only route.
    // Delegate route (env.mirbuilder.emit) is treated as compatibility-only.
    cmd.env("HAKO_SELFHOST_NO_DELEGATE", "1");
    cmd.env("HAKO_MIR_BUILDER_DELEGATE", "0");
}
