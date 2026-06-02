CC ?= cc
CFLAGS ?= -O3 -Wall -Wextra -std=c11 \
	-D_GNU_SOURCE -D_POSIX_C_SOURCE=199309L \
	-D_GLIBC_USE_ISOC2X=0 \
	-D__isoc23_strtol=strtol -D__isoc23_strtoll=strtoll \
	-D__isoc23_strtoul=strtoul -D__isoc23_strtoull=strtoull
LDFLAGS ?=
LDLIBS ?= -ldl -lm -pthread

BUILD_DIR := build
TARGET := $(BUILD_DIR)/bench_mid_large_mt_system

.PHONY: all clean

all: $(TARGET)

$(BUILD_DIR):
	mkdir -p $@

$(TARGET): bench_mid_large_mt.c core/bench_profile.h | $(BUILD_DIR)
	$(CC) $(CFLAGS) -I. $< -o $@ $(LDFLAGS) $(LDLIBS)

clean:
	rm -rf $(BUILD_DIR)
