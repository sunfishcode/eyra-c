#include <string.h>
#include <assert.h>

int main(void) {
    assert(strlen("hello") == 5);
    assert(strcmp(strrchr("hello", 'l'), "lo") == 0);
    return 0;
}
