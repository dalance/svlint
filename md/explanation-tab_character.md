Tab characters appear as different widths in dependent on editor/viewer setup,
leading to confusion for readers with a different setup.
Spaces are all but essential, but tabs are not, so this rule simply forbids the
use of tabs.

NOTE: `sv-parser`, the basis of svlint and svls requires files to be encoded
in UTF-8.
See `man iconv` for details on how to convert legacy encodings to UTF-8.

See also:

- **style_indent** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:

- Not applicable.
