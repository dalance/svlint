Functions defined within a module, interface, program, or package default to
being static, with all declared items being statically allocated.
These items shall be shared across all uses of the function executing
concurrently.
This causes a mismatch between simulation and synthesis.

Functions can be defined to use automatic storage by using the `automatic`
keyword as part of the function declaration, i.e. in simulation each use of a
function is allocated dynamically for each concurrent function call.
This behavior can be accurately inferred in synthesis.

The most relevant clauses of IEEE1800-2017 are:
  - 13.4.2 Static and automatic functions
