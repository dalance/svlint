# Naming Convention Syntax Rules

Rules for checking against naming conventions are named with either the suffix
`_with_label` or one of these prefixes:

- `prefix_`
- `(lower|upper)camelcase_`
- `re_(forbidden|required)_`

Naming conventions are useful to help ensure consistency across components in
large projects.
A naming convention might be designed with several, sometimes competing, points
of view such as:

- Enable simple identification of code's owner, e.g. "Prefix all module
  identifiers with `BlueTeam_` at the point of declaration".
  This makes it easy for the blue team to review their own code, without being
  distracted by other team's code.
  Not specific to SystemVerilog, i.e also applicable to VHDL.
- Enhance readability of netlists, e.g. "Prefix all module instances with `u_`,
  interface instances with `uin_`, and generate blocks with `l_`".
  This facilitates straightforward translation from a netlist identifier to its
  corresponding identifier in SystemVerilog.
  Not specific to SystemVerilog, i.e also applicable to VHDL.
- Enhance readability of code for integrators and reviewers, e.g. "Prefix all
  ports with `i_`, `o_`, or `b_` for inputs, outputs, and bi-directionals
  respectively".
  This allows a reader to glean important information about how ports and
  internal logic are connected without the need to scroll back-and-forth
  through a file and/or memorize the portlist.
- Add redundancy to capture design intent, e.g. "Suffix every signal which
  should infer a flip-flop with `_q`".
  By using conventional terminology (`d` for input, `q` for output) readers
  will be alerted to investigate any flip-flops without this prefix as the
  tools may not be treating the code as the original author intended.
  Some example suffixes include:
  - `_d`: Input to a flip-flop.
  - `_q`: Output from a flip-flop.
  - `_lat`: Output from a latch.
  - `_mem`: Memory model.
  - `_a`: Asynchronous signal.
  - `_n`: Active-low signal.
  - `_dp`, `_dn`: Differential positive/negative pair.
  - `_ana`: Analog signal.
  - `_55MHz`: A signal with a required operating frequency.
- On the above two points, prefixes are redundant re-statements of information
  which must is explicit in SystemVerilog semantics, and suffixes are redundant
  clarifications of information which can only be specified implictly in
  SystemVerilog.

The rules `re_forbidden_*` can also be used to restrict language features.
For example, if a project requires that interfaces must never be used, you can
enable the rule `re_forbidden_interface` and configure it to match all
identifier strings.
By forbidding all possible identifiers at the point of declaration, no
interfaces may be specified.
For example:
```toml
[option]
re_forbidden_interface = ".*"

[rules]
re_forbidden_interface = true
```
