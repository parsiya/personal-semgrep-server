#include <iostream>

int main() {

    char myname[5];
    // ruleid: snprintf-insecure-use
    int ret = snprintf(myname, 10, "0123456789");

    char name2[5] = "1234";
    // ruleid: snprintf-insecure-use
    int len = snprintf(0, 0, "My name is: %s", name2);
    char *p = (char*) malloc(len);
    snprintf(p, len, "My name is: %s", name2);

    // prints 123
    std::cout << p;

    return 0;
}