#!/bin/bash
# Archived one-case promoter for parser handoff Tier-2.
# Historical engineering helper only; keep it frozen and non-growing.
#
# Updates in one command:
#  1) selfhost subset TSV
#  2) phase-29bq Tier-2 backlog checklist
#  3) CURRENT_TASK session log + next pointer（legacy compatibility block がある場合のみ）
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../.." && pwd)"
SUBSET_TSV="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv"
BACKLOG_MD="$ROOT_DIR/docs/development/current/main/phases/phase-29bq/29bq-93-parser-handoff-tier2-backlog.md"
CURRENT_TASK_MD="$ROOT_DIR/CURRENT_TASK.md"

fixture=""
expected=""
backlog_id=""
next_task=""
allow_rc="0"
planner_tag="[joinir/planner_first rule=LoopCondBreak]"
reason=""
summary_token=""
session_slug=""
case_expr=""
today="$(date +%Y-%m-%d)"
dry_run=0
update_current_task=0

usage() {
  cat <<'USAGE' >&2
Usage:
  tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh \
    --fixture <apps/tests/...hako> \
    --expected <string> \
    --backlog-id <T2-...> \
    --next-task "<next expr>"

Optional:
  --allow-rc <int>                 (default: 0)
  --planner-tag <tag>              (default: [joinir/planner_first rule=LoopCondBreak])
  --reason <text>                  (default: promote: <fixture_basename>)
  --summary-token <token>          (default: derived from fixture)
  --session-slug <slug>            (default: derived from fixture)
  --case-expr "<expr text>"        (default: read from backlog by --backlog-id)
  --date YYYY-MM-DD                (default: today)
  --dry-run                        (check-only; no file updates)

Example:
  tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh \
    --fixture apps/tests/phase29bq_selfhost_local_expr_compare_rel_mixed_logic_cleanup_min.hako \
    --expected 2477 \
    --backlog-id T2-CMP-REL-MIX \
    --next-task "! + unary - + 比較 + &&"
USAGE
}

fail() {
  echo "[promote/tier2] $*" >&2
  exit 2
}

while [ $# -gt 0 ]; do
  case "$1" in
    --fixture)
      [ $# -ge 2 ] || fail "--fixture requires value"
      fixture="$2"
      shift 2
      ;;
    --expected)
      [ $# -ge 2 ] || fail "--expected requires value"
      expected="$2"
      shift 2
      ;;
    --backlog-id)
      [ $# -ge 2 ] || fail "--backlog-id requires value"
      backlog_id="$2"
      shift 2
      ;;
    --next-task)
      [ $# -ge 2 ] || fail "--next-task requires value"
      next_task="$2"
      shift 2
      ;;
    --allow-rc)
      [ $# -ge 2 ] || fail "--allow-rc requires value"
      allow_rc="$2"
      shift 2
      ;;
    --planner-tag)
      [ $# -ge 2 ] || fail "--planner-tag requires value"
      planner_tag="$2"
      shift 2
      ;;
    --reason)
      [ $# -ge 2 ] || fail "--reason requires value"
      reason="$2"
      shift 2
      ;;
    --summary-token)
      [ $# -ge 2 ] || fail "--summary-token requires value"
      summary_token="$2"
      shift 2
      ;;
    --session-slug)
      [ $# -ge 2 ] || fail "--session-slug requires value"
      session_slug="$2"
      shift 2
      ;;
    --case-expr)
      [ $# -ge 2 ] || fail "--case-expr requires value"
      case_expr="$2"
      shift 2
      ;;
    --date)
      [ $# -ge 2 ] || fail "--date requires value"
      today="$2"
      shift 2
      ;;
    --dry-run)
      dry_run=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown arg: $1"
      ;;
  esac
done

[ -n "$fixture" ] || fail "--fixture is required"
[ -n "$expected" ] || fail "--expected is required"
[ -n "$backlog_id" ] || fail "--backlog-id is required"
[ -n "$next_task" ] || fail "--next-task is required"
[[ "$allow_rc" =~ ^-?[0-9]+$ ]] || fail "--allow-rc must be integer"

if [[ "$fixture" = /* ]]; then
  case "$fixture" in
    "$ROOT_DIR"/*) fixture="${fixture#$ROOT_DIR/}" ;;
    *) fail "--fixture absolute path must be under repo root" ;;
  esac
fi

fixture_path="$ROOT_DIR/$fixture"
[ -f "$fixture_path" ] || fail "fixture not found: $fixture"

fixture_base="$(basename "$fixture" .hako)"
if [ -z "$reason" ]; then
  reason="promote: $fixture_base"
fi

if [ -z "$session_slug" ]; then
  session_slug="$fixture_base"
  session_slug="${session_slug#phase29bq_selfhost_}"
  session_slug="${session_slug%_cleanup_min}"
  session_slug="${session_slug//_/-}"
fi

if [ -z "$summary_token" ]; then
  summary_token="$session_slug"
fi

if [ -z "$case_expr" ]; then
  backlog_line="$(rg --fixed-strings "\`$backlog_id\`" "$BACKLOG_MD" | head -n 1 || true)"
  [ -n "$backlog_line" ] || fail "backlog id not found in $BACKLOG_MD: $backlog_id"
  case_expr="$(printf '%s\n' "$backlog_line" | sed -E 's/^- \[[ x]\] `[^`]+`: *//')"
  case_expr="${case_expr//\`/}"
  case_expr="$(printf '%s' "$case_expr" | sed -E 's/[[:space:]]+$//')"
  [ -n "$case_expr" ] || fail "failed to derive case expression from backlog for $backlog_id"
  if [[ "$case_expr" == *"\`$backlog_id\`"* ]] || [[ "$case_expr" == "$backlog_line" ]]; then
    fail "failed to parse case expression from backlog line: $backlog_line"
  fi
fi

if rg -q --fixed-strings "$fixture\t" "$SUBSET_TSV"; then
  fail "fixture already exists in subset TSV: $fixture"
fi

backlog_line="$(rg --fixed-strings "\`$backlog_id\`" "$BACKLOG_MD" | head -n 1 || true)"
[ -n "$backlog_line" ] || fail "backlog id not found: $backlog_id"
if [[ "$backlog_line" == "- [x]"* ]]; then
  fail "backlog id already checked: $backlog_id"
fi
if [[ "$backlog_line" != "- [ ]"* ]]; then
  fail "backlog line is not in pending checkbox format: $backlog_line"
fi

if rg -q '^- \[ \] Next single task:$' "$CURRENT_TASK_MD" \
  && rg -q '^- \[ \] next: Tier-2（式網羅）継続として .* 初期化の1件を PROBE→FIX→PROMOTE$' "$CURRENT_TASK_MD"; then
  update_current_task=1
fi

session_header="### Session Update ($today, parser handoff Tier-2 $session_slug + cleanup promote)"
if [ "$update_current_task" -eq 1 ]; then
  if rg -q --fixed-strings "$fixture" "$CURRENT_TASK_MD"; then
    fail "CURRENT_TASK already references fixture: $fixture"
  fi
  if rg -q --fixed-strings "$session_header" "$CURRENT_TASK_MD"; then
    fail "session header already exists in CURRENT_TASK: $session_header"
  fi
fi

if [ "$dry_run" -eq 1 ]; then
  cat <<EOF2
[promote/tier2] dry-run OK
  fixture      : $fixture
  expected     : $expected
  backlog_id   : $backlog_id
  case_expr    : $case_expr
  next_task    : $next_task
  session_slug : $session_slug
  summary_token: $summary_token
  reason       : $reason
EOF2
  exit 0
fi

printf '%s\t%s\t%s\t%s\t%s\n' "$fixture" "$expected" "$allow_rc" "$planner_tag" "$reason" >> "$SUBSET_TSV"

BACKLOG_ID="$backlog_id" SUMMARY_TOKEN="$summary_token" BACKLOG_MD="$BACKLOG_MD" perl - <<'PERL'
use strict;
use warnings;
use utf8;
use Encode qw(decode);

my $path = $ENV{BACKLOG_MD};
my $backlog_id = decode('UTF-8', $ENV{BACKLOG_ID} // '');
my $summary_token = decode('UTF-8', $ENV{SUMMARY_TOKEN} // '');

open my $in, '<:encoding(UTF-8)', $path or die "open $path: $!";
my @lines = <$in>;
close $in;

my $id_found = 0;
my $id_changed = 0;
my $done_updated = 0;
my $remaining_updated = 0;
my $summary_present = 0;
my $summary_patched = 0;

for my $line (@lines) {
    if ($line =~ /\Q$summary_token\E/) {
        $summary_present = 1;
    }
}

for my $line (@lines) {
    if ($line =~ /^- \[( |x)\] `\Q$backlog_id\E`:/) {
        $id_found = 1;
        if ($line =~ /^- \[ \]/) {
            $line =~ s/^- \[ \]/- [x]/;
            $id_changed = 1;
        }
    }

    if ($line =~ /^(Done（Tier-2 local-expr fixtures）: )(\d+)(件)/) {
        my $value = $2 + 1;
        $line = "$1$value$3\n";
        $done_updated = 1;
    }

    if ($line =~ /^(Total remaining candidate pack: )(\d+)(件)/) {
        my $value = $2 - 1;
        die "remaining candidate pack would become negative" if $value < 0;
        $line = "$1$value$3\n";
        $remaining_updated = 1;
    }

    if (!$summary_present && !$summary_patched && $line =~ /まで完了。/) {
        $line =~ s/ まで完了。/, $summary_token まで完了。/;
        $summary_patched = 1;
    }
}

die "backlog id not found: $backlog_id" unless $id_found;
die "backlog id was not pending: $backlog_id" unless $id_changed;
die "done-count header not found" unless $done_updated;
die "remaining-count header not found" unless $remaining_updated;
die "summary line patch failed" if !$summary_present && !$summary_patched;

open my $out, '>:encoding(UTF-8)', $path or die "write $path: $!";
print {$out} @lines;
close $out;
PERL

if [ "$update_current_task" -eq 1 ]; then
  NEXT_TASK="$next_task" CASE_EXPR="$case_expr" FIXTURE="$fixture" EXPECTED="$expected" FIXTURE_BASE="$fixture_base" TODAY="$today" SESSION_SLUG="$session_slug" CURRENT_TASK_MD="$CURRENT_TASK_MD" perl - <<'PERL'
use strict;
use warnings;
use utf8;
use Encode qw(decode);

my $path = $ENV{CURRENT_TASK_MD};
my $next_task = decode('UTF-8', $ENV{NEXT_TASK} // '');
my $case_expr = decode('UTF-8', $ENV{CASE_EXPR} // '');
my $fixture = decode('UTF-8', $ENV{FIXTURE} // '');
my $expected = decode('UTF-8', $ENV{EXPECTED} // '');
my $fixture_base = decode('UTF-8', $ENV{FIXTURE_BASE} // '');
my $today = decode('UTF-8', $ENV{TODAY} // '');
my $session_slug = decode('UTF-8', $ENV{SESSION_SLUG} // '');

open my $in, '<:encoding(UTF-8)', $path or die "open $path: $!";
local $/;
my $text = <$in>;
close $in;

my $header = "### Session Update ($today, parser handoff Tier-2 $session_slug + cleanup promote)";
die "session header already exists" if index($text, $header) >= 0;

my $next_re = qr{(- \[ \] Next single task:\n)\s*-\s*`[^`\n]+`\s*初期化を 1件 PROBE→FIX→PROMOTE};
$text =~ s/$next_re/$1  - `$next_task` 初期化を 1件 PROBE→FIX→PROMOTE/s
  or die "top next-task section not found";

my @pending;
while ($text =~ /^- \[ \] next: Tier-2（式網羅）継続として .* 初期化の1件を PROBE→FIX→PROMOTE$/mg) {
    push @pending, [ $-[0], $+[0] ];
}
die "pending Tier-2 next bullet not found" unless @pending;
my ($start, $end) = @{ $pending[-1] };
my $line = substr($text, $start, $end - $start);
$line =~ s/^- \[ \]/- [x]/;
substr($text, $start, $end - $start) = $line;

my $block = join("\n",
    "",
    $header,
    "",
    "- [x] parser handoff Tier-2（`local` 初期化で `$case_expr` を使用 + postfix `cleanup`）fixture を追加: `$fixture`（expected=`$expected`）",
    "- [x] PROBE: `SMOKES_SELFHOST_LIST=/tmp/selfhost_probe.tsv tools/selfhost/run.sh --gate --planner-required 1 --max-cases 1` PASS",
    "- [x] PROMOTE: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に `$fixture_base.hako` を追加",
    "- [x] verify: `tools/selfhost/run.sh --gate --planner-required 1 --filter $fixture_base --max-cases 1` PASS",
    "- [x] verify: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS",
    "- [ ] next: Tier-2（式網羅）継続として `$next_task` 初期化の1件を PROBE→FIX→PROMOTE",
    ""
);

$text =~ s/\n## Remaining Tasks \(ordered; SSOT\)/$block## Remaining Tasks (ordered; SSOT)/
  or die "Remaining Tasks section not found";

open my $out, '>:encoding(UTF-8)', $path or die "write $path: $!";
print {$out} $text;
close $out;
PERL
else
  echo "[promote/tier2] CURRENT_TASK legacy compatibility block not found; skipped CURRENT_TASK update" >&2
fi

echo "[promote/tier2] updated files:" >&2
echo "  - $SUBSET_TSV" >&2
echo "  - $BACKLOG_MD" >&2
if [ "$update_current_task" -eq 1 ]; then
  echo "  - $CURRENT_TASK_MD" >&2
fi
echo "[promote/tier2] promoted fixture: $fixture (expected=$expected, backlog=$backlog_id)" >&2
