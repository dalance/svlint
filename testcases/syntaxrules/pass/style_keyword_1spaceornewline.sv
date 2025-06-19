module M();
  always_comb
    case (a) matches
      tagged Jmp .j: b = 1;
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M();
  always_comb
    if (a matches tagged Jmp .j)
      b = 1;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M();
  always_comb
    case (a) matches // with a comment
      tagged Jmp .j: b = 1;
    endcase
endmodule
