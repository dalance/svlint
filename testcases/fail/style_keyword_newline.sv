module A;
  generate
    case (foo)
      123: a = b;
    endcase if (foo) a = b; // no newline after `endcase`
  endgenerate   // multiple spaces then comment after `endgenerate`
endmodule

