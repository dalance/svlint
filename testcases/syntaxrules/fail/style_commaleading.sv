module M
  #( bit FOO = 1 // Space after `#(` causes misalignment.
  , int BAR = 2
  ,  bit [31:0] BAZ = 2 // Too many spaces after comma.
  )
  (input  var logic i_abc // Missing space after `(` causes misalignment.
  ,output var logic o_ghi // Missing space after comma.
  );

  assign {foo, bar} = { // One-line style is okay.
      i_abc
    ,12'h345 // Missing space.
    ,  b_def // Too many spaces after comma.
    };

  function foo
  (input a // Missing space after `(` causes misalignment.
  ,  input b // Too many spaces after comma.
  );
  endfunction

endmodule
