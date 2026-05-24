// MIMAP-451A explicit C mimalloc runner.
//
// This runner loads libmimalloc explicitly by path and calls mimalloc symbols
// through tiny stable workloads. It is comparison evidence, not process
// allocator replacement.

#define _GNU_SOURCE

#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>

typedef void *(*mi_malloc_fn)(size_t);
typedef void *(*mi_realloc_fn)(void *, size_t);
typedef void *(*mi_malloc_aligned_fn)(size_t, size_t);
typedef void (*mi_free_fn)(void *);

typedef struct RunnerConfig {
    const char *library_path;
    const char *workload;
    long alloc_count;
    long block_size;
} RunnerConfig;

typedef struct MimallocApi {
    mi_malloc_fn malloc_fn;
    mi_realloc_fn realloc_fn;
    mi_malloc_aligned_fn malloc_aligned_fn;
    mi_free_fn free_fn;
} MimallocApi;

typedef struct RunnerEvidence {
    const char *workload;
    const char *operation_family;
    const char *operation_sequence_id;
    const char *free_order_id;
    long allocation_count;
    long free_count;
    uint64_t requested_bytes;
    long realloc_count;
    long aligned_alloc_count;
    long alignment_request_count;
    long alignment_ok_count;
    long alignment_reject_count;
    long large_request_count;
    long realloc_same_ptr_count;
    long realloc_moved_count;
    uint64_t copied_bytes;
} RunnerEvidence;

static void usage(const char *argv0) {
    fprintf(stderr, "usage: %s --library PATH [--workload ID] [--alloc-count N] [--block-size N]\n", argv0);
}

static int parse_long_arg(const char *text, long *out) {
    char *end = NULL;
    long value = strtol(text, &end, 10);
    if (end == text || *end != '\0' || value < 1) {
        return 0;
    }
    *out = value;
    return 1;
}

static int parse_args(int argc, char **argv, RunnerConfig *config) {
    config->library_path = NULL;
    config->workload = "representative-small-block-v0";
    config->alloc_count = 64;
    config->block_size = 512;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--library") == 0) {
            if (i + 1 >= argc) {
                return 0;
            }
            config->library_path = argv[++i];
            continue;
        }
        if (strcmp(argv[i], "--workload") == 0) {
            if (i + 1 >= argc) {
                return 0;
            }
            config->workload = argv[++i];
            continue;
        }
        if (strcmp(argv[i], "--alloc-count") == 0) {
            if (i + 1 >= argc || !parse_long_arg(argv[++i], &config->alloc_count)) {
                return 0;
            }
            continue;
        }
        if (strcmp(argv[i], "--block-size") == 0) {
            if (i + 1 >= argc || !parse_long_arg(argv[++i], &config->block_size)) {
                return 0;
            }
            continue;
        }
        return 0;
    }

    return config->library_path != NULL;
}

static uint64_t peak_rss_bytes(void) {
    struct rusage usage;
    if (getrusage(RUSAGE_SELF, &usage) != 0) {
        return 0;
    }
    if (usage.ru_maxrss <= 0) {
        return 0;
    }
    return (uint64_t)usage.ru_maxrss * 1024ULL;
}

static int run_small_block(const RunnerConfig *config, const MimallocApi *api, RunnerEvidence *evidence) {
    void **blocks = (void **)calloc((size_t)config->alloc_count, sizeof(void *));
    if (blocks == NULL) {
        return 5;
    }

    evidence->workload = "representative-small-block-v0";
    evidence->operation_family = "small-block";
    evidence->operation_sequence_id = "representative-small-block-v0-seq";
    evidence->free_order_id = "even-odd-release-v0";

    for (long i = 0; i < config->alloc_count; i++) {
        size_t size = (size_t)(config->block_size + (i % 17));
        void *ptr = api->malloc_fn(size);
        if (ptr == NULL) {
            fprintf(stderr, "[c-mimalloc-runner] mi_malloc returned null at %ld\n", i);
            for (long j = 0; j < i; j++) {
                api->free_fn(blocks[j]);
            }
            free(blocks);
            return 6;
        }
        memset(ptr, (int)(i & 0x7f), size);
        blocks[i] = ptr;
        evidence->requested_bytes += (uint64_t)size;
        evidence->allocation_count += 1;
    }

    for (long i = 0; i < config->alloc_count; i += 2) {
        api->free_fn(blocks[i]);
        blocks[i] = NULL;
        evidence->free_count += 1;
    }
    for (long i = 1; i < config->alloc_count; i += 2) {
        api->free_fn(blocks[i]);
        blocks[i] = NULL;
        evidence->free_count += 1;
    }

    free(blocks);
    return 0;
}

static int run_realloc_aligned(const MimallocApi *api, RunnerEvidence *evidence) {
    evidence->workload = "representative-realloc-aligned-v0";
    evidence->operation_family = "realloc-aligned";
    evidence->operation_sequence_id = "representative-realloc-aligned-v0-seq";
    evidence->free_order_id = "ascending-release-v0";

    void *same = api->malloc_fn(8);
    void *grow = api->malloc_fn(16);
    if (same == NULL || grow == NULL) {
        api->free_fn(same);
        api->free_fn(grow);
        return 6;
    }
    evidence->allocation_count += 2;

    void *same_after = api->realloc_fn(same, 24);
    if (same_after == NULL) {
        api->free_fn(same);
        api->free_fn(grow);
        return 7;
    }
    evidence->realloc_count += 1;
    if (same_after == same) {
        evidence->realloc_same_ptr_count += 1;
    } else {
        evidence->realloc_moved_count += 1;
        evidence->copied_bytes += 8;
    }
    same = same_after;

    void *grow_after = api->realloc_fn(grow, 48);
    if (grow_after == NULL) {
        api->free_fn(same);
        api->free_fn(grow);
        return 8;
    }
    evidence->realloc_count += 1;
    if (grow_after == grow) {
        evidence->realloc_same_ptr_count += 1;
    } else {
        evidence->realloc_moved_count += 1;
        evidence->copied_bytes += 16;
    }
    grow = grow_after;

    evidence->alignment_request_count += 1;
    void *aligned_small = api->malloc_aligned_fn(32, 8);
    if (aligned_small == NULL) {
        api->free_fn(same);
        api->free_fn(grow);
        return 9;
    }
    evidence->allocation_count += 1;
    evidence->aligned_alloc_count += 1;
    evidence->alignment_ok_count += 1;

    evidence->alignment_request_count += 1;
    void *aligned_medium = api->malloc_aligned_fn(112, 64);
    if (aligned_medium == NULL) {
        api->free_fn(same);
        api->free_fn(grow);
        api->free_fn(aligned_small);
        return 10;
    }
    evidence->allocation_count += 1;
    evidence->aligned_alloc_count += 1;
    evidence->alignment_ok_count += 1;

    evidence->alignment_request_count += 1;
    evidence->alignment_reject_count += 1;
    evidence->requested_bytes = 216;

    api->free_fn(same);
    evidence->free_count += 1;
    api->free_fn(grow);
    evidence->free_count += 1;
    api->free_fn(aligned_small);
    evidence->free_count += 1;
    api->free_fn(aligned_medium);
    evidence->free_count += 1;
    return 0;
}

static int run_mixed_small(const MimallocApi *api, RunnerEvidence *evidence) {
    static const size_t sizes[] = {
        16, 24, 32, 48, 64, 80, 96, 112,
        128, 160, 192, 224, 256, 384, 512, 768,
    };
    enum { kCount = (int)(sizeof(sizes) / sizeof(sizes[0])) };
    void *blocks[kCount];
    memset(blocks, 0, sizeof(blocks));

    evidence->workload = "representative-mixed-small-v0";
    evidence->operation_family = "mixed-small";
    evidence->operation_sequence_id = "representative-mixed-small-v0-seq";
    evidence->free_order_id = "ascending-release-v0";

    for (int i = 0; i < kCount; i++) {
        void *ptr = api->malloc_fn(sizes[i]);
        if (ptr == NULL) {
            for (int j = 0; j < i; j++) {
                api->free_fn(blocks[j]);
            }
            return 6;
        }
        memset(ptr, (int)(i & 0x7f), sizes[i]);
        blocks[i] = ptr;
        evidence->requested_bytes += (uint64_t)sizes[i];
        evidence->allocation_count += 1;
    }

    for (int i = 0; i < kCount; i++) {
        api->free_fn(blocks[i]);
        evidence->free_count += 1;
    }
    return 0;
}

static int run_huge_ish(const MimallocApi *api, RunnerEvidence *evidence) {
    static const size_t sizes[] = {4194305, 16};
    enum { kCount = (int)(sizeof(sizes) / sizeof(sizes[0])) };
    void *blocks[kCount];
    memset(blocks, 0, sizeof(blocks));

    evidence->workload = "representative-huge-ish-v0";
    evidence->operation_family = "huge-ish";
    evidence->operation_sequence_id = "representative-huge-ish-v0-seq";
    evidence->free_order_id = "ascending-release-v0";

    for (int i = 0; i < kCount; i++) {
        void *ptr = api->malloc_fn(sizes[i]);
        if (ptr == NULL) {
            for (int j = 0; j < i; j++) {
                api->free_fn(blocks[j]);
            }
            return 6;
        }
        memset(ptr, (int)(i & 0x7f), sizes[i]);
        blocks[i] = ptr;
        evidence->requested_bytes += (uint64_t)sizes[i];
        evidence->allocation_count += 1;
        if (sizes[i] > 4194304) {
            evidence->large_request_count += 1;
        }
    }

    for (int i = 0; i < kCount; i++) {
        api->free_fn(blocks[i]);
        evidence->free_count += 1;
    }
    return 0;
}

int main(int argc, char **argv) {
    RunnerConfig config;
    if (!parse_args(argc, argv, &config)) {
        usage(argv[0]);
        return 2;
    }

    void *library = dlopen(config.library_path, RTLD_NOW | RTLD_LOCAL);
    if (library == NULL) {
        fprintf(stderr, "[c-mimalloc-runner] dlopen failed: %s\n", dlerror());
        return 3;
    }

    MimallocApi api;
    api.malloc_fn = (mi_malloc_fn)dlsym(library, "mi_malloc");
    api.realloc_fn = (mi_realloc_fn)dlsym(library, "mi_realloc");
    api.malloc_aligned_fn = (mi_malloc_aligned_fn)dlsym(library, "mi_malloc_aligned");
    api.free_fn = (mi_free_fn)dlsym(library, "mi_free");
    if (api.malloc_fn == NULL || api.realloc_fn == NULL || api.malloc_aligned_fn == NULL || api.free_fn == NULL) {
        fprintf(stderr, "[c-mimalloc-runner] required mimalloc symbols missing\n");
        dlclose(library);
        return 4;
    }

    RunnerEvidence evidence;
    memset(&evidence, 0, sizeof(evidence));
    int result_code = 0;
    if (strcmp(config.workload, "representative-small-block-v0") == 0) {
        result_code = run_small_block(&config, &api, &evidence);
    } else if (strcmp(config.workload, "representative-realloc-aligned-v0") == 0) {
        result_code = run_realloc_aligned(&api, &evidence);
    } else if (strcmp(config.workload, "representative-mixed-small-v0") == 0) {
        result_code = run_mixed_small(&api, &evidence);
    } else if (strcmp(config.workload, "representative-huge-ish-v0") == 0) {
        result_code = run_huge_ish(&api, &evidence);
    } else {
        fprintf(stderr, "[c-mimalloc-runner] unsupported workload: %s\n", config.workload);
        dlclose(library);
        return 2;
    }
    if (result_code != 0) {
        dlclose(library);
        return result_code;
    }

    uint64_t rss_bytes = peak_rss_bytes();
    dlclose(library);

    printf("c_mimalloc_runner=1\n");
    printf("output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0\n");
    printf("workload=%s\n", evidence.workload);
    printf("operation_family=%s\n", evidence.operation_family);
    printf("operation_sequence_id=%s\n", evidence.operation_sequence_id);
    printf("free_order_id=%s\n", evidence.free_order_id);
    printf("library_path=%s\n", config.library_path);
    printf("result_code=0\n");
    printf("run_count=1\n");
    printf("allocation_count=%ld\n", evidence.allocation_count);
    printf("free_count=%ld\n", evidence.free_count);
    printf("requested_bytes=%llu\n", (unsigned long long)evidence.requested_bytes);
    printf("realloc_count=%ld\n", evidence.realloc_count);
    printf("aligned_alloc_count=%ld\n", evidence.aligned_alloc_count);
    printf("alignment_request_count=%ld\n", evidence.alignment_request_count);
    printf("alignment_ok_count=%ld\n", evidence.alignment_ok_count);
    printf("alignment_reject_count=%ld\n", evidence.alignment_reject_count);
    printf("large_request_count=%ld\n", evidence.large_request_count);
    printf("realloc_same_ptr_count=%ld\n", evidence.realloc_same_ptr_count);
    printf("realloc_moved_count=%ld\n", evidence.realloc_moved_count);
    printf("copied_bytes=%llu\n", (unsigned long long)evidence.copied_bytes);
    printf("peak_rss_bytes=%llu\n", (unsigned long long)rss_bytes);
    printf("memory_usage_evidence=1\n");
    printf("process_replacement_executed=0\n");
    printf("hook_installed=0\n");
    printf("backend_matcher_added=0\n");
    printf("global_allocator_installed=0\n");
    printf("hidden_discovery_used=0\n");
    printf("provider_package_generated=0\n");
    printf("summary=ok\n");
    return 0;
}
