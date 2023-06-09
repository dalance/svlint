Simulator event ordering between concurrent action blocks is undefined, so
observed behavior is simulator-dependent.
While assertions with side-effects may appear to work on a single-threaded
simulator, they may interact in unexpected ways on a multi-threaded simulator.
On encountering side-effect code in action blocks, a simulator can either
implement inter-thread locking (with a hit to performance) or allow a
race-condition to occur, neither of which are desirable.

Specifically, action blocks should not contain blocking assignments:
- Blocking assignment operator, e.g. `foo = 123;`
- Increment/decrement operators, e.g. `foo++;`, `foo--;`.
- Sequential IO, e.g. `$display();`, `$write();`.
  The full list of IO system tasks and system functions is given on page 624 of
  IEEE1800-2017.

See also:
- **non_blocking_assignment_in_always_comb** - Useful companion rule.
- **blocking_assignment_in_always_ff** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 15.5.4 Event sequencing: wait\_order()
- 16 Assertions
- 21 Input/output system tasks and system functions
