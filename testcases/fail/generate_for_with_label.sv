module M;
  for (genvar i=0; i < 10; i++) foo[i] = i;// noBegin
  for (genvar i=0; i < 10; i++) begin // noLabel
  end
endmodule
