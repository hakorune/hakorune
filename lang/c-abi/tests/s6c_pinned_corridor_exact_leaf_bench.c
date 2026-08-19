#define _GNU_SOURCE
#include "../../../include/nyrt_text_formal_residence_v1.h"

#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct HakoPromotionTestWireV1 {
  uint64_t slot;
  uint64_t generation;
} HakoPromotionTestWireV1;

extern HakoPromotionTestWireV1 hako_promotion_test_issue_text_wire_v1(
    const char* text);
extern void hako_promotion_test_drop_wire_v1(HakoPromotionTestWireV1 wire);
extern uint8_t hako_s6c_exact_leaf(
    const uint8_t* subject,
    uint64_t offset,
    uint64_t width,
    const uint8_t* needle,
    uint64_t needle_len);
extern uint8_t hako_s6c_c_exact_leaf(
    const uint8_t* subject,
    uint64_t offset,
    uint64_t width,
    const uint8_t* needle,
    uint64_t needle_len);

typedef struct ExactLeafFrameV1 {
  NyrtTextFormalResidenceFrameV1 header;
  NyrtTextFormalResidenceRootRowV1 roots[2];
} ExactLeafFrameV1;

typedef struct ExactLeafCaseV1 {
  const char* name;
  const char* category;
  const char* subject;
  const char* needle;
  uint64_t width;
  uint8_t expected;
  uint8_t alias;
} ExactLeafCaseV1;

static const ExactLeafCaseV1 CASES[] = {
    {"w1-equal", "ascii", "A", "A", 1, 1, 0},
    {"w1-first-mismatch", "ascii", "A", "B", 1, 0, 0},
    {"w1-last-mismatch", "ascii", "A", "C", 1, 0, 0},
    {"w1-length-mismatch", "ascii", "A", "AA", 1, 0, 0},
    {"w1-alias", "ascii", "A", "A", 1, 1, 1},
    {"w2-equal", "mixed", "\xC2\xA2", "\xC2\xA2", 2, 1, 0},
    {"w2-first-mismatch", "mixed", "\xC2\xA2", "\xC3\xA2", 2, 0, 0},
    {"w2-last-mismatch", "mixed", "\xC2\xA2", "\xC2\xA3", 2, 0, 0},
    {"w2-length-mismatch", "mixed", "\xC2\xA2", "A", 2, 0, 0},
    {"w2-alias", "mixed", "\xC2\xA2", "\xC2\xA2", 2, 1, 1},
    {"w3-equal", "mixed", "\xE2\x82\xAC", "\xE2\x82\xAC", 3, 1, 0},
    {"w3-first-mismatch", "mixed", "\xE2\x82\xAC", "\xE3\x82\xAC", 3, 0, 0},
    {"w3-last-mismatch", "mixed", "\xE2\x82\xAC", "\xE2\x82\xAD", 3, 0, 0},
    {"w3-length-mismatch", "mixed", "\xE2\x82\xAC", "\xC2\xA2", 3, 0, 0},
    {"w3-alias", "mixed", "\xE2\x82\xAC", "\xE2\x82\xAC", 3, 1, 1},
    {"w4-equal", "mixed", "\xF0\x9F\x98\x80", "\xF0\x9F\x98\x80", 4, 1, 0},
    {"w4-first-mismatch", "mixed", "\xF0\x9F\x98\x80", "\xF1\x8F\x98\x80", 4, 0, 0},
    {"w4-last-mismatch", "mixed", "\xF0\x9F\x98\x80", "\xF0\x9F\x98\x81", 4, 0, 0},
    {"w4-length-mismatch", "mixed", "\xF0\x9F\x98\x80", "\xE2\x82\xAC", 4, 0, 0},
    {"w4-alias", "mixed", "\xF0\x9F\x98\x80", "\xF0\x9F\x98\x80", 4, 1, 1},
};

static uint64_t now_ns(void) {
  struct timespec value;
  if (clock_gettime(CLOCK_MONOTONIC_RAW, &value) != 0) abort();
  return (uint64_t)value.tv_sec * UINT64_C(1000000000) + (uint64_t)value.tv_nsec;
}

static uint64_t measure(
    int hako,
    const NyrtTextFormalResidenceRootRowV1* subject,
    const NyrtTextFormalResidenceRootRowV1* needle,
    uint64_t width,
    uint64_t iterations,
    volatile uint64_t* sink) {
  uint64_t start = now_ns();
  uint64_t sum = 0;
  for (uint64_t i = 0; i < iterations; i++) {
    sum += hako
        ? hako_s6c_exact_leaf(subject->ptr, 0, width, needle->ptr, needle->byte_len)
        : hako_s6c_c_exact_leaf(
              subject->ptr, 0, width, needle->ptr, needle->byte_len);
  }
  *sink += sum + 1;
  return now_ns() - start;
}

static int run_case(const ExactLeafCaseV1* item) {
  HakoPromotionTestWireV1 subject_wire =
      hako_promotion_test_issue_text_wire_v1(item->subject);
  HakoPromotionTestWireV1 needle_wire = item->alias
      ? subject_wire
      : hako_promotion_test_issue_text_wire_v1(item->needle);
  NyrtTextFormalBorrowV1 pairs[2] = {
      {subject_wire.slot, subject_wire.generation},
      {needle_wire.slot, needle_wire.generation},
  };
  ExactLeafFrameV1 frame;
  volatile uint64_t sink = 0;
  uint64_t iterations = 4096;
  uint64_t hako_ns;
  uint64_t c_ns;
  memset(&frame, 0, sizeof(frame));
  if (!subject_wire.slot || !needle_wire.slot ||
      hako_text_formal_residence_enter_v1(
          pairs, 2, &frame.header, (uint32_t)sizeof(frame)) !=
          NYRT_TEXT_RESIDENCE_VALID_V1) {
    return 0;
  }
  if (hako_s6c_exact_leaf(
          frame.roots[0].ptr, 0, item->width,
          frame.roots[1].ptr, frame.roots[1].byte_len) != item->expected ||
      hako_s6c_c_exact_leaf(
          frame.roots[0].ptr, 0, item->width,
          frame.roots[1].ptr, frame.roots[1].byte_len) != item->expected) {
    return 0;
  }
  for (;;) {
    hako_ns = measure(1, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
    c_ns = measure(0, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
    if (hako_ns >= UINT64_C(20000000) && c_ns >= UINT64_C(20000000)) break;
    if (iterations > UINT64_MAX / 2) return 0;
    iterations *= 2;
  }
  for (unsigned warmup = 0; warmup < 10; warmup++) {
    (void)measure(warmup & 1, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
    (void)measure(!(warmup & 1), &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
  }
  for (unsigned sample = 0; sample < 51; sample++) {
    if ((sample & 1) == 0) {
      hako_ns = measure(1, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
      c_ns = measure(0, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
    } else {
      c_ns = measure(0, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
      hako_ns = measure(1, &frame.roots[0], &frame.roots[1], item->width, iterations, &sink);
    }
    printf(
        "%s,%s,%u,%" PRIu64 ",%" PRIu64 ",%" PRIu64 ",%" PRIu64 "\n",
        item->name, item->category, sample, iterations, hako_ns, c_ns, sink);
  }
  hako_text_formal_residence_finish_or_abort_v1(&frame.header);
  hako_promotion_test_drop_wire_v1(subject_wire);
  if (!item->alias) hako_promotion_test_drop_wire_v1(needle_wire);
  return 1;
}

int main(void) {
  puts("case,category,sample,iterations,hako_ns,c_ns,sink");
  for (size_t i = 0; i < sizeof(CASES) / sizeof(CASES[0]); i++) {
    if (!run_case(&CASES[i])) {
      fprintf(stderr, "exact leaf case failed: %s\n", CASES[i].name);
      return 1;
    }
  }
  return 0;
}
