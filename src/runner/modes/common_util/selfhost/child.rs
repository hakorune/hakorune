pub const ROUTE_RUNTIME_SELFHOST: &str = "SH-RUNTIME-SELFHOST";
pub const ROUTE_MODE_PIPELINE_ENTRY: &str = "pipeline-entry";
pub const ROUTE_MODE_STAGE_A: &str = "stage-a";
pub const ROUTE_MODE_EXE: &str = "exe";

pub fn format_route_tag(route_id: &str, mode: &str, source: &str) -> String {
    format!(
        "[selfhost/route] id={} mode={} source={}",
        route_id, mode, source
    )
}

pub fn emit_route_tag(route_id: &str, mode: &str, source: &str) {
    eprintln!("{}", format_route_tag(route_id, mode, source));
}

pub fn emit_runtime_route_mode(mode: &str, source: &str) {
    emit_route_tag(ROUTE_RUNTIME_SELFHOST, mode, source);
}

#[cfg(test)]
mod tests {
    #[test]
    fn route_tag_format_stable_pipeline_entry() {
        let line = super::format_route_tag(
            super::ROUTE_RUNTIME_SELFHOST,
            super::ROUTE_MODE_PIPELINE_ENTRY,
            "foo.hako",
        );
        assert_eq!(
            line,
            "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=pipeline-entry source=foo.hako"
        );
    }

    #[test]
    fn route_tag_format_stable_stage_a() {
        let line = super::format_route_tag(
            super::ROUTE_RUNTIME_SELFHOST,
            super::ROUTE_MODE_STAGE_A,
            "foo.hako",
        );
        assert_eq!(
            line,
            "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=stage-a source=foo.hako"
        );
    }

    #[test]
    fn route_tag_format_stable_exe() {
        let line = super::format_route_tag(
            super::ROUTE_RUNTIME_SELFHOST,
            super::ROUTE_MODE_EXE,
            "foo.hako",
        );
        assert_eq!(
            line,
            "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=exe source=foo.hako"
        );
    }
}
