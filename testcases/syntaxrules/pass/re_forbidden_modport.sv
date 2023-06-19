interface I;
  modport Xfoo // Identifier doesn't match default forbidden regex (X prefix).
  ( input i
  );
endinterface
