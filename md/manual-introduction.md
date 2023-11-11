
# Introduction

## About This Document

This document is generated from the Markdown files in `md/*.md`, the rules'
source code (`svlint/src/*rules/*.rs`), and their testcases
(`testcases/*rules/(fail|pass)/*.sv`) using the `mdgen` utility.


## Purpose of Lint Checks

The authors of any works must consider their audience, particularly in how
different sections of the audience will interpret the works.
For example, an author of childrens books has two main sections of audience
(children, and their adult parents) so they might aim to please both sections
at once; Children with simple storylines and colorful pictures; Parents with
cultural references and subtle innuendo.
Authors writing in SystemVerilog also have two main sections of audience which
they should aim to please: 1) other silicon engineers, 2) mechanical tools.
Although the differences between human and mechanical readers are significant,
both must be satisfied for the text to be nice/enjoyable to work with.
While a simulation tool doesn't care about whitespace, indentation, or
thoughtful comments, your human colleagues will dread working with messy code
(rewiewing, modifying, building upon outputs, etc.), which ultimately wastes
their time, money, and sanity.
Human readers will usually be polite about sub-par work, but tools are much more
direct, simply spitting back at you with warning messages and an outright
refusal to work if you dare to mis-spell a variable name.

There are two main classes of rule for helping human readers:

1. Rules which codify naming conventions.
2. Rules which codify style/formatting conventions.

Further information on these concepts is provided in the `style` ruleset.

Just as individual human readers have their own preferences (in language,
style, naming conventions, etc.), each tool has its own quirks and ways of
interpreting things, particularly when the language specification is not fully
explicit.
The most prominent example of tools' differences in interpretation of
SystemVerilog is between tools for simulation and tools for synthesis.
The SystemVerilog language is specifed in IEEE1800-2017, also known as the
Language Reference Manual (LRM).
The LRM is clear that the specification is written in terms of simulation, but
that some of its constructs may be synthesized into physical hardware.
This distinction is the basis for a class of functional rules which aim to
minimize the risk of introducing a mismatch between simulation and synthesis.
Another class of functional rules is those which check for datatypes and
constructs that avoid compiler checks for legacy compatibility.


## How Svlint Works

Svlint works in a series of well-defined steps:

1. On startup, search for a configuration file or use a default configuration
  to determine which rules should be enabled and load them into memory.
2. Iterate over each line of a file in order, applying each `TextRule`
  independently.
  If any rule detects an undesirable quality in the text, then return a
  failure, otherwise continue.
3. Parse each whole file for preprocessor constructs like `` `ifdef `` and
  `` `include `` and apply the preprocessor semantics to produce a source
  description text.
4. Parse the file's source description into a syntax tree.
  The grammatical structure of a syntax tree is described in IEEE1800-2017
  Annex A using Backus-Naur Form.
5. Iterate over each node of the syntax tree in order, applying each
  `SyntaxRule` independently.
  If any rule detects an undesirable quality in the syntax tree, then return a
  failure, otherwise return a pass.


## Filelists

Specification of the files to be processed can be given on the command line by
*either* a list of files, e.g. `svlint foo.sv bar/*.sv`, or via filelists,
e.g. `svlint -f foo.fl -f path/to/bar.fl`.
It is not supported to specify both files and filelists, due to
concerns about usability due to the way command-line arguments are processed.

The following features are supported via the
[sv-filelist-parser](https://github.com/supleed2/sv-filelist-parser) crate.

- Comments, i.e. lines beginning with `//` or `#`, and empty lines are ignored.
- Specify include directories like `+incdir+path/to/foo`.
- Define preprocessor macros like `+define+FOO` or `+define+BAR=1`.
- Include other filelists like `-f path/to/foo.fl`
- All remaining lines are treated as file paths.
- Substitute environment variables like `$FOO`, `${FOO}`, or `$(FOO)`.

For example:
```
aaa.sv
$FOO/bbb.sv
${FOO}/ccc.sv
$(FOO)/ddd.sv
+incdir+$PWD/header/src
+define+SYNTHESIS
-f anotherFilelist.fl
```


## Plugin Syntax Rules

Svlint supports plugin syntax rules, an example of which is available
[here](https://github.com/dalance/svlint-plugin-sample).

A plugin rule is one which is compiled separately to the main svlint binary,
and is loaded at runtime.
In the same way as integrated syntax rules, a plugin rule must implement the
`SyntaxRule` trait, i.e. `check`, `name`, `hint`, and `reason`.
The `hint` and `reason` methods allow plugins to provide information back
to the user on the terminal, but they do not require testcases or an
explanation.
All loaded plugins, via the `--plugin` option, are enabled and have access
to all values in the TOML configuration.


## Configuration

Firstly, you need a TOML configuration file to specify which rules to enable.
By default, svlint will search up the filesystem hierarchy from the current
directory for a file called `.svlint.toml`, so you can place your configuration
in the project/repository root alongside `.gitignore`.
For example, if your terminal is currently working in `/path/to/project/`,
then svlint will first look for `/path/to/project/.svlint.toml`, then
`/path/to/.svlint.toml`, and so on until `/.svlint.toml`.

The name to search for, `.svlint.toml`, can be changed using the `--config`
command line option.
Alternatively, the configuration file location can be set explicitly, perhaps
for project-wide rules, by setting the environment variable
`SVLINT_CONFIG` to something like `/cad/projectFoo/teamBar.svlint.toml`.

If you don't have a ready-made configuration, you can use a pre-written one
from `rulesets/*.toml`, or create an example (which requires modification) by
running `svlint --example`.


If no configuration is found, the default behavior is to enable all rules,
which is certain to show failures because some rules conflict, e.g.
**keyword_forbidden_generate** and **keyword_required_generate**.

A configuration looks something like this:

```toml
[option]
exclude_paths = ["ip/.*"]
prefix_label = "lab_"

[textrules]
style_indent = true

[syntaxrules]
non_ansi_module = true
keyword_forbidden_wire_reg = true
```

When moving to a newer version of svlint, sometimes rules are renamed and new
rules are added.
To generate an updated configuration, use the `--update` command line option
which will load your existing configuration then emit the updated TOML to
STDOUT.

### `[option]` Section

- `exclude_paths` is a list of regular expressions.
  If a file path is matched with any regex in the list, the file is skipped.
- `prefix_(inout|input|output)` are strings which port identifiers must begin
  with.
  Only used when the corresponding rule is enabled.
  Defaults to `"b_"`, `"i_"`, and `"o_"` respectively.
- `prefix_label` is a string which generate labels must begin with.
  Applicable to `if/else`, `for`, and `case` generate constructs when the
  corresponding `generate_*_with_label` rule is enabled.
  Defaults to `"l_"`.
  To check only that a label exists, set this to `""`.
- `re_(forbidden|required)_*` are regular expressions for detailed naming
  conventions, used only when the corresponding rules are enabled.
  The defaults for `re_required_*` are either uppercase, lowercase, or
  mixed-case starting with lowercase, i.e. just vaguely sensible.
  The defaults for `re_forbidden_*` are to forbid all strings, except those
  starting with "X", i.e. not at all sensible (configuration required).
- Please see the explanations for individual rules for details of other
  options.

### `[textrules]` and `[syntaxrules]` Sections

All rules are disabled unless explicitly enabled in their corresponding
`[textrules]` or `[syntaxrules]` section.
To enable a rule, assign `true` to its name, e.g. `case_default = true`.

Where no configuration file can be found, all rules are implicitly
enabled which will most likely result in errors from conflicting rules, e.g.
**keyword_forbidden_generate** and **keyword_required_generate**.

If you need to turn off specific syntax rules for a section, then you can use
special comments within your SystemVerilog source code
(not currently available for text rules):
```systemverilog
/* svlint off keyword_forbidden_always */
always @* foo = bar;                      // <-- This line is special.
/* svlint on keyword_forbidden_always */
```


## Rule Documentation

Each rule is documented with 5 pieces of information:

- Hint: A brief instruction on how to modify failing SystemVerilog.
  Also displayed in supported editors using [svls](https://github.com/dalance/svls).
- Reason: A one sentence explanation of the rule's purpose.
  Also displayed in supported editors using [svls](https://github.com/dalance/svls).
- Pass Example: A valid piece of SystemVerilog which is known to pass the rule.
  Ideally, this will show an example of best-practice.
- Fail Example: A valid piece of SystemVerilog which is known to fail the rule.
  In some cases the code shows multiple commented examples.
- Explanation: A full explanation of the rule's purpose with references to any
  other relevant information sources.

In each rule's explanation there is a "see also" list of other rules, each with
a short reason why it should be seen.

- "suggested companion" - Suggestions are given for rules which do not check
  semantics, i.e suggestions are for style and naming conventions only.
- "potential companion" - These are noted where the named rule is given
  (mostly) for completeness, but their use may cause other issues.
  For example, **style_keyword_datatype** exists to ensure all SystemVerilog
  keywords are captured in the `style_keyword_*` syntax rules, but its use is
  not suggested because it is visually appealing (and common practice) to align
  the identifiers in adjacent declarations.
- "useful companion" - Enabling the named rule provides an additional set of
  properties which are useful for reasoning about the function and semantics of
  code which passes.
  For example, the conjunction of **localparam_type_twostate** and
  **localparam_explicit_type** allows for stronger confidence that the author
  has properly considered the type of each constant.
- "alternative" - The named rule *should* not be used in conjunction, i.e.
  enabling both rules is, at best, a waste compute power.
- "mutually exclusive alternative" - The named rule *can* not be used in
  conjunction, i.e. enabling both rules is nonsensical because a failure on one
  implies a pass on the other rule.

You are welcome to suggest a new rule through
[Issues](https://github.com/dalance/svlint/issues) or
[Pull Requests](https://github.com/dalance/svlint/pulls).

