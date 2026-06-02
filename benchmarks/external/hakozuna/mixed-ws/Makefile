CC ?= cc
CFLAGS ?= -O3 -Wall -Wextra -std=c11 \
	-D_GNU_SOURCE -D_POSIX_C_SOURCE=199309L \
	-DHZ3_BENCH_USE_CRT=1
LDFLAGS ?=
LDLIBS ?= -ldl -lm -pthread

BUILD_DIR := build
TARGET := $(BUILD_DIR)/bench_mixed_ws_crt

.PHONY: all clean

all: $(TARGET)

$(BUILD_DIR):
	mkdir -p $@

$(TARGET): bench_mixed_ws.c hz3.h | $(BUILD_DIR)
	$(CC) $(CFLAGS) -I. $< -o $@ $(LDFLAGS) $(LDLIBS)

clean:
	rm -rf $(BUILD_DIR)
