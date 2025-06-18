module M;
  logic a;
  logic b;
  logic c;
  assign c = a == b & 1;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  logic a;
  logic b;
  logic c;
  assign c = a > b | a < b;
endmodule
