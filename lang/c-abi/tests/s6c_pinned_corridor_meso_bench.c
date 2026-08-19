#define _GNU_SOURCE
#include "../../../include/nyrt_text_formal_residence_v1.h"

#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

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

static uint64_t now_ns(void) {
  struct timespec value;
  if (clock_gettime(CLOCK_MONOTONIC_RAW, &value) != 0) abort();
  return (uint64_t)value.tv_sec * UINT64_C(1000000000) + (uint64_t)value.tv_nsec;
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

int main(void) {
  puts("family,size,position,sample,iterations,hako_ns,c_ns,sink,scalars,width1,width2,width3,width4");
  for (size_t family = 0; family < 5; family++)
    for (size_t size = 0; size < 4; size++)
      for (size_t position = 0; position < 4; position++)
        if (!run_case(FAMILIES[family], SIZES[size], POSITIONS[position])) return 1;
  return 0;
}
