module M
  #(parameter int P // Type is specified (good), but default value isn't (bad).
  ) ();
endmodule
////////////////////////////////////////////////////////////////////////////////
module M
  #(parameter Q // Neither type or default value are specified (very bad).
  ) ();
endmodule
////////////////////////////////////////////////////////////////////////////////
module M
  #(parameter int P = 0
  , R // Legal, but even less clear about the author's intention.
  ) ();
endmodule
