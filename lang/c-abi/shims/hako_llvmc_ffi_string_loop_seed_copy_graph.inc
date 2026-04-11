
static int hako_llvmc_copy_chain_reaches_reg(
    yyjson_val* insts,
    long long seed_reg,
    long long target_reg) {
  long long cursor = seed_reg;
  int advanced = 0;

  if (seed_reg == 0 || target_reg == 0 || !(insts && yyjson_is_arr(insts))) return 0;
  if (seed_reg == target_reg) return 1;

  do {
    size_t i;
    advanced = 0;
    for (i = 0; i < yyjson_arr_size(insts); i++) {
      yyjson_val* ins = yyjson_arr_get(insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      yyjson_val* src = ins ? yyjson_obj_get(ins, "src") : NULL;
      yyjson_val* dst = ins ? yyjson_obj_get(ins, "dst") : NULL;
      if (!(op && src && dst)) continue;
      if (strcmp(op, "copy") != 0) continue;
      if ((long long)yyjson_get_sint(src) != cursor) continue;
      cursor = (long long)yyjson_get_sint(dst);
      if (cursor == target_reg) return 1;
      advanced = 1;
      break;
    }
  } while (advanced);

  return 0;
}

static int hako_llvmc_copy_graph_reaches_reg(
    yyjson_val* insts,
    long long seed_reg,
    long long target_reg) {
  long long queue[256];
  long long visited[256];
  size_t queue_head = 0;
  size_t queue_tail = 0;
  size_t visited_n = 0;

  if (seed_reg == 0 || target_reg == 0 || !(insts && yyjson_is_arr(insts))) return 0;
  if (seed_reg == target_reg) return 1;

  queue[queue_tail++] = seed_reg;
  visited[visited_n++] = seed_reg;

  while (queue_head < queue_tail) {
    long long cursor = queue[queue_head++];
    size_t i;
    for (i = 0; i < yyjson_arr_size(insts); i++) {
      yyjson_val* ins = yyjson_arr_get(insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      yyjson_val* src = ins ? yyjson_obj_get(ins, "src") : NULL;
      yyjson_val* dst = ins ? yyjson_obj_get(ins, "dst") : NULL;
      long long next_reg;
      size_t j;
      int seen = 0;
      if (!(op && src && dst)) continue;
      if (strcmp(op, "copy") != 0) continue;
      if ((long long)yyjson_get_sint(src) != cursor) continue;
      next_reg = (long long)yyjson_get_sint(dst);
      if (next_reg == target_reg) return 1;
      for (j = 0; j < visited_n; j++) {
        if (visited[j] == next_reg) {
          seen = 1;
          break;
        }
      }
      if (seen) continue;
      if (visited_n < (sizeof(visited) / sizeof(visited[0]))) {
        visited[visited_n++] = next_reg;
      }
      if (queue_tail < (sizeof(queue) / sizeof(queue[0]))) {
        queue[queue_tail++] = next_reg;
      }
    }
  }

  return 0;
}
