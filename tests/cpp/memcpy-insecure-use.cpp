#include <cstring>

int main() {

    char c[6];
    char name[] = "Parsia";
    // ruleid: memcpy-insecure-use
    memcpy(c, name, sizeof(name));
}