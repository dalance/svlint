module M;
  generate case (2'd0)
    2'd1:     logic a = 1'b0; // nondefaultNoBegin
    default:  logic a = 1'b0; // defaultNoBegin
  endcase endgenerate
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  generate case (2'd1)
    2'd1:     begin logic b = 1'b0; end // nondefaultNoLabel
    default:  begin logic b = 1'b0; end // defaultNoLabel
  endcase endgenerate
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  generate case (2'd2)
    2'd1:     begin: nondefaultNoPrefix logic c = 1'b0; end
    default:  begin: noPrefix           logic c = 1'b0; end
  endcase endgenerate
endmodule
////////////////////////////////////////////////////////////////////////////////
module M;
  case (2'd3) // No need for the generate/endgenerate keywords.
    2'd1:     begin: nondefaultNoPrefix logic d = 1'b0; end
    default:  begin: noPrefix           logic d = 1'b0; end
  endcase
endmodule
