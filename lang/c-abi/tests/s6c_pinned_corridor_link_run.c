#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include "../../../include/nyrt_text_formal_residence_v1.h"

typedef struct HakoPromotionTestWireV1 {
  uint64_t slot;
  uint64_t generation;
} HakoPromotionTestWireV1;

extern HakoPromotionTestWireV1 hako_promotion_test_issue_text_wire_v1(
    const char* text);
extern HakoPromotionTestWireV1 hako_promotion_test_issue_non_text_wire_v1(void);
extern void hako_promotion_test_drop_wire_v1(HakoPromotionTestWireV1 wire);

extern int64_t hako_s6c_candidate(
    uint64_t subject_slot,
    uint64_t subject_generation,
    uint64_t needle_slot,
    uint64_t needle_generation);

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
  size_t length = strlen(subject);
  size_t needle_length = strlen(needle);
  size_t offset = 0;
  int64_t index = 0;
  while (offset < length) {
    size_t width = scalar_width(bytes + offset, length - offset);
    if (!width) return -2;
    if (width == needle_length && memcmp(bytes + offset, needle, width) == 0) {
      return index;
    }
    offset += width;
    index++;
  }
  return -1;
}

static int residence_can_reenter(
    HakoPromotionTestWireV1 subject,
    HakoPromotionTestWireV1 needle) {
  NyrtTextFormalBorrowV1 pairs[2] = {
      {subject.slot, subject.generation},
      {needle.slot, needle.generation},
  };
  uint64_t storage[8] = {0};
  NyrtTextFormalResidenceFrameV1* frame =
      (NyrtTextFormalResidenceFrameV1*)storage;
  uint32_t status = hako_text_formal_residence_enter_v1(
      pairs, 2, frame, (uint32_t)sizeof(storage));
  if (status != NYRT_TEXT_RESIDENCE_VALID_V1) return 0;
  hako_text_formal_residence_finish_or_abort_v1(frame);
  return 1;
}

static int run_case(
    const char* label,
    const char* subject_text,
    const char* needle_text,
    int64_t expected,
    int alias) {
  HakoPromotionTestWireV1 subject =
      hako_promotion_test_issue_text_wire_v1(subject_text);
  HakoPromotionTestWireV1 needle = alias
      ? subject
      : hako_promotion_test_issue_text_wire_v1(needle_text);
  int64_t reference = oracle(subject_text, needle_text);
  int64_t actual = hako_s6c_candidate(
      subject.slot, subject.generation, needle.slot, needle.generation);
  int ok = subject.slot != 0 && needle.slot != 0 && reference == expected &&
      actual == expected && residence_can_reenter(subject, needle);
  if (!ok) {
    fprintf(
        stderr,
        "%s: expected=%lld oracle=%lld actual=%lld\n",
        label,
        (long long)expected,
        (long long)reference,
        (long long)actual);
  }
  hako_promotion_test_drop_wire_v1(subject);
  if (!alias) hako_promotion_test_drop_wire_v1(needle);
  return ok;
}

static int candidate_traps(
    HakoPromotionTestWireV1 subject,
    HakoPromotionTestWireV1 needle) {
  pid_t child = fork();
  int status = 0;
  if (child < 0) return 0;
  if (child == 0) {
    (void)hako_s6c_candidate(
        subject.slot, subject.generation, needle.slot, needle.generation);
    _exit(0);
  }
  if (waitpid(child, &status, 0) != child) return 0;
  return WIFSIGNALED(status);
}

static int run_reject_cases(void) {
  HakoPromotionTestWireV1 valid =
      hako_promotion_test_issue_text_wire_v1("x");
  HakoPromotionTestWireV1 stale =
      hako_promotion_test_issue_text_wire_v1("stale");
  HakoPromotionTestWireV1 non_text =
      hako_promotion_test_issue_non_text_wire_v1();
  HakoPromotionTestWireV1 zero = {0, 0};
  HakoPromotionTestWireV1 foreign = valid;
  NyrtTextFormalBorrowV1 pending_pair;
  uint64_t pending_storage[8] = {0};
  NyrtTextFormalResidenceFrameV1* pending_frame =
      (NyrtTextFormalResidenceFrameV1*)pending_storage;
  uint32_t pending_status;
  int ok;

  foreign.generation++;
  hako_promotion_test_drop_wire_v1(stale);
  pending_pair.slot = valid.slot;
  pending_pair.generation = valid.generation;
  pending_status = hako_text_formal_residence_enter_v1(
      &pending_pair, 1, pending_frame, (uint32_t)sizeof(pending_storage));
  if (pending_status != NYRT_TEXT_RESIDENCE_VALID_V1) return 0;
  hako_promotion_test_drop_wire_v1(valid);

  ok = candidate_traps(zero, zero) && candidate_traps(stale, stale) &&
      candidate_traps(foreign, foreign) && candidate_traps(non_text, non_text) &&
      candidate_traps(valid, valid);

  hako_text_formal_residence_finish_or_abort_v1(pending_frame);
  hako_promotion_test_drop_wire_v1(non_text);
  return ok;
}

int main(void) {
  static const struct {
    const char* label;
    const char* subject;
    const char* needle;
    int64_t expected;
    int alias;
  } cases[] = {
      {"empty-subject", "", "x", -1, 0},
      {"empty-needle", "abc", "", -1, 0},
      {"ascii-first", "abc", "a", 0, 0},
      {"ascii-middle", "abc", "b", 1, 0},
      {"ascii-last", "abc", "c", 2, 0},
      {"ascii-miss", "abc", "x", -1, 0},
      {"utf8-two", "αβγ", "β", 1, 0},
      {"utf8-three", "あいう", "い", 1, 0},
      {"utf8-four",
       "\xf0\x9f\x98\x80" "\xf0\x9f\x98\xba" "z",
       "\xf0\x9f\x98\xba",
       1,
       0},
      {"mixed",
       "a\xce\xb2\xe3\x81\x82" "\xf0\x9f\x98\xba" "z",
       "\xf0\x9f\x98\xba",
       3,
       0},
      {"combining", "éx", "́", 1, 0},
      {"composed-miss", "éx", "é", -1, 0},
      {"multi-scalar", "abc", "ab", -1, 0},
      {"alias-one", "x", "x", 0, 1},
      {"alias-multi", "ab", "ab", -1, 1},
  };
  size_t index;
  for (index = 0; index < sizeof(cases) / sizeof(cases[0]); index++) {
    if (!run_case(
            cases[index].label,
            cases[index].subject,
            cases[index].needle,
            cases[index].expected,
            cases[index].alias)) {
      return 1;
    }
  }
  if (!run_reject_cases()) return 2;
  puts("[s6c-pinned-corridor-link-run] ok");
  return 0;
}
