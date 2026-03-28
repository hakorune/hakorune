pub(super) fn extract_ll(stdout: &str) -> Result<String, String> {
    let begin = "[hako-ll/ll-begin]\n";
    let end = "\n[hako-ll/ll-end]";
    let start = stdout.find(begin).ok_or_else(|| {
        format!(
            "[llvmemit/hako-ll/ll-begin-missing] stdout=`{}`",
            stdout.trim()
        )
    })?;
    let content_start = start + begin.len();
    let tail = &stdout[content_start..];
    let end_offset = tail.find(end).ok_or_else(|| {
        format!(
            "[llvmemit/hako-ll/ll-end-missing] stdout=`{}`",
            stdout.trim()
        )
    })?;
    Ok(tail[..end_offset].to_string())
}

pub(super) fn extract_contract_line(stdout: &str, lane_tag: &str) -> Result<String, String> {
    let prefix = format!("[hako-ll/{}] ", lane_tag);
    stdout
        .lines()
        .find(|line| line.starts_with(&prefix))
        .map(|line| line.to_string())
        .ok_or_else(|| {
            format!(
                "[llvmemit/hako-ll/contract-line-missing] lane={} stdout=`{}`",
                lane_tag,
                stdout.trim()
            )
        })
}
