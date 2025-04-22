module M();
  always_comb
    if (a matches  tagged Jmp .j)
      b = 1;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M();
  always_comb
    case (a) matches  // comment with two spaces
      tagged Jmp .j: b = 1;
    endcase
endmodule
