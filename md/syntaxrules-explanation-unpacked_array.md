This rule forbids unpacked array declarations.

Unpacked arrays are not guaranteed to be represented as contiguous memory, and
can cause issues with synthesis tools, especially with how multidimensional
arrays are synthesized. For example, a synthesis tool might synthesize out
unused memory locations of an unpacked array which is not the intended behavior.

Additionally, packed arrays allow the user to intuitively index and slice the
array and apply bitwise operations.

The most relevant clauses of IEEE1800-2017 are:
- 7.4 Packed and unpacked arrays
