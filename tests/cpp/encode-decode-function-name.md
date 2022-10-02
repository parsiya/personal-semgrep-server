# Encode/Decode in Function Name
Functions that perform encoding/decoding and convert a format to another are
ripe for exploitation.

## Triaging

* Study the incoming and outgoing formats. Look for format specific bugs.
* Is the function using a parser? Parsing is a very vulnerable operation.
* Where is the output stored? Look for buffer overflows.
    * It's usually a pointer passed as a parameter and sometimes the return value.
    * Is the output buffer big enough for all inputs?
* How is the input passed to the function? Look for buffer over-reads.
    * Does the format have a null terminator? Is the parser/function parsing it correctly and detects the end?
    * If not, is the parser aware of the length of the input buffer? Does it read past it?
* Check where the output is going. If it ends up at a different parser, you have found another hot spot.

## References

* https://github.com/googleprojectzero/weggli/blob/main/README.md - search for `Encoding/Conversion functions`
* https://parsiya.net/blog/2022-04-07-code-review-hot-spots-with-semgrep/#function-with-encode-and-decode-in-their-names
