module M;
  always_comb begin
    y = 0;
    z = 0;
    //w = 0;
    case(x)
      1: y = 1; // case default is implicit
      2: begin 
        z = 1;
        w = 1;
      end
    endcase
  end
endmodule