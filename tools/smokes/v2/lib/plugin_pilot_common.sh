#!/bin/bash
# Common helpers for phase29cc plugin pilot smokes.

detect_lib_ext() {
  case "$(uname -s)" in
    Darwin) echo "dylib" ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT) echo "dll" ;;
    *) echo "so" ;;
  esac
}

lib_name_for() {
  local base="$1"
  local ext="$2"
  if [ "$ext" = "dll" ]; then
    echo "${base}.dll"
  else
    echo "lib${base}.${ext}"
  fi
}

require_fixture_file() {
  local smoke_name="$1"
  local fixture_path="$2"
  if [ ! -f "$fixture_path" ]; then
    test_fail "$smoke_name: fixture missing ($fixture_path)"
    return 1
  fi
  return 0
}

build_plugin_release_checked() {
  local smoke_name="$1"
  local label="$2"
  local artifact_path="$3"
  shift 3
  log_info "$smoke_name: building ${label} plugin release artifact"
  (cd "$NYASH_ROOT" && cargo build "$@" --release >/dev/null)
  if [ ! -f "$artifact_path" ]; then
    test_fail "$smoke_name: ${label} plugin artifact missing ($artifact_path)"
    return 1
  fi
  return 0
}

build_string_plugin_release_checked() {
  local smoke_name="$1"
  local artifact_path="$2"
  log_info "$smoke_name: building string plugin release artifact"
  (cd "$NYASH_ROOT/plugins/nyash-string-plugin" && cargo build --release >/dev/null)
  if [ ! -f "$artifact_path" ]; then
    test_fail "$smoke_name: string plugin artifact missing ($artifact_path)"
    return 1
  fi
  return 0
}

append_stringbox_toml() {
  local lib_name="$1"
  local lib_path="$2"
  cat << EOF
[libraries."$lib_name"]
boxes = ["StringBox"]
path = "$lib_path"

[libraries."$lib_name".StringBox]
type_id = 10
abi_version = 1
singleton = false

[libraries."$lib_name".StringBox.methods]
birth = { method_id = 0 }
length = { method_id = 1 }
len = { method_id = 1 }
isEmpty = { method_id = 2 }
charCodeAt = { method_id = 3 }
concat = { method_id = 4 }
toUtf8 = { method_id = 6 }
toString = { method_id = 6 }
fini = { method_id = 4294967295 }
EOF
}
