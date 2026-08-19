#define _GNU_SOURCE
#include "../../../include/nyrt_text_formal_residence_v1.h"

#include <inttypes.h>
#include <errno.h>
#include <linux/perf_event.h>
#include <sched.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/resource.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

typedef struct HakoPromotionTestWireV1 { uint64_t slot; uint64_t generation; } HakoPromotionTestWireV1;
typedef struct MesoFrameV1 { NyrtTextFormalResidenceFrameV1 header; NyrtTextFormalResidenceRootRowV1 roots[2]; } MesoFrameV1;
typedef struct MesoInputV1 {
  char* subject;
  const char* needle;
  uint64_t bytes;
  uint64_t scalars;
  uint64_t histogram[4];
} MesoInputV1;

extern HakoPromotionTestWireV1 hako_promotion_test_issue_text_wire_v1(const char* text);
extern void hako_promotion_test_drop_wire_v1(HakoPromotionTestWireV1 wire);
extern int64_t hako_s6c_meso(const uint8_t*, uint64_t, const uint8_t*, uint64_t);
extern int64_t hako_s6c_c_meso(const uint8_t*, uint64_t, const uint8_t*, uint64_t);

static const uint64_t SIZES[] = {32, 256, 4096, 1048576};
static const char* FAMILIES[] = {"ascii", "width2", "width3", "width4", "mixed"};
static const char* POSITIONS[] = {"first", "middle", "last", "miss"};

typedef int64_t (*MesoArmFn)(const uint8_t*, uint64_t, const uint8_t*, uint64_t);

typedef struct CounterSpecV1 {
  const char* name;
  uint32_t type;
  uint64_t config;
} CounterSpecV1;

typedef struct CounterGroupV1 {
  int fds[4];
  uint64_t ids[4];
} CounterGroupV1;

typedef struct CounterValueV1 {
  uint64_t value;
  uint64_t id;
} CounterValueV1;

typedef struct CounterReadV1 {
  uint64_t nr;
  uint64_t time_enabled;
  uint64_t time_running;
  CounterValueV1 values[4];
} CounterReadV1;

typedef struct EpochResultV1 {
  const CounterSpecV1* specs;
  CounterGroupV1 group;
  CounterReadV1 read;
  uint64_t elapsed_ns;
  uint64_t voluntary_context_switches;
  uint64_t involuntary_context_switches;
  int affinity_cpu_before;
  int affinity_cpu_after;
  int affinity_count_before;
  int affinity_count_after;
  uint64_t sink;
} EpochResultV1;

#define CACHE_EVENT(cache, operation, result) \
  ((uint64_t)(cache) | ((uint64_t)(operation) << 8) | ((uint64_t)(result) << 16))

static const CounterSpecV1 PRIMARY_EVENTS[4] = {
  {"cycles:u", PERF_TYPE_HARDWARE, PERF_COUNT_HW_CPU_CYCLES},
  {"instructions:u", PERF_TYPE_HARDWARE, PERF_COUNT_HW_INSTRUCTIONS},
  {"branches:u", PERF_TYPE_HARDWARE, PERF_COUNT_HW_BRANCH_INSTRUCTIONS},
  {"branch-misses:u", PERF_TYPE_HARDWARE, PERF_COUNT_HW_BRANCH_MISSES},
};

static const CounterSpecV1 FRONTEND_EVENTS[4] = {
  {"cycles:u", PERF_TYPE_HARDWARE, PERF_COUNT_HW_CPU_CYCLES},
  {"stalled-cycles-frontend:u", PERF_TYPE_HARDWARE, PERF_COUNT_HW_STALLED_CYCLES_FRONTEND},
  {"L1-icache-load-misses:u", PERF_TYPE_HW_CACHE,
   CACHE_EVENT(PERF_COUNT_HW_CACHE_L1I, PERF_COUNT_HW_CACHE_OP_READ, PERF_COUNT_HW_CACHE_RESULT_MISS)},
  {"iTLB-load-misses:u", PERF_TYPE_HW_CACHE,
   CACHE_EVENT(PERF_COUNT_HW_CACHE_ITLB, PERF_COUNT_HW_CACHE_OP_READ, PERF_COUNT_HW_CACHE_RESULT_MISS)},
};

static uint64_t now_ns(void) {
  struct timespec value;
  if (clock_gettime(CLOCK_MONOTONIC_RAW, &value) != 0) abort();
  return (uint64_t)value.tv_sec * UINT64_C(1000000000) + (uint64_t)value.tv_nsec;
}

static int perf_open(struct perf_event_attr* attr, int group_fd) {
  return (int)syscall(SYS_perf_event_open, attr, 0, -1, group_fd, PERF_FLAG_FD_CLOEXEC);
}

static int open_counter(uint32_t type, uint64_t config, int group_fd, int pinned) {
  struct perf_event_attr attr;
  memset(&attr, 0, sizeof(attr));
  attr.type = type;
  attr.size = sizeof(attr);
  attr.config = config;
  attr.disabled = group_fd < 0;
  attr.pinned = pinned;
  attr.exclude_kernel = 1;
  attr.exclude_hv = 1;
  attr.read_format = PERF_FORMAT_GROUP | PERF_FORMAT_ID |
      PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;
  return perf_open(&attr, group_fd);
}

static int open_group(const CounterSpecV1 specs[4], CounterGroupV1* group) {
  for (int index = 0; index < 4; index++) group->fds[index] = -1;
  for (int index = 0; index < 4; index++) {
    group->fds[index] = open_counter(
        specs[index].type, specs[index].config, index ? group->fds[0] : -1, index == 0);
    if (group->fds[index] < 0 || ioctl(group->fds[index], PERF_EVENT_IOC_ID, &group->ids[index]) != 0) {
      fprintf(stderr, "NoSafeSlice: unsupported PMU event %s: %s\n", specs[index].name, strerror(errno));
      return 0;
    }
  }
  return 1;
}

static void close_group(CounterGroupV1* group) {
  for (int index = 0; index < 4; index++) {
    if (group->fds[index] >= 0) close(group->fds[index]);
    group->fds[index] = -1;
  }
}

static uint64_t run_hako_loop(const MesoFrameV1* frame, uint64_t iterations) {
  int64_t sum = 0;
  for (uint64_t index = 0; index < iterations; index++)
    sum += hako_s6c_meso(frame->roots[0].ptr, frame->roots[0].byte_len,
                         frame->roots[1].ptr, frame->roots[1].byte_len);
  return (uint64_t)sum + 1;
}

static uint64_t run_c_loop(const MesoFrameV1* frame, uint64_t iterations) {
  int64_t sum = 0;
  for (uint64_t index = 0; index < iterations; index++)
    sum += hako_s6c_c_meso(frame->roots[0].ptr, frame->roots[0].byte_len,
                           frame->roots[1].ptr, frame->roots[1].byte_len);
  return (uint64_t)sum + 1;
}

static int read_single_cpu_affinity(int* cpu, int* count) {
  cpu_set_t affinity;
  CPU_ZERO(&affinity);
  if (sched_getaffinity(0, sizeof(affinity), &affinity) != 0) return 0;
  *count = CPU_COUNT(&affinity);
  *cpu = sched_getcpu();
  return *count == 1 && *cpu >= 0 && CPU_ISSET(*cpu, &affinity);
}

static int measure_epoch(
    int hako, const MesoFrameV1* frame, uint64_t iterations,
    const CounterSpecV1 specs[4], EpochResultV1* out) {
  struct rusage before, after;
  uint64_t start, expected_size = sizeof(CounterReadV1);
  memset(out, 0, sizeof(*out));
  out->specs = specs;
  if (!open_group(specs, &out->group)) goto fail;
  if (getrusage(RUSAGE_SELF, &before) != 0 ||
      !read_single_cpu_affinity(&out->affinity_cpu_before, &out->affinity_count_before) ||
      ioctl(out->group.fds[0], PERF_EVENT_IOC_RESET, PERF_IOC_FLAG_GROUP))
    goto ioctl_fail;
  start = now_ns();
  if (ioctl(out->group.fds[0], PERF_EVENT_IOC_ENABLE, PERF_IOC_FLAG_GROUP)) goto ioctl_fail;
  out->sink = hako ? run_hako_loop(frame, iterations) : run_c_loop(frame, iterations);
  if (ioctl(out->group.fds[0], PERF_EVENT_IOC_DISABLE, PERF_IOC_FLAG_GROUP)) goto ioctl_fail;
  out->elapsed_ns = now_ns() - start;
  if (!read_single_cpu_affinity(&out->affinity_cpu_after, &out->affinity_count_after) ||
      getrusage(RUSAGE_SELF, &after) != 0) goto ioctl_fail;
  out->voluntary_context_switches = (uint64_t)(after.ru_nvcsw - before.ru_nvcsw);
  out->involuntary_context_switches = (uint64_t)(after.ru_nivcsw - before.ru_nivcsw);
  if (read(out->group.fds[0], &out->read, sizeof(out->read)) != (ssize_t)expected_size ||
      after.ru_nvcsw < before.ru_nvcsw || after.ru_nivcsw < before.ru_nivcsw) {
    fprintf(stderr, "NoSafeSlice: incomplete PMU read\n");
    goto fail;
  }
  if (out->read.nr != 4 || !out->read.time_enabled ||
      out->read.time_enabled != out->read.time_running) {
    fprintf(stderr, "NoSafeSlice: multiplex/time scaling detected\n");
    goto fail;
  }
  for (int index = 0; index < 4; index++) {
    if (out->read.values[index].id != out->group.ids[index]) {
      fprintf(stderr, "NoSafeSlice: event ID drift for %s\n", specs[index].name);
      goto fail;
    }
  }
  if (out->voluntary_context_switches || out->involuntary_context_switches ||
      out->affinity_cpu_before != out->affinity_cpu_after ||
      out->affinity_count_before != 1 || out->affinity_count_after != 1) {
    fprintf(stderr, "NoSafeSlice: context-switch/affinity drift detected\n");
    goto fail;
  }
  return 1;
ioctl_fail:
  fprintf(stderr, "NoSafeSlice: PMU ioctl failed: %s\n", strerror(errno));
fail:
  close_group(&out->group);
  return 0;
}

static uint64_t fnv1a(const uint8_t* bytes, uint64_t length, uint64_t hash) {
  for (uint64_t index = 0; index < length; index++) {
    hash ^= bytes[index];
    hash *= UINT64_C(1099511628211);
  }
  return hash;
}

static void print_epoch(const char* name, const EpochResultV1* epoch) {
  printf("\"%s\":{\"group_event_count\":4,\"time_enabled\":%" PRIu64
         ",\"time_running\":%" PRIu64 ",\"lost_samples\":0,"
         "\"voluntary_context_switches\":%" PRIu64
         ",\"involuntary_context_switches\":%" PRIu64
         ",\"affinity_cpu_before\":%d,\"affinity_cpu_after\":%d,"
         "\"affinity_count_before\":%d,\"affinity_count_after\":%d,"
         "\"elapsed_ns\":%" PRIu64 ",\"events\":[",
         name, epoch->read.time_enabled, epoch->read.time_running,
         epoch->voluntary_context_switches, epoch->involuntary_context_switches,
         epoch->affinity_cpu_before, epoch->affinity_cpu_after,
         epoch->affinity_count_before, epoch->affinity_count_after, epoch->elapsed_ns);
  for (int index = 0; index < 4; index++) {
    printf("%s{\"name\":\"%s\",\"expected_id\":%" PRIu64
           ",\"read_id\":%" PRIu64 ",\"count\":%" PRIu64 "}",
           index ? "," : "", epoch->specs[index].name, epoch->group.ids[index],
           epoch->read.values[index].id, epoch->read.values[index].value);
  }
  printf("]}");
}

static void append_scalar(MesoInputV1* out, uint64_t* offset, const uint8_t* bytes, uint64_t width) {
  memcpy(out->subject + *offset, bytes, (size_t)width);
  *offset += width;
  out->scalars++;
  out->histogram[width - 1]++;
}

static MesoInputV1 build_input(const char* family, uint64_t size, const char* position) {
  static const uint8_t W2[] = {0xc2, 0xa2}, W2_TARGET[] = {0xc2, 0xa3};
  static const uint8_t W3[] = {0xe3, 0x81, 0x82}, W3_TARGET[] = {0xe3, 0x81, 0x84};
  static const uint8_t W4[] = {0xf0, 0x9f, 0x98, 0x80}, W4_TARGET[] = {0xf0, 0x9f, 0x98, 0xba};
  MesoInputV1 out = {(char*)calloc((size_t)size + 1, 1), NULL, size, 0, {0, 0, 0, 0}};
  uint64_t offset = 0, target = UINT64_MAX;
  int miss = strcmp(position, "miss") == 0;
  if (!out.subject) abort();
  if (!strcmp(family, "ascii")) {
    memset(out.subject, 'a', (size_t)size);
    out.scalars = size;
    out.histogram[0] = size;
    out.needle = "b";
    target = !strcmp(position, "first") ? 0 : !strcmp(position, "middle") ? size / 2 : size - 1;
    if (!miss) out.subject[target] = 'b';
  } else if (!strcmp(family, "width2") || !strcmp(family, "width4")) {
    const uint8_t* fill = !strcmp(family, "width2") ? W2 : W4;
    const uint8_t* replacement = !strcmp(family, "width2") ? W2_TARGET : W4_TARGET;
    uint64_t width = !strcmp(family, "width2") ? 2 : 4, count = size / width;
    out.needle = !strcmp(family, "width2") ? "£" : "😺";
    target = !strcmp(position, "first") ? 0 : !strcmp(position, "middle") ? count / 2 : count - 1;
    for (uint64_t index = 0; index < count; index++) {
      append_scalar(&out, &offset, (!miss && index == target) ? replacement : fill, width);
    }
  } else if (!strcmp(family, "width3")) {
    uint64_t pad = size % 3, count = (size - pad) / 3;
    int pad_first = !strcmp(position, "last");
    out.needle = "い";
    target = !strcmp(position, "first") ? 0 : !strcmp(position, "middle") ? count / 2 : count - 1;
    if (pad_first) for (uint64_t i = 0; i < pad; i++) {
      const uint8_t a = 'a'; append_scalar(&out, &offset, &a, 1);
    }
    for (uint64_t index = 0; index < count; index++) {
      append_scalar(&out, &offset, (!miss && index == target) ? W3_TARGET : W3, 3);
    }
    if (!pad_first) for (uint64_t i = 0; i < pad; i++) {
      const uint8_t a = 'a'; append_scalar(&out, &offset, &a, 1);
    }
  } else {
    static const uint8_t A[] = {'a'};
    uint64_t cycles = size / 10, remainder = size % 10;
    out.needle = "b";
    target = !strcmp(position, "first") ? 0 : !strcmp(position, "middle") ? cycles / 2 : UINT64_MAX;
    for (uint64_t cycle = 0; cycle < cycles; cycle++) {
      uint8_t first = (!miss && cycle == target) ? 'b' : 'a';
      append_scalar(&out, &offset, &first, 1);
      append_scalar(&out, &offset, W2, 2);
      append_scalar(&out, &offset, W3, 3);
      append_scalar(&out, &offset, W4, 4);
    }
    for (uint64_t i = 0; i < remainder; i++) append_scalar(&out, &offset, A, 1);
    if (!miss && !strcmp(position, "last")) out.subject[size - 1] = 'b';
  }
  if (offset != size && strcmp(family, "ascii")) abort();
  return out;
}

static uint64_t measure(int hako, const MesoFrameV1* frame, uint64_t iterations, volatile uint64_t* sink) {
  uint64_t start = now_ns();
  int64_t sum = 0;
  for (uint64_t i = 0; i < iterations; i++) {
    sum += hako
        ? hako_s6c_meso(frame->roots[0].ptr, frame->roots[0].byte_len, frame->roots[1].ptr, frame->roots[1].byte_len)
        : hako_s6c_c_meso(frame->roots[0].ptr, frame->roots[0].byte_len, frame->roots[1].ptr, frame->roots[1].byte_len);
  }
  *sink += (uint64_t)sum + 1;
  return now_ns() - start;
}

static int run_case(const char* family, uint64_t size, const char* position) {
  MesoInputV1 input = build_input(family, size, position);
  MesoFrameV1 frame;
  HakoPromotionTestWireV1 subject = hako_promotion_test_issue_text_wire_v1(input.subject);
  HakoPromotionTestWireV1 needle = hako_promotion_test_issue_text_wire_v1(input.needle);
  NyrtTextFormalBorrowV1 pairs[2] = {{subject.slot, subject.generation}, {needle.slot, needle.generation}};
  uint64_t iterations = 1, hako_ns, c_ns;
  volatile uint64_t sink = 0;
  memset(&frame, 0, sizeof(frame));
  if (!subject.slot || !needle.slot || hako_text_formal_residence_enter_v1(
      pairs, 2, &frame.header, (uint32_t)sizeof(frame)) != NYRT_TEXT_RESIDENCE_VALID_V1) return 0;
  if (frame.roots[0].byte_len != size ||
      hako_s6c_meso(frame.roots[0].ptr, size, frame.roots[1].ptr, frame.roots[1].byte_len) !=
      hako_s6c_c_meso(frame.roots[0].ptr, size, frame.roots[1].ptr, frame.roots[1].byte_len)) return 0;
  for (;;) {
    hako_ns = measure(1, &frame, iterations, &sink);
    c_ns = measure(0, &frame, iterations, &sink);
    if (hako_ns >= 30000000 && c_ns >= 30000000) break;
    if (iterations > UINT64_MAX / 2) return 0;
    iterations *= 2;
  }
  for (unsigned warmup = 0; warmup < 10; warmup++) {
    (void)measure(warmup & 1, &frame, iterations, &sink);
    (void)measure(!(warmup & 1), &frame, iterations, &sink);
  }
  for (unsigned sample = 0; sample < 51; sample++) {
    if (!(sample & 1)) {
      hako_ns = measure(1, &frame, iterations, &sink);
      c_ns = measure(0, &frame, iterations, &sink);
    } else {
      c_ns = measure(0, &frame, iterations, &sink);
      hako_ns = measure(1, &frame, iterations, &sink);
    }
    printf("%s,%" PRIu64 ",%s,%u,%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64
           ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 "\n",
           family, size, position, sample, iterations, hako_ns, c_ns, sink, input.scalars,
           input.histogram[0], input.histogram[1], input.histogram[2], input.histogram[3]);
  }
  hako_text_formal_residence_finish_or_abort_v1(&frame.header);
  hako_promotion_test_drop_wire_v1(subject);
  hako_promotion_test_drop_wire_v1(needle);
  free(input.subject);
  return 1;
}

static int run_counter_arm(const char* arm, const char* case_name, uint64_t iterations) {
  MesoInputV1 input;
  MesoFrameV1 frame;
  HakoPromotionTestWireV1 subject, needle;
  NyrtTextFormalBorrowV1 pairs[2];
  EpochResultV1 primary, frontend;
  cpu_set_t affinity;
  int affinity_count, cpu, hako = !strcmp(arm, "hako");
  int64_t hako_result, c_result;
  uint64_t fingerprint = UINT64_C(14695981039346656037);
  if (strcmp(arm, "hako") && strcmp(arm, "c")) {
    fprintf(stderr, "wrong arm: expected hako or c\n");
    return 0;
  }
  if (strcmp(case_name, "mixed/4096/first")) {
    fprintf(stderr, "wrong case: expected mixed/4096/first\n");
    return 0;
  }
  if (!iterations) {
    fprintf(stderr, "iterations must be positive\n");
    return 0;
  }
  CPU_ZERO(&affinity);
  if (sched_getaffinity(0, sizeof(affinity), &affinity) != 0) return 0;
  affinity_count = CPU_COUNT(&affinity);
  cpu = sched_getcpu();
  if (affinity_count != 1 || cpu < 0 || !CPU_ISSET(cpu, &affinity)) {
    fprintf(stderr, "NoSafeSlice: process is not fixed to exactly one CPU\n");
    return 0;
  }
  input = build_input("mixed", 4096, "first");
  subject = hako_promotion_test_issue_text_wire_v1(input.subject);
  needle = hako_promotion_test_issue_text_wire_v1(input.needle);
  pairs[0] = (NyrtTextFormalBorrowV1){subject.slot, subject.generation};
  pairs[1] = (NyrtTextFormalBorrowV1){needle.slot, needle.generation};
  memset(&frame, 0, sizeof(frame));
  if (!subject.slot || !needle.slot || hako_text_formal_residence_enter_v1(
      pairs, 2, &frame.header, (uint32_t)sizeof(frame)) != NYRT_TEXT_RESIDENCE_VALID_V1) {
    fprintf(stderr, "NoSafeSlice: Residence acquisition failed\n");
    return 0;
  }
  hako_result = hako_s6c_meso(frame.roots[0].ptr, frame.roots[0].byte_len,
                              frame.roots[1].ptr, frame.roots[1].byte_len);
  c_result = hako_s6c_c_meso(frame.roots[0].ptr, frame.roots[0].byte_len,
                             frame.roots[1].ptr, frame.roots[1].byte_len);
  if (hako_result != c_result) {
    fprintf(stderr, "NoSafeSlice: result mismatch before counters\n");
    return 0;
  }
  fingerprint = fnv1a(frame.roots[0].ptr, frame.roots[0].byte_len, fingerprint);
  fingerprint = fnv1a(frame.roots[1].ptr, frame.roots[1].byte_len, fingerprint);
  if (!measure_epoch(hako, &frame, iterations, PRIMARY_EVENTS, &primary) ||
      !measure_epoch(hako, &frame, iterations, FRONTEND_EVENTS, &frontend)) return 0;
  hako_text_formal_residence_finish_or_abort_v1(&frame.header);
  hako_promotion_test_drop_wire_v1(subject);
  hako_promotion_test_drop_wire_v1(needle);
  free(input.subject);
  printf("{\"schema\":\"s6c-meso-separate-arm-sample-v1\",\"arm\":\"%s\","
         "\"case\":\"%s\",\"iterations\":%" PRIu64 ",\"cpu\":%d,"
         "\"affinity_count\":%d,\"input_fingerprint\":\"%016" PRIx64 "\","
         "\"subject_byte_len\":%" PRIu64 ",\"needle_byte_len\":%" PRIu64 ","
         "\"scalars\":%" PRIu64 ",\"width_histogram\":[%" PRIu64 ",%" PRIu64
         ",%" PRIu64 ",%" PRIu64 "],"
         "\"result\":%" PRId64 ",\"parity_result\":%" PRId64 ",\"sink\":%" PRIu64 ",",
         arm, case_name, iterations, cpu, affinity_count, fingerprint,
         frame.roots[0].byte_len, frame.roots[1].byte_len, input.scalars,
         input.histogram[0], input.histogram[1], input.histogram[2], input.histogram[3],
         hako_result, c_result,
         primary.sink ^ frontend.sink);
  print_epoch("primary", &primary);
  printf(",");
  print_epoch("frontend", &frontend);
  printf("}\n");
  close_group(&primary.group);
  close_group(&frontend.group);
  return 1;
}

int main(int argc, char** argv) {
  if (argc != 1) {
    const char *arm = NULL, *case_name = NULL, *iterations_text = NULL;
    char* end = NULL;
    uint64_t iterations;
    for (int index = 1; index < argc; index += 2) {
      if (index + 1 >= argc) return 2;
      if (!strcmp(argv[index], "--arm")) arm = argv[index + 1];
      else if (!strcmp(argv[index], "--case")) case_name = argv[index + 1];
      else if (!strcmp(argv[index], "--iterations")) iterations_text = argv[index + 1];
      else return 2;
    }
    if (!arm || !case_name || !iterations_text) return 2;
    if (iterations_text[0] == '-') return 2;
    errno = 0;
    iterations = strtoull(iterations_text, &end, 10);
    if (errno || !end || *end) return 2;
    return run_counter_arm(arm, case_name, iterations) ? 0 : 1;
  }
  puts("family,size,position,sample,iterations,hako_ns,c_ns,sink,scalars,width1,width2,width3,width4");
  for (size_t family = 0; family < 5; family++)
    for (size_t size = 0; size < 4; size++)
      for (size_t position = 0; position < 4; position++)
        if (!run_case(FAMILIES[family], SIZES[size], POSITIONS[position])) return 1;
  return 0;
}
