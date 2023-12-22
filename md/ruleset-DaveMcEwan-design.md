
This ruleset requires a [defensive coding](https://en.wikipedia.org/wiki/Defensive_programming)
style where artistic licence is restricted in favour of consistency, clarity,
and ease of review.
As indicated by the ruleset's name, this is the svlint configuration preferred
by the user [DaveMcEwan](https://github.com/DaveMcEwan) for synthesisable
design code.


### Motivation

The term "consistency" is used to mean both the cosmetic appearance and the
semantic interpretation across various tools.
Engineering peers and/or employers surely value creativity in addressing the
difficult challenges of logic design far more than creativity in formatting,
i.e. the value of personal taste (about where to place whitespace and how to
phrase logical constructions) is considerably lower than the value of
consistency.

A reviewer is a person who reads code then decisively states whether, in
their opinion, the code achieves the necessary.
Given the potentially huge costs incurred by mistakes in silicon development,
sensible reviewers should err on the side of caution and refrain from declaring
the code "finished" until they are absolutely certain that it is - This can be
frustrating for developers who are keen to move onto something else, so it's in
a developer's interest to make the process as easy as possible for reviewers.

There are several ways that this ruleset aims to reduce the mental burden on
reviewers:
1. Present code in a consistent format, i.e. using *explicitly specified*
  conventions for style/whitespace and naming/identifiers.
2. Assure the reviewer that common assumptions hold true, e.g. "*all* constants
  are 2-state".
3. Minimise scope of objects, i.e. how much information a reader must keep in
  mind while reading a section of code.
4. Encourage canonicalisation.
  The infamous [Zen of Python](https://peps.python.org/pep-0020/) phrases this
  concept as "There should be one-- and preferably only one --obvious way to do
  it."
  By enforcing a strict style, readers can read and comprehend a large body of
  code quickly and accurately.
5. Above all else, ensure that the intention is crystal clear.
  An author should demonstrate (to their readers) that they have considered the
  precise meaning of what they wrote, thus giving little room for
  mis-interpretation by blurry-eyed readers or tools operating in the grey
  areas of the SystemVerilog LRM.
  One example is requiring every `case` to have a `default` arm.
  Another prominent example is in the rules `sequential block_in_*`, i.e.
  specifying and implementing purely combinatorial logic is clearer with purely
  combinatorial code (rather than procedures).

Through using rules which align with those 5 aims, reviewers are
free to concentrate on aspects which require high-level thought such as
"Is this an efficient design?", instead of less interesting things like "Will
this code be synthesized as I expect?".
This ruleset builds upon **ruleset-style** for cosmetic consistency,
**ruleset-designintent** for consistent intepretation across tools, and
**ruleset-DaveMcEwan-designnaming** for naming conventions.


### Style (Whitespace) Consistency

Style conventions also help a human reader to quickly and efficiently
comprehend large bodies of code.
Indeed, that is exactly what a reader wants to do when they're working with
code written by other people, often complete strangers.
The reader simply wishes to open the file, extract the necessary information,
close the file, and get on with their life.
Unlike mechanical tools, people process code visually (by translating their
view of the screen into a mental model) and any noise which obscures the useful
information will require extra mental effort to process.
When code is written with consistent and regular whitespace, the important
details like operators and identifiers are easily extracted.
In contrast, when little attention is paid to indentation or spaces around
keywords, operators, or identifers, the readers must waste their energy
performing a mental noise reduction.
Therefore, the main motivation behind this ruleset is to avoid visual noise.

Two notable style conventions help with a change-review process, i.e. comparing
multiple versions of a file, rather than reading one version:

- Line length limited to a fixed number of characters, usually 80.
  - Excessively long lines may indicate problems with a program's logic.
  - Excessively long lines prevent viewing differences side-by-side.
  - Side-by-side reading is awkward when sideways scrolling is involved.
  - Code which is printed on paper cannot be scrolled sideways, and soft-wrap
    alternatives interrupt indentation.
- Trailing whitespace is forbidden.
  - Changes to trailing whitespace are not usually visible to human readers,
    but are found by version control tools.
  - Editors are often configured to remove trailing whitespace, resulting in
    unnecessary differences.
  - Git, a popular version control tool will (by default) warn against trailing
    whitespace with prominent markers specifically because of the unnecessary
    noise introduced to a repository's history.
  - Closely related, is the obfuscation of statements by using whitespace to
    push a semicolon off the RHS of the screen, thus misleading the viewer into
    thinking that the next line is a continuation instead of a new statement.

These conventions help give a consistent view over different ways of viewing
files which include the writer's text editor (Vim, VSCode, Emacs, etc.),
consumer's text editor, reviewer's web-based tools (GitHub, BitBucket, GitLab,
etc.), printed material (e.g. via PDF), and logfiles from CI/CD tools (GitHub
Actions, Bamboo, Jenkins, etc).

```toml
option.textwidth = 80
textrules.style_textwidth = true
textrules.style_semicolon = true
```


#### Indentation

An indent of 2 spaces, not tabs, is chosen.
For better or worse, contemporary computer language styles have moved
decisively away from using tabs for indentation.
The most likely reason behind this is that tab display width is configurable
so tab indentations are shown differently, depending on the reader's personal
configuration.
```toml
option.indent = 2
syntaxrules.tab_character = true
syntaxrules.style_indent = true
```
Note that the **style_indent** rule does not check that indentations are the
correct level - only that the indentation is an integer multiple of 2 spaces.

In SystemVerilog, most of the language is independent of whitespace characters,
so readers are (hopefully) aware that they should be careful not to interpret
indentation with semantic meaning, but its human nature to do so.
Therefore, author care is still required to use the correct indent, i.e.
**style_indent** only points out indentations which are obviously wrong, but
does not understand the logical semantics of any SystemVerilog constructs.

```systemverilog
always_comb begin
  x = 0;
  y = 123;

  if (a)
    x = 1;
  else
    x = 2;
    y = 666;

  z = y + x;
end
```
Above is a simple demonstration of how the human eye can be misled in ways
that mechanical tools like compilers are immune to.
Depending on the value of expression `a`, the variable `z` takes the value
either `667` or `668`, but never `124`.
To mitigate the risk of confusion around multi-line conditional statements and
loops, two further rules are enabled to check that either `begin`/`end`
keyword delimiters are used, or the statement is moved to the same line as the
condition.
```toml
syntaxrules.multiline_if_begin = true
syntaxrules.multiline_for_begin = true
```


#### Indentation Preprocessor Considerations

A potential source of confusion is in the use of the preprocessor to
accidentally introduce whitespace.
In these examples, a dot character (`.`) is used to visually present a space
character where it's important.
```systemverilog
`ifdef A
..foo();
`endif.// A space between the "endif" directive and the line comment
```
If `A` is defined, the above example will be emitted from the preprocessor as
this text:
```systemverilog
foo();
.// A space between the "endif" directive and the line comment
```
The line after `foo()` begins with a 1 space, which violates the
**style_indent** check.
Note that the violation occurs even if `A` is not defined.

To further confuse things, the following example will *not* cause a violation
when `A` is undefined!
```systemverilog
.`ifdef A
..foo();
.`endif.// A space between the "endif" directive and the line comment
```
The 1 space on the `ifdef` line is joined to the 1 space after `endif` to make
a line with a 2 space indent like this:
```systemverilog
..// A space between the "endif" directive and the line comment
```

Confusing situations like these arise from the fact that SystemVerilog is a
combination of two languages;

1. A text processing language (defining the preprocessor) in specified
  informally in IEEE1800-2017 Clause 22 among other compiler directives.
2. The rest of SystemVerilog syntax is formally called `source_text`, is
  specified formally in IEEE1800-2017 Annex A.

Svlint syntax rules operate on the `source_text` part of SystemVerilog, i.e.
after the preprocessor has been applied.
As with other languages with similar text-based templating features, most
notably C, use of the preprocessor is discouraged except where absolutely
necessary.
To avoid confusion with preprocessor, here are two recommendations:

1. Don't indent compiler directives, especially preprocessor statements
  containing any `source_text`.
2. Don't put any spaces between compiler directives and comments on the same
   line.

These are some examples of confusion-ridden style, not recommended.
```systemverilog
`define Z // Space then comment
`ifdef A // Space then comment
..`ifdef B// Indented ifdef
....foo(); // Indent of source_text mixed with preprocessor
..`endif// Indented endif
`endif // Space then comment
```
The above examples can be reformed like this:
```systemverilog
`define Z// No space then comment
`ifdef A// No space then comment
`ifdef B
..foo();
`endif// B
`endif// A
```

Where no `source_text` is contained in the ifdef block, i.e. only preprocessor
definitions, these may be indented without causing confusion:
```systemverilog
`ifdef A
..`ifdef B
....`define Z
..`endif// B
`endif// A
```
For clarification, when both `A` and `B` are defined, the above block will be
emitted from the svlint preprocessor as shown below.
```systemverilog
`define Z
..// B
// A
```

One method which can help catch unintended whitespace, both from the
preprocessor and written by hand, is to forbid trailing spaces, i.e. space
characters followed immediately by a newline.
```toml
syntaxrules.style_trailingwhitespace = true
```

Problems around indented preprocessor directives must be caught before svlint's
preprocessor stage.
```toml
textrules.style_directives = true
```


#### Operators and Keywords

Consistent use of whitespace around operators and keywords makes it easier to
read expressions quickly and accurately.
```toml
syntaxrules.style_operator_arithmetic = true
syntaxrules.style_operator_boolean = true
syntaxrules.style_operator_integer = true
syntaxrules.style_operator_unary = true

syntaxrules.style_keyword_0or1space = true
syntaxrules.style_keyword_0space = true
syntaxrules.style_keyword_1or2space = true
syntaxrules.style_keyword_1space = true
syntaxrules.style_keyword_construct = true
syntaxrules.style_keyword_datatype = false # Overly restrictive.
syntaxrules.style_keyword_end = true
syntaxrules.style_keyword_maybelabel = true
syntaxrules.style_keyword_new = true
syntaxrules.style_keyword_newline = true
```


#### Comma-Separated Lists

SystemVerilog code has many uses for comma-separated lists of items specified
in IEEE1800-2017 Annex A.
Most of these uses can be found by searching for BNF symbols containing the
string `list_of_`, but uses are specified in BNF expressions for other symbols,
e.g. `modport_declaration` and `data_type`.

Without careful review processes in place, the large variety semantics and
syntax surrounding comma-separated lists can easily lead authors writing in a
large variety of styles.
To make matters worse, the use of comma-separated lists varies is common in
other languages - but with significant subtle differences.
For example, while Python and Rust allow an extra comma after the last argument
in a function call, C and SystemVerilog do not allow this.

The desire for consistent formatting and readability provides motivation for a
simple rule which can be easily remembered by authors.
The most common style in functional programming language Haskell provides
inspiration for such a rule:
"Every comma must be followed by exactly one space".
```toml
syntaxrules.style_commaleading = true
```

This rule leads to the comma-leading style which, although perhaps unfamiliar
to authors with a background in C or Python, has a number of advantages.
- The rule is extremely simple, especially in comparison to the multitude of
  rules requried to format comma-trailing lists consistently.
- A comma character is visually similar to bullet-point.
- When changing code over time, it's more common to add items to the end of a
  list than the beginning.
  This means that comma-leading style often leads to diffs which are easier to
  review.
  Closely related to this is that comma-leading style makes it less likely to
  introduce an extra comma at the end of a list (which would be a syntax
  error).
- Multi-dimensional arrays are easier to read, because it's natural to put a
  line without elements (only the closing `}`) between elements of the
  more-significant axis.
- Comma is visually similar to bulletpoint (a common symbol for introducing an
  item of a list in prose).
- Comma-leading style can be said to be more closely aligned with BNF
  specification, e.g.
  `list_of_genvar_identifiers ::= genvar_identifier { , genvar_identifier }`.
  This is reflected by how sv-parser attaches `Comment` nodes (which contain
  whitespace) to the RHS of comma symbols.

For some examples, please see the explanation of the **style_commaleading**
rule.

Additionally, `eventlist_or` mandates the use of `,` (comma) as the separator
in `always_ff` sensitivity lists only for consistency and readabilty.
```toml
syntaxrules.eventlist_or = true
```


### Tool Consistency

Rules that forbid suspicious constructions, i.e. ways of specifying hardware
that are legal according to the LRM, but may express their intention unclearly.

The following subset is designed to detect potential mismatches between
simulation and synthesis.
These rules don't intentionally interact to provide additional properties.

```toml
# Common to **ruleset-simsynth** (a subset of **ruleset-designintent**).
syntaxrules.blocking_assignment_in_always_ff = true
syntaxrules.blocking_assignment_in_always_latch = true
syntaxrules.non_blocking_assignment_in_always_comb = true
syntaxrules.case_default = true
syntaxrules.enum_with_type = true
syntaxrules.function_with_automatic = true
syntaxrules.keyword_forbidden_priority = true
syntaxrules.keyword_forbidden_unique = true
syntaxrules.keyword_forbidden_unique0 = true
syntaxrules.general_always_no_edge = true
syntaxrules.operator_case_equality = true
syntaxrules.procedural_continuous_assignment = true

# Common to **ruleset-designintent**.
syntaxrules.action_block_with_side_effect = true
syntaxrules.default_nettype_none = true
syntaxrules.function_same_as_system_function = true
syntaxrules.keyword_forbidden_always = true
syntaxrules.keyword_forbidden_wire_reg = true
syntaxrules.module_nonansi_forbidden = true
```

Generally, elaboration-time constants (`parameter`, `localparam`) should be
2-state types and declared with a default value.
Additionally, where the context defines that `parameter` is an alias for
`localparam`, authors should demonstate that they understand the constant
cannot be overriden by using the `localparam` keyword.
```toml
syntaxrules.localparam_type_twostate = true
syntaxrules.parameter_type_twostate = true
syntaxrules.localparam_explicit_type = true
syntaxrules.parameter_explicit_type = true
syntaxrules.parameter_default_value = true
syntaxrules.parameter_in_generate = true
syntaxrules.parameter_in_package = true
```

Genvars, which are also elaboration-time constants, should be declared within
generate `for` loops to reduce their scope.
This allows readers to be confident that they can see all of the relevant
information about a genvar in one place, i.e. declaration and usage.
A notable advantage of declaring genvars in each generate loop is that authors
are encouraged to give them suitably descriptive names.
Rules on the use of the `generate` and `endgenerate` keywords is similarly
subjective, but this ruleset forbids their use because readers should be aware
that all `case`, `for`, and `if` blocks outside of assignment processes are
generate blocks.
According to the LRM, use of `generate` and `endgenerate` is optional with no
semantic difference to not using them.
However, at least one (older) FPGA synthesis tool is prone to crashing when
generate blocks are used outside explicit generate regions.
```toml
syntaxrules.genvar_declaration_in_loop = true
syntaxrules.genvar_declaration_out_loop = false
syntaxrules.keyword_forbidden_generate = false
syntaxrules.keyword_required_generate = true
```

Rules in the following subset combine to provide an important property for the
robust design of synthesisable hardware - that you can easily draw a schematic
of what the synthesis result should look like.
The two rules of thumb are to always fully specify decision logic, and never
use sequential models for (what will be synthesized to) parallel logic.
A sequential block is one delimited by `begin`/`end` keywords.
```toml
syntaxrules.explicit_case_default = true
syntaxrules.explicit_if_else = true
syntaxrules.loop_statement_in_always_comb = true
syntaxrules.loop_statement_in_always_ff = true
syntaxrules.loop_statement_in_always_latch = true
syntaxrules.sequential_block_in_always_comb = true
syntaxrules.sequential_block_in_always_ff = true
syntaxrules.sequential_block_in_always_latch = true
```

Where sequential modelling of parallel logic is an unavoidable pragmatic
approach, `begin` and `end` keywords should be used carefully and with proper
indentation.

The semantics around port declarations are, perhaps, unintuitive but were
designed for backward compliance with Verilog (IEEE1364-1995).
The below subset ensures that port declarations clearly convey important
information about the direction and update mechanism of each signal port.
```toml
syntaxrules.inout_with_tri = true
syntaxrules.input_with_var = true
syntaxrules.output_with_var = true
syntaxrules.interface_port_with_modport = true
```

Some kinds of SystemVerilog objects should never be declared in synthesizable
code, so regex rules can be used to forbid declarations with *any* name.

```toml
option.re_forbidden_checker = ".*"
syntaxrules.re_forbidden_checker = true
option.re_forbidden_class = ".*"
syntaxrules.re_forbidden_class = true
option.re_forbidden_port_ref = ".*"
syntaxrules.re_forbidden_port_ref = true
option.re_forbidden_property = ".*"
syntaxrules.re_forbidden_property = true
option.re_forbidden_sequence = ".*"
syntaxrules.re_forbidden_sequence = true
option.re_forbidden_task = ".*"
syntaxrules.re_forbidden_task = true
```


### Naming Conventions

These rules around naming conventions are also available in the specialized
ruleset **ruleset-DaveMcEwan-designnaming**.


#### Filesystem and Logical Hierarchy

In synthesisable design code, there are three main types of description
(package, module, and interface), which should normally be kept in separate
files for each description.
A straightforward way to manage these in a filesystem is to have the filename
match the identifier of the description inside, i.e. `myModule.sv` should
contain only the module named `myModule`, and `pkg1.sv` should contain
only the package named `pkg1`.
Note, this ruleset does not perform checks on file names.

Additionally, it is useful for the identifiers used in code to be immediately
obvious which type of description they refer to.
References to packages are always obvious because of the scope resolution
operator `::` (see IEEE1800-2017 clause 26.3).
However, interfaces and modules use identical instantiation syntax which makes
it difficult to easily identify if an instance refers to a module or interface
(see the definitions of `module_instantiation` and `interface_instantiation` in
IEEE1800-2017 Annex A.4 Instantiations).
A good naming scheme should make these easy to distinguish without introducing
too much visual noise.

The approach in this ruleset is similar to that in typical Haskell - the case
of the first letter of the identifier signifies what it refers to.
To begin, modules should have the first letter as Uppercase - Modules are the
most common thing to instance, so they should use the minimum number of
characters to avoid visual noise.
Next, packages are referred to more often than interfaces, so these are
distinguished by their first letter as lowercase.
Interface identifiers are usually used less often in a module than package
identifies - for example constants and functions in a package might be used
in the declarations and assignments of many signals, but interface identifiers
are only used for instantiations.
To distinguish instantiations of interfaces from modules, interface identifiers
should be prefixed with `ifc_`.
There are no restrictions on the rest of an interface identifier (everything
after the `ifc_` prefix) or modport or variable identifiers within an
interface declaration.

```toml
syntaxrules.lowercamelcase_package = true
syntaxrules.uppercamelcase_module = true
option.prefix_interface = "ifc_"
syntaxrules.prefix_interface = true
```

The above rules help readers to navigate a filesystem to find the right source
files containing packages, modules, and interfaces.
Another common situation where it is necessary to distinguish between these is
in examining tool output such as netlists and waveforms.
In these scenarios, naming conventions on hierarchical nodes can help engineers
distinguish between modules, interfaces, and generate blocks.
Although the LRM is clear about the implict naming of unlabelled generate
blocks (see IEEE1800-2017 clause 27.6), using a well-named label provides some
clarification about the intention behind that logic.
Instance identifiers of both modules and interfaces should be prefixed with
`u_`, whereas generate block labels should be prefixed with `l_`.

```toml
option.prefix_instance = "u_"
syntaxrules.prefix_instance = true
option.prefix_label = "l_"
syntaxrules.generate_case_with_label = true
syntaxrules.generate_for_with_label = true
syntaxrules.generate_if_with_label = true
```

A further convention, not checked by this ruleset, is to use Uppercase vs
lowercase for the first letter after the `u_` prefix to distinguish between
instances of modules vs interfaces.
For example, a module instance looks like `u_Foo` and an interface instance
looks like `u_foo`.
This makes it easier in navigate hierarchy in, for example, waveform viewers.

The above rules around filesystem and logical hierarchy are demonstrated in the
example below:

```systemverilog
/* filename: path/to/usb.sv */
package usb;                                    // package declaration
...
endpackage

////////////////////////////////////////////////////////////////////////////////

/* filename: path/to/ifc_fifo.sv */
interface ifc_fifo;                             // interface declaration
...
endinterface

////////////////////////////////////////////////////////////////////////////////

/* filename: path/to/UsbRx.sv */
module UsbRx                                    // module declaration
  ( ...
  , ifc_fifo.read                     rdData    // interface port
  , output var logic [usb::PID_W-1:0] o_pid     // package reference
  );
...
  ifc_fifo u_packer;                            // interface instance
...
  Fifo u_Queue ( ... );                         // module instance
...
  if (FOO) begin: l_foo                         // generate block
...
  end: l_foo
endmodule
```


#### Ports and Direction

A distinctive feature of this naming convention is that all ports have prefix
denoting their direction: `i_`, `o_`, or `b_` for inputs, outputs, and
bi-directionals respectively.
This technique adds useful redundancy for readers/reviewers, which is
especially useful for very large modules.
By analogy, this is similar to the use of arrowheads in a electical schematic -
sure, arrowheads aren't essential but they can be very useful in helping
readers understand the flow of information!
There are several ways in which this naming convention technique adds value:

- Visually highlight port connections.
  Internal signals should not have any prefix but ports should, so the prefixes
  make port connections stand out clearly in code reviews.
- Provide assurance that inputs are not accidentally connected to the wrong
  thing.
  For example, an input that should be connected directly to a DFF, but must
  not feed into any combinational logic.
- Clarify that the direction is the one which the author intended.
  For example, in `output var logic o_foo`, the direction is written twice
  (`output` keyword, then `o_` prefix).
  It isn't foolproof, but a mismatch such as `input o_foo` indicates a
  copy-pasta error which might otherwise be easily overlooked, especially
  because the rules about assignments to ports are not intuitive or
  consistently implemented across tools.
- In assertions (or other testbench code) where signals are connected via
  `bind`, assure readers that only inputs are `assume`d and that outputs are
  only `assert`d or `cover`ed (but outputs aren't `assume`d).
- In long files which don't fit on one screen, the reader doesn't need to
  scroll back-and-forth or memorize the portlist to determine which parts are
  connected to the boundary and which are purely internal.
- When viewing a colleague's waveforms in a viewer like
  [GTKWave](https://github.com/gtkwave/gtkwave), prefixes clearly show whether
  each wave is for a port or a module-internal signal.
  A common task in examining waveforms is to search for only inputs or only
  outputs, made easy by searching for `i_*` or `o_*`.
- In complex synthesis flows, ports are often considered more stable
  [API points](https://davemcewan.github.io/SemVerSoC/)
  than internal signals, so this naming convention highlights to script owners
  if they are using unstable points which might require more script
  maintenance.

Interface ports do not benefit in the same ways because `modport`s can
change the component signals' direction.
The only benefit which interface port prefixes would give is to highlight
connections to the module boundary vs internal interfaces.
This reason is deemed too weak to require the use of another prefix.

```toml
option.prefix_inout = "b_"
syntaxrules.prefix_inout = true
option.prefix_input = "i_"
syntaxrules.prefix_input = true
option.prefix_output = "o_"
syntaxrules.prefix_output = true
option.re_required_port_interface = "^[a-z]+[a-zA-Z0-9_]*$"
syntaxrules.re_required_port_interface = true
```

This illustrative example shows some of the common features addressed by this
convention:

```systemveilog
module Fifo
  ( input  var logic i_data   // Same name `data` used in both directions.
  , output var logic o_data

  , input  var logic i_push
  , input  var logic o_full   // Copy/paste error, now might be caught.

  , input  var logic i_pop
  , output var logic o_empty  // This looks better.

  , ifc_fifo.debug   dbg      // Interface port has no prefix.
  );

  ...

  RAMBLOCK u_ram
    ( RDATA   (o_data)        // Connected directly to port.
    , WDATA   (dataRetime_q)  // Connected to an internal DFF.
    );

  always_comb i_data = foo;   // Assignment to input looks wrong.
  always_comb o_full = i_pop && foo; // Feedthrough logic may be wrong.

  always_comb dbg.foo = foo;  // Direction of connection to interface
                              // port is not clear, regardless of the
                              // lack of prefix on `dbg`.
endmodule
```


#### Elaboration-Time Constants

Functions may have a short lowercase name where the functionality is "obvious
and frequently used" but not provided in IEEE1800-2017 clause 20 Utility
system tasks and system functions, e.g. `flog2`.
More complex functions should have an `f_` prefix which can help readers
navigate a large codebase using text-based tools like `grep`.
Use good judgement to decide if a particular function is obvious and
frequently used.

Parameters, either overridable (`parameter`) or non-overridable (`localparam`)
must be fully UPPERCASE.
This helps clarify which portions of logic may be optimized during synthesis,
e.g. `assign foo = bar && MYCONST;` should be optimized to either
`assign foo = bar;` or `assign foo = 1'b0;`.

Generate loop variables (declared with `genvar`) should have lowercase names of
no more than 3 characters.
This rule aims to easily distinguish genvars from signals, allow the common
usage of single-character names like `i`, and prevent hard-to-read code with
long genvar names like `index_of element`.

```toml
option.re_required_function = "^([a-z]{1,1}[a-z0-9]{0,9}|f_[a-zA-Z0-9_]+)$"
syntaxrules.re_required_function = true
option.re_required_localparam = "^[A-Z]+[A-Z0-9_]*$"
syntaxrules.re_required_localparam = true
option.re_required_parameter = "^[A-Z]+[A-Z0-9_]*$"
syntaxrules.re_required_parameter = true
option.re_required_genvar = "^[a-z]{1,3}$"
syntaxrules.re_required_genvar = true
```

The above rules are shown in this example:

```systemveilog
module Buffer
  #(parameter int WIDTH = 8
  , localparam int N_STAGE = 5
  )
  ( ... );

  localparam bit MYBOOL = 1'b0;

  // Check that element of parameter `MYARRAY` satisfy constraints.
  function automatic bit [N_ITEM-1:0] f_paramcheck_MYARRAY ();
    for (int i=0; i < N_ITEM; i++)
      f_paramcheck_MYARRAY[i] =
        &{(0 <= MYARRAY[i])
        , (MYARRAY[i] < DEPTH)
        , (MYARRAY[i] != 2)
        , IS_ODD[i] ? (MYARRAY[i] % 2) : 1'b1
        };
  endfunction

  function automatic int unsigned flog2 (int x);
    ...
  endfunction

  for (genvar i = 0; i < N_ITEM; i++) begin: l_foo
    ...
  end: l_foo

  ...
endmodule
```


#### Variables

Finally, some elements of design intent can be clarified by adding some useful
redundancy in the form of suffixes on identifiers, e.g. "Every signal which
should infer the output of a flip-flop with `_q`".
By using conventional terminology (`d` for input, `q` for output) readers
will be alerted to investigate any flip-flops (in a netlist) without this
suffix as the tools may not be treating the code as the original author
intended.

Some common suffixes include:

- `_d`: Input to a flip-flop.
- `_q`: Output from a flip-flop.
- `_lat`: Output from a latch.
- `_mem`: Memory model.
- `_a`: Asynchronous signal.
- `_n`: Active-low signal.
- `_dp`, `_dn`: Differential positive/negative pair.
- `_ana`: Analog signal.
- `_55MHz`: A signal with a required operating frequency.

Throughout this ruleset, prefixes are (usefully) redundant re-statements of
information already defined in SystemVerilog semantics, whereas suffixes are
(usefully) redundant clarifications of information which can only be inferred.
Note, svlint does not perform semantic analysis, so there are no rules to
check for these conventions.
