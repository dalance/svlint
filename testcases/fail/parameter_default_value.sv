module M
  #(parameter int P // Type is specified (good), but default value isn't (bad).
  , parameter Q // Neither type or default value are specified (very bad).
  , R // Legal, but even less clear about the author's intention.
  ) ();
endmodule
