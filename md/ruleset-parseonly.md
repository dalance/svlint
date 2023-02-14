
If a file passes this ruleset you have these pieces of information:

- The file is valid UTF-8.
- svlint's preprocessor can successfully parse and emit text.
- The emitted text is valid SystemVerilog adhering to Annex A of IEEE1800-2017,
  i.e. there are no syntax errors.


### Disable All Rules

All rules are implicitly disabled, and all options are implicitly set to their
default values.
Despite non of svlint's rules being enabled, this instructs the files to be
preprocessed and parsed, i.e. internally processed from text to a syntax tree.

```toml
[option]
[rules]
```
