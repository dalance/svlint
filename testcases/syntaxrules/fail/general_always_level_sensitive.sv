module M;
  always @(b) // Missing sensitivity to `c`.
    a = b + c;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @(a or b) // Correct sensitivity list, but error prone.
    a = b + c;
endmodule
