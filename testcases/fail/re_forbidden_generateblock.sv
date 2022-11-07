module M;
  if (0) begin: foo // Unconfigured forbidden regex matches (almost) anything.
    assign a = 0;
  end: foo
  else begin: bar // Unconfigured forbidden regex matches (almost) anything.
    assign a = 1;
  end: bar

  // Unconfigured forbidden regex matches (almost) anything.
  for (genvar i=0; i < 5; i++) begin: foo
    assign b[i] = 0;
  end: foo

  case (0)
    0: begin: foo // Unconfigured forbidden regex matches (almost) anything.
      assign c = 0;
    end: foo
    1: begin: bar // Unconfigured forbidden regex matches (almost) anything.
      assign c = 1;
    end: bar
    default: begin: baz // Unconfigured forbidden regex matches (almost) anything.
      assign c = 2;
    end: baz
  endcase
endmodule
