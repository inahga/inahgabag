#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// csort_compare_func returns a positive result if the first argument is greater
// than the second, a negative result if it is less, and 0 if they are equal.
typedef int (*csort_compare_func_t)(const void*, const void*);

static inline void csort_swap(void* a, void* b, size_t elem_size)
{
    uint8_t *ap = (uint8_t*)a, *bp = (uint8_t*)b;
    for (size_t i = 0; i < elem_size; i++) {
        uint8_t temp = ap[i];
        ap[i] = bp[i];
        bp[i] = temp;
    }
}

static void* csort_q_helper(void* arr, size_t arr_len, size_t elem_size, csort_compare_func_t compare, size_t low, size_t high)
{
    if (low < high) {
        size_t p = (low == 0) ? 0 : low;
        for (int j = low; j < high; j++) {
            if (compare(arr + j * elem_size, arr + high * elem_size) < 0) {
                csort_swap(arr + j * elem_size, arr + p * elem_size, elem_size);
                p++;
            }
        }
        csort_swap(arr + p * elem_size, arr + high * elem_size, elem_size);

        if (p > 0) {
            csort_q_helper(arr, arr_len, elem_size, compare, low, p - 1);
        }
        csort_q_helper(arr, arr_len, elem_size, compare, p + 1, high);
    }
    return arr;
}

void* csort_q(void* arr, size_t arr_len, size_t elem_size, csort_compare_func_t compare)
{
    return (arr_len <= 1) ? arr : csort_q_helper(arr, arr_len, elem_size, compare, 0, arr_len - 1);
}

bool csort_is_sorted(const void* arr, size_t arr_len, size_t elem_size, csort_compare_func_t compare)
{
    if (arr_len == 0) {
        return false;
    }
    for (int i = 0; i < arr_len - 1; i++) {
        if (compare(arr + i * elem_size, arr + ((i + 1) * elem_size)) > 0) {
            return false;
        }
    }
    return true;
}

#ifdef CSORT_TEST
#include <stdio.h>

int compare_int(const void* a, const void* b)
{
    return *(int*)a - *(int*)b;
}

static struct {
    char* name;
    int to_sort[10];
    int len;
} basic_sort_cases[] = {
    { "case 1", { 5, 4, 3, 2, 1 }, 5 },
    { "case 2", { 4, 5, 2, 1, 3 }, 5 },
    { "case 3", { 3, 1 }, 2 },
    { "case 4", { 1, 2 }, 2 },
    { "case 5", { 1 }, 1 },
    { "case 6", { 5, 4, 3, 2, 1, 7 }, 6 },
    { "case 7", { 2, 2, 2, 2, 2, 1 }, 6 },
    { "case 8", { 2, 2, 2, 2, 2, 2 }, 6 },
};

static struct {
    char* name;
    int to_check[10];
    int len;
    bool expected;
} basic_check_sort_cases[] = {
    { "no elements", { 0 }, 0, false },
    { "one element", { 1 }, 1, true },
    { "two sorted elements", { 0, 1 }, 2, true },
    { "two unsorted elements", { 1, 0 }, 2, false },
    { "three sorted elements", { 0, 1, 2 }, 3, true },
    { "three unsorted elements", { 2, 1, 2 }, 3, false },
    { "four sorted elements", { 0, 1, 2, 3 }, 4, true },
    { "four unsorted elements", { 2, 1, 2, 3 }, 4, false },
};

int main(int argc, char* argv[])
{
    for (int i = 0; i < sizeof(basic_check_sort_cases) / sizeof(basic_check_sort_cases[0]); i++) {
        printf("%s -> ", basic_check_sort_cases[i].name);

        bool result = csort_is_sorted(basic_check_sort_cases[i].to_check,
            basic_check_sort_cases[i].len, sizeof(int), compare_int);

        if (result != basic_check_sort_cases[i].expected) {
            puts("FAILED");
        } else {
            puts("PASSED");
        }
    }

    puts("");

    for (int i = 0; i < sizeof(basic_sort_cases) / sizeof(basic_sort_cases[0]); i++) {
        printf("%s -> ", basic_sort_cases[i].name);

        bool result = csort_is_sorted(
            csort_q(basic_sort_cases[i].to_sort, basic_sort_cases[i].len, sizeof(int), compare_int),
            basic_sort_cases[i].len,
            sizeof(int),
            compare_int);

        if (!result) {
            printf("FAILED, got {");
            for (int j = 0; j < basic_sort_cases[i].len; j++) {
                printf("%d, ", basic_sort_cases[i].to_sort[j]);
            }
            puts("}");
        } else {
            puts("PASSED");
        }
    }
}
#endif
