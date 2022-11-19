module M;
  initial
    unique case (a)
      default: b = 1;
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial
    unique if (a)
      b = 1;
    else if (a)
      b = 2;
    else
      b = 3;
endmodule
