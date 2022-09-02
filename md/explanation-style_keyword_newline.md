This rule checks the whitespace immediately following these keywords:
, `endcase`
, `endgenerate`
, `endspecify`
, `endtable`
, `specify`
, and `table`.
These keywords are used to delimit code blocks and should always be followed by
a newline or exactly 1 space then a comment, e.g:
```systemverilog
case (FOO)
  ...
endcase // Followed by a comment.

// Followed by a newline.
case (FOO)
  ...
endcase
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1or2space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.

The most relevant clauses of IEEE1800-2017 are:
  - Not applicable.
