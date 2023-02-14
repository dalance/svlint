Trailing whitespace, i.e. space characters immediately followed by a newline,
lead to unnecessary differences in version control because some/many/most
developer's editors are setup to remove this on writing to disk.
This rule simply checks that any newline (outside of string literals) is
not immediately preceeded by a space character.
You can

See also:

- **style_indent** - Suggested companion rule.
- **tab_character** - Suggested companion rule.
- Vim: <https://vimtricks.com/p/vim-remove-trailing-whitespace/>
- Emacs: <https://www.emacswiki.org/emacs/WhiteSpace>
- VSCode: `files.trimTrailingWhitespace: true,`
- Notepad++: "Trim Trailing Space" on <https://npp-user-manual.org/docs/editing/>
