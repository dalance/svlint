module M;
  localparam bit P1 = &{a, b}; // No space after `&`.

  for (genvar i=0; i < 5; i++) begin // No space after `++`.
  end
endmodule
