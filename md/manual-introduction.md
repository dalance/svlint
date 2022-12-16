
# Introduction

## About This Document

This document is generated from the Markdown files in `md/*.md`, the rules'
source code (`svlint/src/rules/*.rs`), and their testcases
(`testcases/(fail|pass)/*.sv`) using the `mdgen` utility.

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

Naming conventions help a human reader to take in large amounts of detailed
information (e.g. from netlists, timing reports) by allowing the reader to
predict the function of a signal from its name, and predict part of a signal's
name from its function.
For example, a common convention is: "All signals inferring the output of a
flip-flop must be suffixed with `_q`."
If an engineer reads a synthesized netlist and sees a flip-flop cell named
without the `_q` suffix, there may be a coding error, so further checks
with the author are required.
On the frontend, a reader can quickly scan a SystemVerilog file to check that
signals have the `_q` suffix if (and only if) they are driven in `always_ff`
processes.
Other naming conventions are useful for helping readers follow the direction of
signals through ports, find files quickly in a large filesystem, find
appropriate files from hierarchical paths in a netlist, and more.

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

## How It Works

This tool (svlint) works in a series of well-defined steps:
1. On startup, search for a configuration file or use a default configuration.
2. Examine the configuration to determine which rules should be enabled and
  load them into memory.
3. Parse a whole file for preprocessor constructs like `` `ifdef `` and
  `` `include ``.
4. Apply the preprocessor semantics to produce a source descrition text.
5. Parse the source description into a syntax tree.
  The grammatical structure of a syntax tree is described in IEEE1800-2017
  Annex A using Backus-Naur Form.
6. Iterate over each node of the syntax tree in order.
7. For each node, apply each rule independently.
8. If a rule detects an undesirable quality in the syntax tree, then return a
  failure, otherwise return a pass.

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
  primarily out of completeness, but their use may cause other issues.
  For example, **style_keyword_datatype** exists to ensure all SystemVerilog
  keywords are captured in the `style_keyword_*` set, but its use is not
  suggested because it is visually appealing (and common practice) to align
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
