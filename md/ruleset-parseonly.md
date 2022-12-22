# parseonly

If a file passes this ruleset you have these pieces of information:
- The file is valid UTF-8.
- svlint's preprocessor can successfully parse and emit text.
- The emitted text is valid SystemVerilog adhering to Annex A of IEEE1800-2017,
  i.e. there are no syntax errors.

### Test Each File for UTF-8 Encoding

Errors resulting from files with non-UTF8 encodings can be difficult to read,
possibly because UTF8 is a fundamental assumption made by the Rust components
used to build the parser stage.
This can be particularly difficult when multiple files or filelists are used
given to svlint.
On Unix-like platforms, we can use common utilities to assist debugging the
issue of differently encoded files.

To get a list of all the files examined by a particular invocation of svlint,
use the `--dump-filelist` option and parse the output.
```sh
# Delete ANSI control sequences that begin with ESC and (usually) end with m.
STRIP_ANSI_CONTROL="| sed -e 's/\\o33\\[[0-9;]*[mGKHF]//g'"

# Delete every ASCII control character except line feed ('\n' = 0o12 = 10 = 0x0A).
STRIP_ASCII_CONTROL="| tr -d '[\\000-\\011\\013-\\037\\177]'"

# Extract files from YAML output of --dump-filelist.
# First, delete all lines up to and including one equal to `".":`.
# Next, delete all lines equal to `  files:`.
# On encountering a line equal to `  incdirs:`, quit processing immediately.
# Replace lines containing double quotes with the characters between those quotes.
# Suppress normal output with -n, but print remaining content.
FILES_FROM_YAML="| sed -n '"
FILES_FROM_YAML="${FILES_FROM_YAML}0,/^\"\\.\":\$/d;"
FILES_FROM_YAML="${FILES_FROM_YAML}/^  files:\$/d;"
FILES_FROM_YAML="${FILES_FROM_YAML}/^  incdirs:\$/q;"
FILES_FROM_YAML="${FILES_FROM_YAML}s/[^\"]*\"\\([^\"]*\\).*/\1/;"
FILES_FROM_YAML="${FILES_FROM_YAML}p"
FILES_FROM_YAML="${FILES_FROM_YAML}'"

# Combine the above fragments into a string which can be evaluated and
# processed with xargs.
# NOTE: Creating a variable with the result (instead of the command) would lead
# to undefined behavior where the list of file paths exceeds 2MiB.
SVFILES="svlint --dump-filelist $*"
SVFILES="${SVFILES} ${STRIP_ANSI_CONTROL}"
SVFILES="${SVFILES} ${STRIP_ASCII_CONTROL}"
SVFILES="${SVFILES} ${FILES_FROM_YAML}"
```

Testing files for alternate (i.e. legacy) encodings is made easier by
specialized tools like `iconv` on Unix.
```sh
eval "${SVFILES}" | xargs -I {} iconv -f UTF-8 -t UTF-8 {} > /dev/null
```

On Windows, the default environment does not contain utilities such as `iconv`,
so some system-specific scripting may be more appropriate.

### Disable All Rules

All rules are implicitly disabled, and all options are implicitly set to their
default values.
Despite non of svlint's rules being enabled, this instructs the files to be
preprocessed and parsed, i.e. internally processed from text to a syntax tree.

```toml
[option]
[rules]
```
