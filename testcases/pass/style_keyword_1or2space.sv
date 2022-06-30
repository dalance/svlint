module M (
  input a,
  inout b,  // 1 space after `input` or `inout` keywords
  output c, // makes port identifiers unaligned.

  input  d,
  inout  e, // 2 spaces after `input` or `inout` keywords
  output f  // makes port identifiers aligned.
);
endmodule
