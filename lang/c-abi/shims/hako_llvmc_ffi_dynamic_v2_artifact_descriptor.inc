/*
 * Artifact-bound projection of the already-validated C1 metadata.
 *
 * This owner emits one fixed-width little-endian descriptor into the object.
 * It does not select entries, invent ABI facts, link, or issue a receipt.
 */

#include "../../../include/hako_dynamic_v2_artifact_descriptor_v1.h"

#define HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT(stage)                              \
  do {                                                                         \
    if (hako_llvmc_route_trace_enabled()) {                                    \
      fprintf(stderr, "[dynamic-v2/artifact-descriptor] reject=%s\n", stage); \
    }                                                                          \
    return 0;                                                                  \
  } while (0)

static void hako_dynamic_v2_descriptor_write_u32(
    unsigned char* bytes,
    size_t offset,
    uint32_t value) {
  for (size_t i = 0; i < 4; i++) {
    bytes[offset + i] = (unsigned char)((value >> (i * 8)) & UINT32_C(0xff));
  }
}

static void hako_dynamic_v2_descriptor_write_u64(
    unsigned char* bytes,
    size_t offset,
    uint64_t value) {
  for (size_t i = 0; i < 8; i++) {
    bytes[offset + i] = (unsigned char)((value >> (i * 8)) & UINT64_C(0xff));
  }
}

static int hako_dynamic_v2_descriptor_write_text(
    unsigned char* bytes,
    size_t offset,
    size_t capacity,
    const char* value) {
  size_t length;
  if (!bytes || !value || capacity == 0) return 0;
  length = strlen(value);
  if (length == 0 || length >= capacity) return 0;
  memcpy(bytes + offset, value, length);
  return 1;
}

static int hako_llvmc_c1_build_artifact_descriptor(
    yyjson_val* metadata,
    unsigned char bytes[HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE]) {
  yyjson_val* stamp;
  yyjson_val* calls;
  const char* contract_id;
  int ok = 0;
  long long profile;
  long long abi;
  long long wire;
  uint64_t compiler_domain = 0;
  uint64_t invocation_ordinal = 0;
  uint64_t registry_generation = 0;
  if (!metadata || !bytes) HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("missing_input");
  memset(bytes, 0, HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE);
  contract_id = hako_llvmc_c1_text(metadata, "contract_id");
  profile = hako_llvmc_c1_int(metadata, "profile", &ok);
  if (!ok || profile <= 0) HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("profile");
  abi = hako_llvmc_c1_int(metadata, "abi_revision", &ok);
  if (!ok || abi <= 0) HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("abi");
  wire = hako_llvmc_c1_int(metadata, "wire_revision", &ok);
  if (!ok || wire <= 0) HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("wire");
  if (!hako_llvmc_c1_u64(metadata, "registry_generation", &registry_generation) ||
      registry_generation == 0)
    HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("registry_generation");
  stamp = yyjson_obj_get(metadata, "plan_stamp");
  if (!hako_llvmc_c1_u64(stamp, "compiler_domain", &compiler_domain) ||
      compiler_domain == 0)
    HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("compiler_domain");
  if (!hako_llvmc_c1_u64(stamp, "invocation_ordinal", &invocation_ordinal) ||
      invocation_ordinal == 0)
    HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("invocation_ordinal");
  calls = yyjson_obj_get(metadata, "calls");
  if (!(calls && yyjson_is_arr(calls) && yyjson_arr_size(calls) == 2))
    HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("calls");

  memcpy(
      bytes + HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_MAGIC,
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_MAGIC,
      strlen(HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_MAGIC));
  hako_dynamic_v2_descriptor_write_u32(
      bytes,
      HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_SCHEMA,
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SCHEMA);
  hako_dynamic_v2_descriptor_write_u32(
      bytes,
      HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_SIZE,
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE);
  hako_dynamic_v2_descriptor_write_u32(
      bytes, HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_PROFILE, (uint32_t)profile);
  hako_dynamic_v2_descriptor_write_u32(
      bytes, HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_ABI, (uint32_t)abi);
  hako_dynamic_v2_descriptor_write_u32(
      bytes, HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_WIRE, (uint32_t)wire);
  hako_dynamic_v2_descriptor_write_u32(
      bytes,
      HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_ENTRY_COUNT,
      HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_COUNT);
  hako_dynamic_v2_descriptor_write_u64(
      bytes,
      HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_COMPILER_DOMAIN,
      compiler_domain);
  hako_dynamic_v2_descriptor_write_u64(
      bytes,
      HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_INVOCATION_ORDINAL,
      invocation_ordinal);
  hako_dynamic_v2_descriptor_write_u64(
      bytes,
      HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_REGISTRY_GENERATION,
      registry_generation);
  if (!hako_dynamic_v2_descriptor_write_text(
          bytes,
          HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_CONTRACT_ID,
          HAKO_DYNAMIC_V2_ARTIFACT_CONTRACT_CAPACITY,
          contract_id)) {
    HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("contract_id");
  }

  for (size_t site = 0; site < 2; site++) {
    yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, (long long)site);
    yyjson_val* args;
    const char* symbol;
    long long entry_id;
    size_t entry_offset = HAKO_DYNAMIC_V2_ARTIFACT_OFFSET_ENTRIES +
                          site * HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_SIZE;
    if (!call) HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("call_site");
    entry_id = hako_llvmc_c1_int(call, "entry_id", &ok);
    if (!ok || entry_id <= 0) HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("entry_id");
    args = yyjson_obj_get(call, "argument_lanes");
    symbol = hako_llvmc_c1_text(call, "symbol");
    if (!(args && yyjson_is_arr(args)))
      HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("argument_lanes");
    hako_dynamic_v2_descriptor_write_u32(
        bytes,
        entry_offset + HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_SITE_ID,
        (uint32_t)site);
    hako_dynamic_v2_descriptor_write_u32(
        bytes,
        entry_offset + HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_ID,
        (uint32_t)entry_id);
    hako_dynamic_v2_descriptor_write_u32(
        bytes,
        entry_offset + HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_ARITY,
        (uint32_t)yyjson_arr_size(args));
    if (!hako_dynamic_v2_descriptor_write_text(
            bytes,
            entry_offset + HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_OFFSET_SYMBOL,
            HAKO_DYNAMIC_V2_ARTIFACT_ENTRY_SYMBOL_CAPACITY,
            symbol)) {
      HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT("entry_symbol");
    }
  }
  return 1;
}

static int hako_llvmc_c1_emit_artifact_descriptor(
    FILE* f,
    yyjson_val* metadata) {
  unsigned char bytes[HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE];
  yyjson_val* nested;
  if (!metadata) return 1;
  nested = yyjson_obj_get(metadata, "dynamic_v2_aot_call_admission_v2");
  if (!nested) return 1;
  if (!yyjson_is_obj(nested)) return 0;
  if (!hako_llvmc_c1_build_artifact_descriptor(nested, bytes)) return 0;
  fprintf(
      f,
      "@%s = constant [%u x i8] c\"",
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SYMBOL,
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE);
  for (size_t i = 0; i < HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SIZE; i++) {
    fprintf(f, "\\%02X", (unsigned int)bytes[i]);
  }
  fprintf(
      f,
      "\", section \"%s\", align 8\n",
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SECTION);
  fprintf(
      f,
      "@llvm.used = appending global [1 x ptr] [ptr @%s], section \"llvm.metadata\"\n\n",
      HAKO_DYNAMIC_V2_ARTIFACT_DESCRIPTOR_SYMBOL);
  return 1;
}

#undef HAKO_DYNAMIC_V2_DESCRIPTOR_REJECT
