/* Native backend smoke fixture: shape of code emitted for examples/control_flow.cling. */
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <inttypes.h>
static void cling_print_int(int64_t x) { printf("%" PRId64 "\n", x); }
void cling_print_bool(bool x) { puts(x ? "true" : "false"); }
void cling_print_string(const char *x) { puts(x); }
int64_t cling_fn_sum_to(int64_t limit);
void cling_fn_main(void);
int64_t cling_fn_sum_to(int64_t limit) {
    int64_t total = INT64_C(0);
    int64_t current = INT64_C(1);
    while ((current <= limit)) {
        total = (total + current);
        current = (current + INT64_C(1));
    }
    return total;
}
void cling_fn_main(void) {
    int64_t answer = cling_fn_sum_to(INT64_C(10));
    cling_print_int(answer);
    if (answer == INT64_C(55)) {
        cling_print_string("CIL execution succeeded");
    } else {
        cling_print_string("unexpected result");
    }
    return;
}
int main(void) { cling_fn_main(); return 0; }
