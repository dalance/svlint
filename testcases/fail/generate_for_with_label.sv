module M;
  for (genvar i=0; i < 10; i++) // No begin/end delimeters.
    assign a[i] = i;
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  for (genvar i=0; i < 10; i++) begin // begin/end delimiters, but no label.
    assign a[i] = i;
  end
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  for (genvar i=0; i < 10; i++) begin: foo // With label, but no prefix.
    assign a[i] = i;
  end
endmodule
