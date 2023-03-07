This rule checks the whitespace immediately following these keywords:
`always_comb`
, `always_latch`
, `assign`
, `else`
, `final`
, `generate`
, and `initial`.
These keyword open constucts and should always be followed by a newline,
exactly 1 space then another keyword/identifier, or exactly 1 space then a
comment, e.g:
```systemverilog
// Followed by 1 space then another keyword.
always_comb begin
  foo = '0;
  foo[0] = 5;
end

// Followed by 1 space then an identifier.
always_comb bar = 5;

// Followed by a newline.
always_comb
  if (x < y)
    z = 5;
  else // Followed by 1 space then this comment.
    z = 6;

// Assign to a concatenation.
assign // You could use `always_comb` instead.
  { foo
  , bar
  , baz[i][j][k]
  } = '0;
```

See also:
- **style_keyword_indent** - Suggested companion rule.
- **style_keyword_0or1space** - Suggested companion rule.
- **style_keyword_0space** - Suggested companion rule.
- **style_keyword_1or2space** - Suggested companion rule.
- **style_keyword_1space** - Suggested companion rule.
- **style_keyword_datatype** - Potential companion rule.
- **style_keyword_end** - Suggested companion rule.
- **style_keyword_maybelabel** - Suggested companion rule.
- **style_keyword_newline** - Suggested companion rule.
