// MIMAP-451A explicit C mimalloc runner.
//
// This runner loads libmimalloc explicitly by path and calls mi_malloc/mi_free
// through a tiny stable workload. It is a comparison evidence tool, not process
// allocator replacement.

#define _GNU_SOURCE

#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>

typedef void *(*mi_malloc_fn)(size_t);
typedef void (*mi_free_fn)(void *);

typedef struct RunnerConfig {
    const char *library_path;
    long alloc_count;
    long block_size;
} RunnerConfig;

static void usage(const char *argv0) {
    fprintf(stderr, "usage: %s --library PATH [--alloc-count N] [--block-size N]\n", argv0);
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

    mi_malloc_fn mi_malloc_ptr = (mi_malloc_fn)dlsym(library, "mi_malloc");
    mi_free_fn mi_free_ptr = (mi_free_fn)dlsym(library, "mi_free");
    if (mi_malloc_ptr == NULL || mi_free_ptr == NULL) {
        fprintf(stderr, "[c-mimalloc-runner] required mimalloc symbols missing\n");
        dlclose(library);
        return 4;
    }

    void **blocks = (void **)calloc((size_t)config.alloc_count, sizeof(void *));
    if (blocks == NULL) {
        dlclose(library);
        return 5;
    }

    uint64_t requested_bytes = 0;
    long allocation_count = 0;
    long free_count = 0;
    for (long i = 0; i < config.alloc_count; i++) {
        size_t size = (size_t)(config.block_size + (i % 17));
        void *ptr = mi_malloc_ptr(size);
        if (ptr == NULL) {
            fprintf(stderr, "[c-mimalloc-runner] mi_malloc returned null at %ld\n", i);
            for (long j = 0; j < i; j++) {
                mi_free_ptr(blocks[j]);
            }
            free(blocks);
            dlclose(library);
            return 6;
        }
        memset(ptr, (int)(i & 0x7f), size);
        blocks[i] = ptr;
        requested_bytes += (uint64_t)size;
        allocation_count += 1;
    }

    for (long i = 0; i < config.alloc_count; i += 2) {
        mi_free_ptr(blocks[i]);
        blocks[i] = NULL;
        free_count += 1;
    }
    for (long i = 1; i < config.alloc_count; i += 2) {
        mi_free_ptr(blocks[i]);
        blocks[i] = NULL;
        free_count += 1;
    }

    free(blocks);
    uint64_t rss_bytes = peak_rss_bytes();
    dlclose(library);

    printf("c_mimalloc_runner=1\n");
    printf("output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0\n");
    printf("workload=representative-small-block-v0\n");
    printf("library_path=%s\n", config.library_path);
    printf("result_code=0\n");
    printf("run_count=1\n");
    printf("allocation_count=%ld\n", allocation_count);
    printf("free_count=%ld\n", free_count);
    printf("requested_bytes=%llu\n", (unsigned long long)requested_bytes);
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
