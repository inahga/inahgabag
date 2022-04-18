#include <stdint.h>
#include <stdio.h>
#include <sys/resource.h>

void func(uint64_t n)
{
    uint64_t arr[n];
    printf("%ld\n", sizeof(arr));
    for (uint64_t i = 0; i < n; i++) {
        arr[i] = 0;
    }
    printf("%ld\n", arr[0]);
}

int main(int argc, char* argv[])
{
    func(UINT64_MAX - 1);

    struct rusage r_usage;
    getrusage(RUSAGE_SELF, &r_usage);
    printf("%ldkb\n", r_usage.ru_maxrss);

    return 0;
}
