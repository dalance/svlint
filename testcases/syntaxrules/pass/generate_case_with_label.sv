module A;
  generate case (2'd3)
    2'd1:     begin: l_nondefault wire c = 1'b0; end
    default:  begin: l_default    wire c = 1'b0; end
  endcase endgenerate
endmodule
