The keywords `wire` and `reg` are present in SystemVerilog primarily for
backwards compatibility with Verilog (IEEE1364-1995).
In SystemVerilog, there are additional keywords, such as `logic` and `tri`
with more refined semantics to better express the programmer's intent.

The LRM covers the use of `wire`:
> The net types `wire` and `tri` shall be identical in their syntax and
> functions; two names are provided so that the name of a net can indicate the
> purpose of the net in that model.

The LRM covers the use of `reg`:
> The keyword `reg` does not always accurately describe user intent, as it
> could be perceived to imply a hardware register. The keyword `logic` is a
> more descriptive term. `logic` and `reg` denote the same type.

See also:

- **default_nettype** - Useful companion rule.
- **inout_with_tri** - Useful companion rule.
- **input_with_var** - Useful companion rule.
- **output_with_var** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:

- 6.6.1 Wire and tri nets
- 6.11.2 2-state (two-value) and 4-state (four-value) data types
