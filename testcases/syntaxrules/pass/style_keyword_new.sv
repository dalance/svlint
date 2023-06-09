class Packet; // Example from IEEE1800-2017, page 174.
  integer command;

  function new(); // Constructor without arguments.
    command = IDLE;
  endfunction
endclass

class C1 extends Packet;
  function new // Constructor with arguments.
    ( int cmd = IDLE
    , int addr = 123
    , int data = 0
    );
    command = cmd;
  endfunction
endclass

class C2 extends C1;
  function new;
    super.new(5); // Super constructor.
  endfunction
endclass

module M;
  Packet p1 = new; // Construction without arguments

  C1 p2 = new(1, 2, 3); // Construction with short arguments.

  C2 p3 = new // Construction with long arguments.
    ( STARTUP
    , A_RATHER_LONG_CONSTANT_IDENTIFIER
    , 456
    );
endmodule
