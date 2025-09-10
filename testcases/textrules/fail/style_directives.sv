module `ifdef FOO Foo `else Bar `endif
  (); // ifdef, else, and endif are on a single line.
endmodule
////////////////////////////////////////////////////////////////////////////////
`ifdef FOO
  `ifdef BAR
    // Preprocessor directives are indented with respect to surrounding
    // preprocessor code.
    `define FOOBAR
  `endif
`endif
////////////////////////////////////////////////////////////////////////////////
module M ();
  always_comb
    // Preprocessor directives are indented with respect to source description.
    `ifdef FOO
      if (a)
        b = c;
      else
        b = d;
    `else
      b = e;
    `endif
endmodule
////////////////////////////////////////////////////////////////////////////////
 `ifdef FOO // The ifdef is not commented and indented
`endif
