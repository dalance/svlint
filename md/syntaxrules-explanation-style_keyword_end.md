This rule checks the whitespace immediately following the `end` keyword.
The keyword `end` always be followed by a newline,
exactly 1 space then another keyword, a colon, or exactly 1 space then a
comment, e.g:
```systemverilog
// Followed by a newline.
if (FOO) begin
  ...
end

// Followed by 1 space then a keyword.
if (FOO) begin
  ...
end else ...

// Followed by a colon.
if (FOO) begin: l_foo
  ...
end: l_foo

// Followed by a comment.
if (FOO) begin // {{{ An opening fold marker.
  ...
end // }}} A closing fold marker.
```

See also:
- **style_keyword_indent** - Suggested companion rule.
- **style_keyword_0or1space** - Suggested companion rule.
- **style_keyword_0space** - Suggested companion rule.
- **style_keyword_1or2space** - Suggested companion rule.
- **style_keyword_1space** - Suggested companion rule.
- **style_keyword_construct** - Suggested companion rule.
- **style_keyword_datatype** - Potential companion rule.
- **style_keyword_maybelabel** - Suggested companion rule.
- **style_keyword_new** - Suggested companion rule.
- **style_keyword_newline** - Suggested companion rule.
