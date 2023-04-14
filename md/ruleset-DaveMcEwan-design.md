
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
1. Assure the reviewer that common assumptions hold true, e.g. "*all* variables
  are declared before use" and "*all* constants are 2-state".
2. Minimise scope of objects, i.e. how much information a reader must keep in
  mind while reading a section of code.
3. Clarify intent.
  An author should demonstrate (to their readers) that they have considered the
  precise meaning of what they wrote, thus giving little room for
  mis-interpretation by blurry-eyed readers or tools operating in the grey
  areas of the SystemVerilog LRM.
  One example is requiring every `case` to have a `default` arm.
  Another prominent example is in the rules `sequential block_in_*`, i.e.
  specifying and implementing purely combinatorial logic is clearer with purely
  combinatorial code (rather than procedures).
4. Encourage canonicalisation.
  The infamous [Zen of Python](https://peps.python.org/pep-0020/) phrases this
  concept as "There should be one-- and preferably only one --obvious way to do
  it."
  By enforcing a strict style, readers can read and comprehend a large body of
  code quickly and accurately.

Through rules which detect code that goes against those aims, reviewers are
free to concentrate on aspects which require high-level thought such as
"Is this an efficient design?", instead of "Is this code formatted prettily?".
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

These conventions help give a consistent view over different ways of viewing
files which include the writer's text editor (Vim, VSCode, Emacs, etc.),
consumer's text editor, reviewer's web-based tools (GitHub, BitBucket, GitLab,
etc.), printed material (e.g. via PDF), and logfiles from CI/CD tools (GitHub
Actions, Bamboo, Jenkins, etc).


#### Test Each File for Excessively Long Lines

TODO: Replace with textrules when they are available.

To get a list of all the files examined by a particular invocation of svlint,
use the variable `${SVFILES}`, which is provided in all POSIX wrapper scripts.

The `grep` utility can be used to detect, and report, lines longer than a given
number of characters.
```sh
TEXTWIDTH='80'
LINELEN="grep -EvIxHn --color '.{0,${TEXTWIDTH}}' {};"
LINELEN="${LINELEN} if [ \"\$?\" -eq \"0\" ]; then"
LINELEN="${LINELEN}   echo '!!! Lines longer than ${TEXTWIDTH} characters !!!';"
LINELEN="${LINELEN}   exit 1;"
LINELEN="${LINELEN} else"
LINELEN="${LINELEN}   exit 0;"
LINELEN="${LINELEN} fi"
eval "${SVFILES}" | xargs -I {} sh -c "${LINELEN}"
```

Another use of `grep` is to report obfuscated statements where semicolons are
pushed off the RHS of the screen.
```sh
OBFUSTMT="grep -EIHn --color '[ ]+;' {};"
OBFUSTMT="${OBFUSTMT} if [ \"\$?\" -eq \"0\" ]; then"
OBFUSTMT="${OBFUSTMT}   echo '!!! Potentially obfuscated statements !!!';"
OBFUSTMT="${OBFUSTMT}   exit 1;"
OBFUSTMT="${OBFUSTMT} else"
OBFUSTMT="${OBFUSTMT}   exit 0;"
OBFUSTMT="${OBFUSTMT} fi"
eval "${SVFILES}" | xargs -I {} sh -c "${OBFUSTMT}"
```

On Windows, the default environment does not contain utilities such as `grep`,
so some system-specific scripting may be more appropriate.


#### Indentation

An indent of 2 spaces, not tabs, is chosen.
For better or worse, contemporary computer language styles have moved
decisively away from using tabs for indentation.
The most likely reason behind this is that tab display width is configurable
so tab indentations are shown differently, depending on the reader's personal
configuration.
```toml
option.indent = 2
rules.tab_character = true
rules.style_indent = true
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
rules.multiline_if_begin = true
rules.multiline_for_begin = true
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

Svlint rules operate on the `source_text` part of SystemVerilog, i.e. after the
preprocessor has been applied.
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
rules.style_trailingwhitespace = true
```

Problems around indented preprocessor directives must be caught before svlint's
preprocessor stage, so searching with `grep` beforehand is appropriate.
```sh
PPDIRECTIVES="define|undef|undefineall|resetall"
PPDIRECTIVES="${PPDIRECTIVES}|ifdef|ifndef|elsif|else|endif"
PPDIRECTIVES="${PPDIRECTIVES}|include"

PPINDENT="grep -EIHn --color '[ ]+\`(${PPDIRECTIVES})' {};"
PPINDENT="${PPINDENT} if [ \"\$?\" -eq \"0\" ]; then"
PPINDENT="${PPINDENT}   echo '!!! Indented preprocessor directives !!!';"
PPINDENT="${PPINDENT}   exit 1;"
PPINDENT="${PPINDENT} else"
PPINDENT="${PPINDENT}   exit 0;"
PPINDENT="${PPINDENT} fi"
eval "${SVFILES}" | xargs -I {} sh -c "${PPINDENT}"
```


#### Operators and Keywords

Consistent use of whitespace around operators and keywords makes it easier to
read expressions quickly and accurately.
```toml
rules.style_operator_arithmetic = true
rules.style_operator_boolean = true
rules.style_operator_integer = true
rules.style_operator_unary = true

rules.style_keyword_0or1space = true
rules.style_keyword_0space = true
rules.style_keyword_1or2space = true
rules.style_keyword_1space = true
rules.style_keyword_construct = true
rules.style_keyword_datatype = false # Overly restrictive.
rules.style_keyword_end = true
rules.style_keyword_maybelabel = true
rules.style_keyword_new = true
rules.style_keyword_newline = true
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
rules.style_commaleading = true
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
#rules.eventlist_or = true TODO: Uncomment when eventlist_or is available.
```


### Tool Consistency

TODO: Introduction and examples of tool incompatibilities.

Rules that forbid suspicious constructions, i.e. ways of specifying hardware
that are legal according to the LRM, but may express their intention unclearly.

The following subset is designed to detect potential mismatches between
simulation and synthesis.
These rules don't intentionally interact to provide additional properties.

```toml
# Common to **ruleset-simsynth** (a subset of **ruleset-designintent**).
rules.blocking_assignment_in_always_ff = true
#rules.blocking_assignment_in_always_latch = true TODO
rules.non_blocking_assignment_in_always_comb = true
rules.case_default = true
rules.enum_with_type = true
rules.function_with_automatic = true
rules.keyword_forbidden_priority = true
rules.keyword_forbidden_unique = true
rules.keyword_forbidden_unique0 = true
rules.level_sensitive_always = true
#rules.operator_case_equality = true # TODO

# Common to **ruleset-designintent**.
rules.action_block_with_side_effect = true
rules.default_nettype_none = true
rules.function_same_as_system_function = true
rules.keyword_forbidden_always = true
rules.keyword_forbidden_wire_reg = true
rules.non_ansi_module = true
```

When synthesised into a netlist, generate blocks should have labels so that
their inferred logic can be detected in hierarchical paths.
Although the LRM is clear about the implict naming of unlabelled generate
blocks (see IEEE1800-2017 clause 27.6), using a well-named label provides some
clarification about the intention behind that logic.
```toml
rules.generate_case_with_label = true
rules.generate_for_with_label = true
rules.generate_if_with_label = true
```

Generally, elaboration-time constants (`parameter`, `localparam`) should be
2-state types and declared with a default value.
Additionally, where the context defines that `parameter` is an alias for
`localparam`, authors should demonstate that they understand the constant
cannot be overriden by using the `localparam` keyword.
```toml
rules.localparam_type_twostate = true
rules.parameter_type_twostate = true
rules.localparam_explicit_type = true
rules.parameter_explicit_type = true
rules.parameter_default_value = true
rules.parameter_in_generate = true
rules.parameter_in_package = true
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
Further, the use of `generate` and `endgenerate` is entirely optional with no
semantic difference to not using them.
```toml
rules.genvar_declaration_in_loop = true
rules.genvar_declaration_out_loop = false
rules.keyword_forbidden_generate = true
rules.keyword_required_generate = false
```

Rules in the following subset combine to provide an important property for the
robust design of synthesisable hardware - that you can easily draw a schematic
of what the synthesis result should look like.
The two rules of thumb are to always fully specify decision logic, and never
use sequential models for (what will be synthesised to) parallel logic.
```toml
rules.explicit_case_default = true
rules.explicit_if_else = true
#rules.loop_statement_in_always_comb = true TODO
#rules.loop_statement_in_always_ff = true TODO
#rules.loop_statement_in_always_latch = true TODO
rules.sequential_block_in_always_comb = true
rules.sequential_block_in_always_ff = true
rules.sequential_block_in_always_latch = true
```

Where sequential modelling of parallel logic is an unavoidable pragmatic
approach, `begin` and `end` keywords should be used carefully and with proper
indentation.
```toml
rules.multiline_for_begin = true
rules.multiline_if_begin = true
```

The semantics around port declarations are, perhaps, unintuitive but were
designed for backward compliance with Verilog (IEEE1364-1995).
The below subset ensures that port declarations clearly convey important
information about the direction and update mechanism of each signal port.
```toml
rules.inout_with_tri = true
rules.input_with_var = true
rules.output_with_var = true
rules.interface_port_with_modport = true
```

TODO: `re_forbidden_*` for objects which should not be declared in designs.

```toml
option.re_forbidden_checker = ".*"
rules.re_forbidden_checker = false
option.re_forbidden_class = ".*"
rules.re_forbidden_class = false
option.re_forbidden_port_ref = ".*"
rules.re_forbidden_port_ref = false
option.re_forbidden_property = ".*"
rules.re_forbidden_property = false
option.re_forbidden_sequence = ".*"
rules.re_forbidden_sequence = false
option.re_forbidden_task = ".*"
rules.re_forbidden_task = false
```


### Naming Conventions

TODO: Replicate this section as **ruleset-DaveMcEwan-designnaming**.

TODO: Haskell-style hierarchy distinctions.

```toml
rules.lowercamelcase_interface = true
rules.uppercamelcase_module = true
rules.uppercamelcase_package = true
rules.prefix_instance = true
rules.re_required_generateblock = true
```

TODO: Port direction

```toml
rules.prefix_inout = true
rules.prefix_input = true
rules.prefix_output = true
rules.re_required_port_interface = true
```

TODO: Constant/variable distinction.

```toml
rules.re_required_function = true
rules.re_required_localparam = true
rules.re_required_parameter = true
rules.re_required_genvar = true
```

