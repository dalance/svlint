module M;
  if (a) begin // No label
  end else if (b) begin: l_def
  end else begin: l_hij
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (a) begin: l_abc
  end else if (b) begin // No label
  end else begin: l_hij
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (a) begin: l_abc
  end else if (b) begin: l_def
  end else begin // No label
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (c) begin: abc // No prefix
  end else if (d) begin: l_def
  end else begin: l_hij
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (c) begin: l_abc
  end else if (d) begin: def // No prefix
  end else begin: l_hij
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  if (c) begin: l_abc
  end else if (d) begin: l_def
  end else begin: hij // No prefix
  end
endmodule
