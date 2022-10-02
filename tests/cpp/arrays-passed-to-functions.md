# Arrays Passed to Functions
If a pointer is passed to a function as input we need to see if it's vulnerable
to a buffer over-read. This happens if the function doesn't know the length of
input and keeps reading.

A common C/C++ design pattern is passing a pointer to the function as output. We
need to check for buffer overflows. This usually happens if the function does
not know the length of the output and keeps writing past its capacity.

Sometimes, these are still issues if lengths are passed. This happens when an
attacker can control the number passed as length to the function or when the
function doesn't check it.

## Triaging

* Is the pointer is passed as input or output?
* Is a length is passed to the function or is the function is aware of the
  length (e.g., hardcoded input).
    * Can the attacker control the value of length passed to the function? Yes? Report and investigate more.
* If no length is passed: investigate.
* If the function can write past the array's capacity, it's bad.
    * If input: Buffer over-read.
    * If output: Buffer overflow.