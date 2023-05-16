module M;
  generate
    case (x)
      123: a = b;
    endcase if (x) a = b; // No newline after `endcase`.
  endgenerate   // Multiple spaces then comment after `endgenerate`.
endmodule

