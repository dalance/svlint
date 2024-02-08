module M;
  always_comb
    y = 0;
    case(x)
      1: y = 1; // case default is implicit
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    case(x)
      1: y = 1;
      default: y = 0;
    endcase
endmodule
