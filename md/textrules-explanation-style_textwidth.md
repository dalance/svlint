Check that each line is fewer than 80 (configurable) characters in length.
While modern screens can support huge widths with hundreds of characters, there
are several good reasons for using hard wraps at a fixed width:

1. Side-by-side diffs are difficult to read, and might not fit on even a wide
  screen when lines are too long.
  This is important for your code to get through reviews smoothly.
2. Your colleagues' eyesight may not be as good as your's, so they might not be
  able to view as many horizontal characters as your setup allows.
  This may be important if you have policies and/or regulations around
  discrimination in your workplace.
3. Humans tend to find it easier to read narrower columns of text.
  For example, newspapers print articles in columns.
4. If you ever need to print code, hard wraps at less than 80 characters will
  make this much easier.
5. If you need to give colleagues a walkthrough of your code, it's much easier
  for a presenter to only need one axis of scrolling (vertically).
  Similarly, the autoscroll function on e-readers usually only works
  vertically.

Some arguments are made that this restriction removes artistic licence.
The usual counter to this is that engineers (most SystemVerilog authors) should
be focused on engineering problems, not artistry.
If other engineers cannot efficiently read and understand your code, then this
becomes an engineering problem.

See also:
- <https://en.wikipedia.org/wiki/Characters_per_line>
- <https://en.wikipedia.org/wiki/Line_length>
