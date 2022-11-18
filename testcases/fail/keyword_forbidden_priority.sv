module M;
  initial
    priority case (a)
      default: b = 1;
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial
    priority if (a)
      b = 1;
    else if (a)
      b = 2;
    else
      b = 3;
endmodule
