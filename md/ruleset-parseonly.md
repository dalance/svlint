parseonly
---------

All rules are implicitly disabled, and all options are implicitly set to their
default values.

```toml
[option]
[rules]
```

If a file passes this ruleset you have these pieces of information:
- The file is valid UTF-8.
- svlint's preprocessor can successfully parse and emit text.
- The emitted text is valid SystemVerilog adhering to Annex A of IEEE1800-2017,
  i.e. there are no syntax errors.
