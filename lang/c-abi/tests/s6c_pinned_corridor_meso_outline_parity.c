#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "../../../include/nyrt_text_formal_residence_v1.h"

typedef struct HakoPromotionTestWireV1 {
  uint64_t slot;
  uint64_t generation;
} HakoPromotionTestWireV1;

typedef struct MesoFrameV1 {
  NyrtTextFormalResidenceFrameV1 header;
  NyrtTextFormalResidenceRootRowV1 roots[2];
} MesoFrameV1;

extern HakoPromotionTestWireV1 hako_promotion_test_issue_text_wire_v1(const char* text);
extern void hako_promotion_test_drop_wire_v1(HakoPromotionTestWireV1 wire);
extern int64_t hako_s6c_candidate(uint64_t, uint64_t, uint64_t, uint64_t);
extern int64_t hako_s6c_meso(const uint8_t*, uint64_t, const uint8_t*, uint64_t);

static size_t scalar_width(const unsigned char* text, size_t remaining) {
  if (!remaining) return 0;
  if (text[0] < 0x80) return 1;
  if (text[0] < 0xe0 && remaining >= 2) return 2;
  if (text[0] < 0xf0 && remaining >= 3) return 3;
  if (text[0] < 0xf8 && remaining >= 4) return 4;
  return 0;
}

static int64_t oracle(const char* subject, const char* needle) {
  const unsigned char* bytes = (const unsigned char*)subject;
  size_t length = strlen(subject), needle_length = strlen(needle), offset = 0;
  int64_t index = 0;
  while (offset < length) {
    size_t width = scalar_width(bytes + offset, length - offset);
    if (!width) return -2;
    if (width == needle_length && memcmp(bytes + offset, needle, width) == 0) return index;
    offset += width;
    index++;
  }
  return -1;
}

static int run_case(const char* label, const char* subject_text, const char* needle_text, int alias) {
  HakoPromotionTestWireV1 subject = hako_promotion_test_issue_text_wire_v1(subject_text);
  HakoPromotionTestWireV1 needle = alias ? subject : hako_promotion_test_issue_text_wire_v1(needle_text);
  NyrtTextFormalBorrowV1 pairs[2] = {
      {subject.slot, subject.generation}, {needle.slot, needle.generation},
  };
  MesoFrameV1 frame;
  int64_t expected, whole, outlined;
  memset(&frame, 0, sizeof(frame));
  if (!subject.slot || !needle.slot || hako_text_formal_residence_enter_v1(
      pairs, 2, &frame.header, (uint32_t)sizeof(frame)) != NYRT_TEXT_RESIDENCE_VALID_V1) return 0;
  expected = oracle(subject_text, needle_text);
  whole = hako_s6c_candidate(subject.slot, subject.generation, needle.slot, needle.generation);
  outlined = hako_s6c_meso(frame.roots[0].ptr, frame.roots[0].byte_len,
                           frame.roots[1].ptr, frame.roots[1].byte_len);
  hako_text_formal_residence_finish_or_abort_v1(&frame.header);
  hako_promotion_test_drop_wire_v1(subject);
  if (!alias) hako_promotion_test_drop_wire_v1(needle);
  if (expected != whole || expected != outlined) {
    fprintf(stderr, "%s: oracle=%lld whole=%lld outline=%lld\n", label,
            (long long)expected, (long long)whole, (long long)outlined);
    return 0;
  }
  return 1;
}

int main(void) {
  static const struct { const char* label; const char* subject; const char* needle; int alias; } cases[] = {
    {"empty-subject", "", "x", 0}, {"empty-needle", "abc", "", 0},
    {"w1-first", "abc", "a", 0}, {"w1-middle", "abc", "b", 0},
    {"w1-last", "abc", "c", 0}, {"w1-miss", "abc", "x", 0},
    {"w2-first", "αβγ", "α", 0}, {"w2-middle", "αβγ", "β", 0},
    {"w2-last", "αβγ", "γ", 0}, {"w2-miss", "αβγ", "δ", 0},
    {"w3-first", "あいう", "あ", 0}, {"w3-middle", "あいう", "い", 0},
    {"w3-last", "あいう", "う", 0}, {"w3-miss", "あいう", "え", 0},
    {"w4-first", "😀😺🙀", "😀", 0}, {"w4-middle", "😀😺🙀", "😺", 0},
    {"w4-last", "😀😺🙀", "🙀", 0}, {"w4-miss", "😀😺🙀", "😸", 0},
    {"mixed-first", "aβあ😺z", "a", 0}, {"mixed-middle", "aβあ😺z", "あ", 0},
    {"mixed-last", "aβあ😺z", "z", 0}, {"mixed-miss", "aβあ😺z", "x", 0},
    {"alias-one", "x", "x", 1}, {"alias-multi", "ab", "ab", 1},
  };
  for (size_t index = 0; index < sizeof(cases) / sizeof(cases[0]); index++) {
    if (!run_case(cases[index].label, cases[index].subject, cases[index].needle, cases[index].alias)) return 1;
  }
  puts("[s6c-pinned-corridor-meso-outline-parity] ok");
  return 0;
}
