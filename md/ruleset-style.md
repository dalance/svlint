Style
-----

The set of whitespace-only checks which are "suggested" in the explanations
of the **style_** rules.

### Motivation

TODO

### Indentation

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
1) A text processing language (defining the preprocessor) in specified
informally in IEEE1800-2017 Clause 22 among other compiler directives.
2) The rest of SystemVerilog syntax is formally called `source_text`, is
specified formally in IEEE1800-2017 Annex A.
The svlint tool operates on the `source_text` part of SystemVerilog, after the
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

### Operators

TODO
```toml
rules.style_operator_arithmetic = true
rules.style_operator_boolean = true
rules.style_operator_integer = true
rules.style_operator_unary = true
```

### Keywords

TODO
```toml
rules.style_keyword_0or1space = true
rules.style_keyword_0space = true
rules.style_keyword_1or2space = true
rules.style_keyword_1space = true
rules.style_keyword_construct = true
rules.style_keyword_datatype = false
rules.style_keyword_end = true
rules.style_keyword_maybelabel = true
rules.style_keyword_newline = true
```

### Comma-Separated Lists

TODO
```toml
rules.style_commaleading = true
```
