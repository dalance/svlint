module M;
  always @* begin // No sensitivity list.
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  always @ (a or b) begin // No sensitivity to posedge, negedge, or edge.
  end
endmodule
