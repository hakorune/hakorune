#include <stdio.h>
#include <stdint.h>

// Minimal C runtime symbols (design-stage). These provide a safe, tiny set of
// externs for experiments; real NyKernel remains authoritative.

// Print: accept pointer (may be NULL). Returns 0 on success.
long nyash_console_log(char* p) {
    (void)p;
    puts("hello");
    return 0;
}

// from_i8_string: returns a fake handle (0). Real mapping is in Rust NyKernel.
long nyash_box_from_i8_string(char* p) {
    (void)p; // not used in design stage stub
    return 0;
}

// from_i8_string_const: returns a fake handle (0). Real intern cache is in Rust NyKernel.
long nyash_box_from_i8_string_const(char* p) {
    (void)p; // not used in design stage stub
    return 0;
}

// Note: Additional array/map stubs intentionally omitted to avoid symbol
// clashes with the full NyKernel when linked together. Keep this file minimal
// (console only) for print canaries.
