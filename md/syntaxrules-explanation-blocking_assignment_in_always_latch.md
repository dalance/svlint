Mixed blocking and non-blocking assignments under `always_latch` processes can
be difficult to read, and in the worst cases may lead to mismatches between
simulation and synthesis.

```systemverilog
always_latch
  if (load)
    q_blocking = getD();

always_latch
  if (load)
    q_nonblocking <= getD();
```

Those processes should be equivalent under synthesis, but not necessarily under
simulation where `getD()` has side effects.
For consistent results and readability, this rule prefers non-blocking
assignments in `always_latch` processes.

See also:
- **blocking_assignment_in_always_ff** - Useful companion rule.
- **non_blocking_assignment_in_always_comb** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 4.9.3 Blocking assignment
- 4.9.4 Non-blocking assignment
- 9.2.2.3 Latched logic `always_latch` procedure
- 9.4.2 Event control
- 10.4.1 Blocking procedural assignments
- 10.4.2 Nonblocking procedural assignments
- 16.5.1 Sampling
