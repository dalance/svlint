module M;
  genvar foo; // Unconfigured forbidden regex matches (almost) anything.
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  // Unconfigured forbidden regex matches (almost) anything.
  for (genvar bar=0; bar < 5; bar++) begin
  end
endmodule
