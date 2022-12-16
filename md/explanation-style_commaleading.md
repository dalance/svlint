This rule is intended to enforce consistent formatting of comma-separated lists
such as parameter/signal port declarations, concatenations, assignment
patterns, and function arguments.
The rule is very simple: Each comma must be followed by exactly 1 space.

Comma-leading style is seen extensively in other languages, e.g. Haskell, and
lends itself well to SystemVerilog, as seen in the following examples.
```systemverilog
/* Module declaration without parameter ports.
*/
module Mod_A
  ( input  var logic i_abc // comment
  , inout  tri logic b_def /* comment */
  , output var logic o_ghi
  );
endmodule

/* Module declaration with parameter ports.
*/
module Mod_B
  #(int FOO = 1 // comment
  , bit BAR = 2 /* comment */
  , bit [31:0] BAZ = 2
  , parameter int BUZZ = 4
  )
  ( input  var logic i_abc // comment
  , inout  tri logic b_def /* comment */
  , output var logic o_ghi
  );


  /* Each item on its own line.
  - Short lines.
  - Every list indented to same level.
  - Single-line LHS can be any length without indent issue.
  */
  assign {foo, bar} =
    { i_abc
    , 12'h345
    , b_def     // comment
    , 16'h3456  /* comment */
    };


  /* Everything can fit on one line.
  - No space after opening parenthesis/bracket/brace.
  */
  assign singleline1D = {i_abc, 12'h345, b_def, 16'h3456};
  assign singleline2D = {{foo, bar}, {foo, bar}, {foo, bar}};

  /* Multi-dimensional concatenation with innermost array on one line.
  */
  assign matrix2D_A =
    { {elem21, elem20}
    , {elem11, elem10} // comment
    , {elem01, elem00} /* comment */
    };
  assign matrix3D_A =
    { { {elem211, elem210}
      , {elem201, elem200}
      }
    , { {elem111, elem110} // comment
      , {elem101, elem100} /* comment */
      }
    , { {elem011, elem010}
      , {elem001, elem000}
      }
    };

  /* Multi-dimensional concatenation with one element per line.
  */
  assign matrix2D_B =
    { { elem21
      , elem20_with_long_name
      }
    , { elem11 // comment
      , elem10 /* comment */
      }
    , { elem01_note_no_misalignment
      , elem00
      }
    };

  /* Module instance without parameter ports.
  */
  Mod_A u_instanceA
    ( .i_abc(foo) // comment
    , .b_def({bar, bar}) /* comment */
    , .o_ghi
    );

  /* Module instance with parameter ports.
  */
  Mod_B
    #(.FOO(1) // comment
    , .BAR(2) /* comment */
    , .BUZZ(2)
    ) u_instanceB
    ( .i_abc(foo) // comment
    , .b_def({bar, bar}) /* comment */
    , .o_ghi
    );

endmodule
```

See also:
  - **style_indent** - Suggested companion rule.
