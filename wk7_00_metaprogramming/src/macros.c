#include <stdio.h>
#include <stdlib.h>

// [EXAMPLE: MAX macro]
// note: no types whoa
// can use preprocessor (e.g. clang -E) to see the expanded code
#define MAX(a, b) ((a) > (b) ? (a) : (b))

// BAD: missing parens... imagine we had MULTIPLY_BY_7(2 + 3) -> this would evaluate to 2 + 3 * 7 = 23
#define MULTIPLY_BY_7_BAD(x) x * 7 
// FIX: add parens
#define MULTIPLY_BY_7_GOOD(x) ((x) * 7)

// rule: parenthesise everything when writing macros

int main() {
    // simulating parametric polymophism
    int imax = MAX(1, 2);
    float fmax = MAX(1.0, 2.0);

    printf("imax: %d\n", imax);
    printf("fmax: %f\n", fmax);

    return 0;
}

// macros are fundamentally text substitution
// [EXAMPLE: using a function call in a macro]
int print_and_return(int x) {
    printf("x: %d\n", x);
    return x;
}
void macro_with_function() {
    int x = 5;
    int y = 10;
    // expands to: int z = ((print_and_return(x)) > (print_and_return(y)) ? (print_and_return(x)) : (print_and_return(y));
    int z = MAX(print_and_return(x), print_and_return(y));
    printf("z: %d\n", z);
    // output:
    // x: 5
    // x: 10
    // x: 10 -- since the function is called twice in the macro
    // z: 10
}

// [EXAMPLE: using a macro for syntactic sugar]
#define UP_TO(type, i, n) for (type (i) = 0; (i) < (n); (i)++)
void macro_for_syntactic_sugar() {
    UP_TO(int, i, 10) {
        printf("i: %d\n", i);
    }
    // output: i: 0, i: 1, ..., i: 9
}

// [EXAMPLE: using a macro for generating code -- polymorphism]
#define DeclareSort(prefix, type) \
static int \
_DeclareSort_ ## prefix ## _Compare(const void *a, const void *b) { \
    const type *aa; const type *bb; \
    aa = a; bb = b; \
    if (*aa < *bb) return -1; \
    else if (*bb < *aa) return 1; \
    else return 0; \
} \
\
void \
prefix ## _sort(type *a, int n) { \
    qsort(a, n, sizeof(type), _DeclareSort_ ## prefix ## _Compare); \
}

// generates sort functions for int, float, double, char
DeclareSort(int, int)
DeclareSort(float, float)
DeclareSort(double, double)
DeclareSort(char, char)

void using_declare_sort() {
    int a[] = {3, 2, 1};
    int n = 3;
    int_sort(a, n);
    for (int i = 0; i < n; i++) {
        printf("a[%d]: %d\n", i, a[i]);
    }
    // output: a[0]: 1, a[1]: 2, a[2]: 3
}
