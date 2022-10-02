# Out of Bounds Array Access
Arrays (and most things) start from 0. `a[10]` is invalid for an array with 10
members.

This rules uses the [metavariable-comparison][met-com] field to do the math.
While, this is only detecting issues in a very specific scenario (array defined
and used in the same scope), it has good precision.

[met-com]: https://semgrep.dev/docs/writing-rules/rule-syntax/#metavariable-comparison