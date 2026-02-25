#!/bin/bash
./target/release/nyash local_tests/test_filebox_debug.hako 2>&1 | grep -E "(TLV data|Plugin method returned)"