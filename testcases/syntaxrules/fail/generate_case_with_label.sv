module M;
  case (2'd0)             // No begin/end delimiters.
    2'd1:
      logic a = 1'b0;
    default:
      logic a = 1'b0;
  endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  case (2'd1)             // begin/end delimiters, but no label.
    2'd1: begin
      logic b = 1'b0;
    end
    default: begin
      logic b = 1'b0;
    end
  endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  case (2'd2)             // With label, but no prefix.
    2'd1: begin: foo
      logic c = 1'b0;
    end: foo              // NOTE: With optional label on end.
    default: begin: bar
      logic c = 1'b0;
    end                   // NOTE: Without optional label on end.
  endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  case (2'd4)             // Without default arm.
    2'd1: begin: foo
      logic e = 1'b0;
    end
  endcase
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  case (2'd5)             // Without non-default arm.
    default: begin: bar
      logic f = 1'b0;
    end
  endcase
endmodule
