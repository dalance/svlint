module M
  ( inout var foo // `foo` is missing prefix.
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M
  ( inout var logic [A-1:0] bar // `bar` is missing prefix, not `A`.
  );
endmodule
////////////////////////////////////////////////////////////////////////////////
module M
  ( inout var i_foo
  , inout var bar // `bar` is missing prefix.
  );
endmodule
