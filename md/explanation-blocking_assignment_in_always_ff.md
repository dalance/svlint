Simulator event ordering between blocking and non-blocking assignments
is undefined, so observed behavior simulator-dependent.
As all examples in IEEE1800-2017 show, `always_ff` should only contain
non-blocking assignments in order for sampling and variable evaluation
to operate in a defined order.

Specifically, `always_ff` constructs should not contain blocking assignments:
  - Blocking assignment operator, e.g. `foo = 123;`
  - Increment/decrement operators, e.g. `foo++;`, `foo--;`.

The most relevant clauses of IEEE1800-2017 are:
  - 9.2.2.4 Sequential logic always_ff procedure
  - 9.4.2 Event control
  - 10.4.1 Blocking procedural assignments
  - 10.4.2 Nonblocking procedural assignments
  - 16.5.1 Sampling
