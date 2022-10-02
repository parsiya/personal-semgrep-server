# Insecure memcpy Usage
`memcpy(destination, source, size);` copies `size` bytes from `source` to
`destination`. It doesn't do any length checks. We must check if we're getting a
buffer overflow or buffer over-read.

Using `sizeof` incorrectly in the third parameter is a common mistake. Such as:

1. Using `sizeof(source)`.
2. Using `sizeof(*ptr)` instead of the actual underlying object. Also covered in another rule.
3. Not calculating the size correctly.

## Example

```cpp
    char c[6];
    char name[] = "Parsia";
    // ruleid: memcpy-insecure-use
    memcpy(c, name, sizeof(name));
```

We're copying the a 7-byte string (remember the null-terminator) to a 6 byte
buffer. We're overwriting one byte.

## Triaging

* Check the third parameter which is the number of bytes to copy against the
  size of both source and destination (note it's in bytes).
    * If larger than size of source: Buffer over-read.
    * If larger than size of source: Buffer overflow.

## References

* https://dustri.org/b/playing-with-weggli.html - search for `sizeof(ptr)`
* https://parsiya.net/blog/2022-04-07-code-review-hot-spots-with-semgrep/#sizeofptr-in-c