module A();
initial begin
    priority case (a)
        default: b = 1;
    endcase
end
endmodule
