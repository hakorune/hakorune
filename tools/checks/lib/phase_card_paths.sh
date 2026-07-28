#!/usr/bin/env bash
# phase_card_paths.sh - helpers for phase card path lookup during archive moves.

phase_card_path() {
  local phase="$1"
  local filename="$2"
  local -a candidates=()
  local candidate

  while IFS= read -r candidate; do
    candidates+=("$candidate")
  done < <(phase_card_candidates "$phase" "$filename")

  phase_card_select_candidates "${candidates[@]}"
}

phase_card_candidates() {
  local phase="$1"
  local filename="$2"
  local live_root="docs/development/current/main/phases/phase-$phase"
  local global_root="docs/development/archive/phases/phase-$phase"
  local top_transitional_root="docs/development/current/main/phases/archive/phase-$phase"

  echo "$live_root/$filename"
  echo "$global_root/$filename"
  echo "$global_root/cards/$filename"

  if [[ "$phase" == "293x" ]]; then
    local bucket
    if bucket="$(phase293x_card_bucket "$filename")"; then
      echo "$global_root/cards/$bucket/$filename"
    fi
  fi

  echo "$top_transitional_root/$filename"

  if [[ "$phase" == "293x" ]]; then
    local bucket
    if bucket="$(phase293x_card_bucket "$filename")"; then
      echo "$live_root/archive/cards/$bucket/$filename"
    fi
  fi

  echo "$live_root/archive/$filename"
}

phase_card_is_forwarding_stub() {
  local path="$1"

  grep -Eq '^#{1,6}[[:space:]]+Moved([[:space:]]|$)' "$path" \
    && grep -Eq '^Moved to:' "$path"
}

phase_card_select_candidates() {
  local -a full_paths=()
  local candidate

  for candidate in "$@"; do
    [[ -f "$candidate" ]] || continue
    if ! phase_card_is_forwarding_stub "$candidate"; then
      full_paths+=("$candidate")
    fi
  done

  case "${#full_paths[@]}" in
    0)
      return 1
      ;;
    1)
      echo "${full_paths[0]}"
      ;;
    *)
      echo "[phase-card-paths] authoritative phase-card collision:" >&2
      printf '  %s\n' "${full_paths[@]}" >&2
      return 2
      ;;
  esac
}

guard_require_phase_card() {
  local tag="$1"
  local phase="$2"
  local filename="$3"
  local path

  if path="$(phase_card_path "$phase" "$filename")"; then
    echo "$path"
    return 0
  fi

  local status=$?
  if (( status == 2 )); then
    guard_fail "$tag" "phase-$phase card has multiple authoritative copies: $filename"
  fi
  guard_fail "$tag" "phase-$phase card not found in live or archive roots: $filename"
}

phase293x_card_bucket() {
  local filename="$1"
  [[ "$filename" =~ ^293x-([0-9]+)- ]] || return 1

  local number=$((10#${BASH_REMATCH[1]}))
  local lower=$(((number / 100) * 100))
  local upper=$((lower + 99))
  printf '293x-%03d-%03d\n' "$lower" "$upper"
}

phase293x_card_path() {
  local filename="$1"
  phase_card_path "293x" "$filename"
}

guard_require_phase293x_card() {
  local tag="$1"
  local filename="$2"
  guard_require_phase_card "$tag" "293x" "$filename"
}
