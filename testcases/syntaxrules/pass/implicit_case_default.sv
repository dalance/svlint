module M;
  always_comb begin
    y = 0;
    case (x)
      1: y = 1; // case default is implicit
    endcase
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb begin
    y = 0;
    z = 0;
    w = 0;
    case (x)
      1: y = 1;
      2: begin
        z = 1;
        w = 1;
      end
    endcase
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    case (x)
      1: y = 1;
      default: y = 0;
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb
    case (x)
      1: p = 1;
      2: q = 0;
      default: begin
        p = 0;
        q = 0;
      end
    endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always_comb begin
    p = 0;  // p -> implicit default
    q = 0;  // q -> implicit default
    case (x)
      1: p = 1;
      2: q = 1;
      3: r = 1;
      default: r = 1; // r -> explicit default
    endcase
  end
endmodule
