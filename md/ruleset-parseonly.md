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

Errors resulting from files with non-UTF8 encodings can be difficult to read,
possibly because UTF8 is fundamental assumption made by Rust components used to
build the parser stage.
This can be particularly difficult when multiple files or filelists are used
given to svlint.
On Unix-like platforms, we can use common utilities to assist debugging the
issue of differently encoded files.

To get a list of all the files examined by a particular invocation of svlint,
use the `--dump-filelist` option and parse the output.
```sh
SVLINT_FILES=$(svlint --dump-filelist $* |
  sed -n '1,/^"\.":$/d;/  files:$/d;/  incdirs:$/q;s/[^"]*"\([^"]*\).*/\1/;p'
)
```

Testing files for alternate (i.e. legacy) encodings is made easier by
specialized tools like `iconv` on Unix.
```sh
for f in $SVLINT_FILES; do
  iconv -f utf8 "$f" -t utf8 -o /dev/null
done
```

On Windows, such utilities are not generally installed by default.

TODO: rm (TEST GitHub Actions)
