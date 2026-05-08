#!/usr/bin/env bash

emit_case_kilo() {
  # kilo は C 参照側が重く、デフォルト N=5_000_000 だと実行が非常に長くなる。
  # Phase 21.5 最適化フェーズでは LLVM 系ベンチは EXE 経路のみを対象にする。
  # - LLVM backend かつ N が既定値（5_000_000）の場合は、常に N=200_000 に下げる。
  # - LLVM backend で EXE_MODE=0 の場合も、EXE 経路へ強制昇格する（VM フォールバック禁止）。
  if [[ "$BACKEND" = "llvm" && "$N" = "5000000" ]]; then
    N=200000
  fi
  if [[ "$BACKEND" = "llvm" && "$EXE_MODE" = "0" ]]; then
    echo "[info] kilo: forcing --exe for llvm backend (Phase 21.5 optimization)" >&2
    EXE_MODE=1
  fi
  HAKO_FILE=$(mktemp_hako)
  cat >"$HAKO_FILE" <<HAKO
box KiloBench {
  init { lines, undo }
  birth() {
    me.lines = new ArrayBox()
    me.undo = new ArrayBox()
    local i = 0
    loop(i < 64) {
      me.lines.push("line-" + i.toString())
      i = i + 1
    }
  }
  insert_chunk(row, text) {
    local line = me.lines.get(row)
    local len_box = line.length()
    local len = 0
    if len_box != null { len = len_box.toString().toInteger() }
    local split = len / 2
    local new_line = line.substring(0, split) + text + line.substring(split, len)
    me.lines.set(row, new_line)
    me.undo.push(text)
  }
  replace(pattern, replacement) {
    local count = me.lines.length().toString().toInteger()
    local i = 0
    loop(i < count) {
      local line = me.lines.get(i)
      if (line.indexOf(pattern) >= 0) {
        me.lines.set(i, line + replacement)
      }
      i = i + 1
    }
  }
  digest() {
    local total = 0
    local count = me.lines.length().toString().toInteger()
    local i = 0
    loop(i < count) {
      total = total + me.lines.get(i).length()
      i = i + 1
    }
    return total + me.undo.length().toString().toInteger()
  }
}
static box Main { method main(args) {
  local ops = ${N}
  local bench = new KiloBench()
  bench.birth()
  local i = 0
  loop(i < ops) {
    bench.insert_chunk(i % 64, "xx")
    if (i % 8 == 0) {
      bench.replace("line", "ln")
    }
    i = i + 1
  }
  return bench.digest()
} }
HAKO
  C_FILE=$(mktemp_c)
  cat >"$C_FILE" <<'C'
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
static void insert_chunk(char **lines, int row, const char *text){
  char *line = lines[row];
  size_t len = strlen(line);
  size_t split = len/2;
  char *out = malloc(len + strlen(text) + 1);
  memcpy(out, line, split);
  strcpy(out+split, text);
  strcpy(out+split+strlen(text), line+split);
  free(line);
  lines[row] = out;
}
static void replace_line(char **lines, const char *pattern, const char *repl){
  for (int i=0;i<64;i++){
    if (strstr(lines[i], pattern)){
      size_t len = strlen(lines[i]) + strlen(repl) + 1;
      char *out = malloc(len);
      strcpy(out, lines[i]);
      strcat(out, repl);
      free(lines[i]);
      lines[i] = out;
    }
  }
}
int main(){
  volatile int64_t ops = N_PLACEHOLDER;
  char *lines[64];
  for (int i=0;i<64;i++){
    char buf[32]; sprintf(buf, "line-%d", i);
    lines[i] = strdup(buf);
  }
  for (int64_t i=0;i<ops;i++){
    insert_chunk(lines, i % 64, "xx");
    if (i % 8 == 0) replace_line(lines, "line", "ln");
  }
  int64_t total = 0;
  for (int i=0;i<64;i++){ total += strlen(lines[i]); }
  for (int i=0;i<64;i++){ free(lines[i]); }
  return (int)(total & 0xFF);
}
C
  sed -i "s/N_PLACEHOLDER/${N}/" "$C_FILE"
}
