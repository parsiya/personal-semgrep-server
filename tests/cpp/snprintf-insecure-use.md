# Insecure snprintf Usage
`int snprintf(char* s, size_t n, const char * format, ...);`

1. `snprintf` writes the format string up to `n-1` characters to `s`.
2. `snprintf` adds the null-terminator automatically.
3. The return value is the number of characters written **if n had been
   sufficiently large** to copy the whole format string.
4. To check if the entire string has been written, check if `ret < n`.

## Examples

### First Example

```cpp
    char myname[5];
    // ruleid: snprintf-insecure-use
    int ret = snprintf(myname, 10, "0123456789");
```

There are two issues here:

1. We're writing `10` character into `myname` which is 5 bytes. That's a buffer overflow.
2. Even if `myname` was big enough, we're only copying 9 characters from the
   original string because the null-terminator is added by `snprintf` and it
   only copies up to `n-1` characters.

### Second Example

```cpp
    char name2[5] = "1234";
    // ruleid: snprintf-insecure-use
    int len = snprintf(0, 0, "My name is: %s", myname);
    char *p = (char*) malloc(len);
    snprintf(p, len, "My name is: %s", myname);
```

`snprintf(NULL, 0, ...);` (the first parameter can also be anything in this
case) is a popular way to figure out the length of a format string at runtime.
The manual says `If n is zero, nothing is written, and s may be a null pointer`.

However, we're not accounting for the extra null-terminator added by `snprintf`
in `malloc(len)`. As a result, the last character of the string is not written
to `p`. If we print the value of `p`, we'll only see `123`.

## Triaging

* Is the size of `s` (the destination) big enough for all final strings?
    * This is specially important if some parts of the string are user-controlled.
* What are we doing with the value of `snprintf`? Are we considering the automatic null-terminator?

## References

* https://dustri.org/b/playing-with-weggli.html - search for `snprintf`
* https://cplusplus.com/reference/cstdio/snprintf/