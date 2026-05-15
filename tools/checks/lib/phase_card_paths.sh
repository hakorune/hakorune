#!/usr/bin/env bash
# phase_card_paths.sh - helpers for phase card path lookup during archive moves.

phase293x_card_bucket() {
  local filename="$1"
  local number="${filename#293x-}"
  number="${number%%-*}"

  case "$number" in
    0[0-9][0-9])
      echo "293x-000-099"
      ;;
    1[0-9][0-9])
      echo "293x-100-199"
      ;;
    2[0-9][0-9])
      echo "293x-200-299"
      ;;
    3[0-9][0-9])
      echo "293x-300-399"
      ;;
    4[0-9][0-9])
      echo "293x-400-499"
      ;;
    *)
      return 1
      ;;
  esac
}

phase293x_card_path() {
  local filename="$1"
  local phase_dir="docs/development/current/main/phases/phase-293x"
  local live_path="$phase_dir/$filename"

  if [[ -f "$live_path" ]]; then
    echo "$live_path"
    return 0
  fi

  local bucket
  if ! bucket="$(phase293x_card_bucket "$filename")"; then
    return 1
  fi

  local archive_path="$phase_dir/archive/cards/$bucket/$filename"
  if [[ -f "$archive_path" ]]; then
    echo "$archive_path"
    return 0
  fi

  return 1
}

guard_require_phase293x_card() {
  local tag="$1"
  local filename="$2"
  local path

  if ! path="$(phase293x_card_path "$filename")"; then
    guard_fail "$tag" "phase-293x card not found in live root or archive: $filename"
  fi

  echo "$path"
}
