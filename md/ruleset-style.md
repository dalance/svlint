
The set of whitespace-only checks which are "suggested" in the explanations
of the **style_** rules.


### Motivation

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

```toml
option.textwidth = 80
textrules.style_textwidth = true
```


### Test for Obfuscated Statements

To get a list of all the files examined by a particular invocation of svlint,
use the variable `${SVFILES}`, which is provided in all POSIX wrapper scripts.

The `grep` utility can be used to report obfuscated statements where semicolons
are pushed off the RHS of the screen.
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


### Indentation

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


### Indentation Preprocessor Considerations

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


### Operators and Keywords

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


### Comma-Separated Lists

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

