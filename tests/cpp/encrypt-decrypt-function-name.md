# Encrypt/Decrypt in Function Name
Functions that perform encryption/decryption are great hot spots for
cryptographic issues.

## Triaging

* Study the encryption. Look for cryptographic bugs.
* If it's a homebrew cryptographic protocol, RED ALERT!
* What is it encrypting? If it's non-sensitive, you might want to ignore this function.
* Where is the output stored? Look for buffer overflows.
    * It's usually a pointer passed as a parameter and sometimes the return value.
    * Is the output buffer big enough for all inputs?
* How is the input passed to the function? Look for buffer over-reads.
    * Does the format have a null terminator? Is the parser/function parsing it correctly and detects the end?
    * If not, is the parser aware of the length of the input buffer? Does it read past it?

## References

* https://parsiya.net/blog/2022-04-07-code-review-hot-spots-with-semgrep/#function-with-encode-and-decode-in-their-names
