# Operations on Typed Data
Bart Massey

The module `stackque` in this crate provides a "stack
queue", a structure supporting adding to the back of the
queue and popping from the back or front.

This data structure is both heterogenous and statically
typed.  See the crate docs for an explanation and example.

Trait implementations here are such that only legitimate
`StackQue`s can be pushed onto: this is a consequence of
the private field of `StackQue` and the orphan rule.

This is all a bad idea.
