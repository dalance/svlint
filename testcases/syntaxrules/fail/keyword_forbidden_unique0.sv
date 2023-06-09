module M;
  initial begin
    unique0 case (a)
      default: b = 1;
    endcase
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  initial
    unique0 if (a)
      b = 1;
    else if (a)
      b = 2;
    else
      b = 3;
endmodule
