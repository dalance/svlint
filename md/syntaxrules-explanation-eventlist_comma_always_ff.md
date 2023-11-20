Require the `or` keyword as the event expression separator instead of the comma
character (`,`) in `always_ff` processes, for cosmetics/readability and
potential textual conversion to Verilog95.

SystemVerilog allows for two synonymous separators (`or` and `,`) in event
control sensitivity lists.
The separators may be mixed freely, as shown in the following examples from
IEEE1800-2017 page 218.

```systemverilog
always @(a, b, c, d, e)
always @(posedge clk, negedge rstn)
always @(a or b, c, d or e)
```

The first released standard of Verilog (IEEE1364-1995) allows only the `or`
keyword as a separator in sensitivity lists.
Perhaps realising that other types of lists required the comma separator,
subsequent releases of Verilog (IEEE1364-2001 and IEEE1364-2005) and all
versions of SystemVerilog allow the use of either separator.
It can be visually jarring for readers to parse lists with more than one
separator, thus impairing readabilty.
Therefore, this rule requires that only one type of separator is used, i.e.
forbidding the use of the comma separator.

The advantage of requiring `or` rather than `,` in the sensitivity list of
`always_ff` processes is that a codebase may be converted from SystemVerilog to
Verilog95, with a simple text-replacement of `always_ff` to `always`.
Naturally, the rest of the codebase must contain only Verilog95-compatible
syntax for that conversion to be worthwhile.
This rule only applies to event expressions in `always_ff` processes.

See also:
- **eventlist_or** - Mutually exclusive rule.
- **blocking_assignment_in_always_ff** - Useful companion rule.
- **general_always_no_edge** - Useful companion rule.
- **style_keyword_1space** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 9.2.2 Always procedures
- 9.4 Procedural timing controls
- 9.4.2.1 Event OR operator
