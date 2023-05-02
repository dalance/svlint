Require the comma character (`,`) as the event expression separator instead of
the `or` keyword, for cosmetics/readability.

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
forbidding the use of the `or` separator.

The advantage of requiring `,` rather than `or` is that sensitivity lists look
the same as every other type of list which the reader's eye will be better
trained to read.
This rule applies to event expressions in any context, not only `always_ff`
processes.

See also:
- **eventlist_comma_always_ff** - Mutually exclusive rule.
- **blocking_assignment_in_always_ff** - Useful companion rule.
- **level_sensitive_always** - Useful companion rule.
- **style_keyword_commaleading** - Useful companion rule.

The most relevant clauses of IEEE1800-2017 are:
- 9.2.2 Always procedures
- 9.4 Procedural timing controls
- 9.4.2.1 Event OR operator
