# Potentially Uninitialized Pointer
This rule checks if a pointer has been initialized before being passed to a
function. This might not always be an issue because it might be used as an
output.

## Triaging

* What is the pointer used for? Input or output?
* If input, what's being read from it?
* If output, is it initialized inside the function?
    * If not, does the function fill all of it?
    * If not, check where it's used. Does the program only read the parts filled
      by the function? We want to check if uninitialized data in the pointer are
      used or not.