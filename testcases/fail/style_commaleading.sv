module M
#( bit FOO = 1 // space after `#(` causes misalignment
, int BAR = 2
,  bit [31:0] BAZ = 2 // too many spaces after comma
)
(input  var logic i_abc // missing space after `(`
,output var logic o_ghi // missing space after comma
);
  assign {foo, bar} = { // brace not followed by a single space
      i_abc
    ,12'h345 // missing space after `(`
    ,  b_def // too many spaces after comma
    };
  function foo
  (input a // missing space after `(`
  ,  input b // too many spaces after comma
  );
  endfunction
endmodule
