This rule checks the whitespace immediately following these keywords:
`begin`
, `endchecker`
, `endclass`
, `endclocking`
, `endconfig`
, `endfunction`
, `endgroup`
, `endinterface`
, `endmodule`
, `endpackage`
, `endprimitive`
, `endprogram`
, `endproperty`
, `endsequence`
, `endtask`
, `fork`
, `join`
, `join_any`
, and `join_none`.
These keywords are used to delimit code blocks and should always be followed by
a colon, a newline, or exactly 1 space then a comment, e.g:
```systemverilog
if (FOO) begin: l_foo // Followed by a colon.
  ...
end

module top;
  ...
endmodule: top  // Followed by a colon.

// Followed by a newline.
if (FOO) begin
  ...
end

if (FOO) begin // Followed by a comment.
  ...
end
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
  - **style_keyword_newline** - Suggested companion rule.
