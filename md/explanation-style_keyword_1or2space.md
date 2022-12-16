This rule checks the whitespace immediately following the `inout` and `input`
keywords.
These keywords specify the direction of signal ports, and are frequently used
alongside the `output` keyword which is 1 character longer.
The suggested companion rule **style_keyword_1space** checks that `output` is
followed by a single space, and this rule allows `inout`/`input` to be followed
by a single space too.
However, it is common and visually appealing to have port definitions
vertically aligned, so this rule also allows 2 following spaces, e.g:
```systemverilog
module foo
  ( input  var logic i_foo // aligned, 2 spaces
  , output var logic o_bar
  , inout tri logic b_baz // unaligned, 1 space
  );
endmodule
```

See also:
  - **style_keyword_indent** - Suggested companion rule.
  - **style_keyword_0or1space** - Suggested companion rule.
  - **style_keyword_0space** - Suggested companion rule.
  - **style_keyword_1space** - Suggested companion rule.
  - **style_keyword_construct** - Suggested companion rule.
  - **style_keyword_datatype** - Potential companion rule.
  - **style_keyword_end** - Suggested companion rule.
  - **style_keyword_maybelabel** - Suggested companion rule.
  - **style_keyword_newline** - Suggested companion rule.
