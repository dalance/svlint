module M
  #(bit FOO = 1 // comment
  , int BAR = 2 /* comment */
  , bit [31:0] BAZ = 2
  )
  ( input  var logic i_abc // comment
  , output var logic o_ghi /* comment */
  );

  assign {foo, bar} =
    { i_abc
    , 12'h345
    , b_def     // comment
    , 16'h3456  /* comment */
    };

  assign singleline2D = {{foo, bar}, {foo, bar}, {foo, bar}};

  function F
    ( input a
    , input b
    );
  endfunction
endmodule
