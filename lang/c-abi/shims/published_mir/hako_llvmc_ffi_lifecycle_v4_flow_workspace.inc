/* One function owns all indexed flow storage. No semantic ledger or cache. */
typedef struct {
  hako_lv4_state *states, *scratch;
  signed char* faults;
  size_t *queue, head, tail, count, entry;
} hako_lv4_worklist;

static void lv4_work_close(hako_lv4_worklist* work) {
  free(work->states); free(work->scratch); free(work->faults); free(work->queue);
  memset(work, 0, sizeof(*work));
}

static int lv4_work_open(hako_lv4_worklist* work, size_t count, size_t entry) {
  if (!count || entry >= count || count > SIZE_MAX / count || count > SIZE_MAX / 3) return 0;
  work->states = lv4_checked_calloc(count * count, sizeof(*work->states));
  work->scratch = lv4_checked_calloc(count * 3, sizeof(*work->scratch));
  work->faults = lv4_checked_calloc(count, sizeof(*work->faults));
  work->queue = lv4_checked_calloc(count, sizeof(*work->queue));
  if (!work->states || !work->scratch || !work->faults || !work->queue) {
    lv4_work_close(work); return 0;
  }
  work->count = count; work->entry = entry;
  memset(work->faults, -1, count * sizeof(*work->faults));
  work->faults[entry] = 0;
  work->queue[work->tail++] = entry;
  return 1;
}
