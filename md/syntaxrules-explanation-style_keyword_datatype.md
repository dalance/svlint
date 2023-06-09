This rule checks the whitespace immediately following these keywords:
`byte`
, `chandle`
, `event`
, `int`
, `integer`
, `logic`
, `longint`
, `real`
, `realtime`
, `ref`
, `reg`
, `shortint`
, `shortreal`
, `signed`
, `string`
, `supply0`
, `supply1`
, `time`
, `tri`
, `tri0`
, `tri1`
, `triand`
, `trior`
, `unsigned`
, `uwire`
, `void`
, `wand`
, `wire`
, and `wor`.
These keywords are used to declare the datatype of signals/variables (like
`logic foo`), and cast expressions (like `int'(foo)`).

See also:
- **style_keyword_indent** - Suggested companion rule.
- **style_keyword_0or1space** - Suggested companion rule.
- **style_keyword_0space** - Suggested companion rule.
- **style_keyword_1or2space** - Suggested companion rule.
- **style_keyword_1space** - Suggested companion rule.
- **style_keyword_construct** - Suggested companion rule.
- **style_keyword_end** - Suggested companion rule.
- **style_keyword_maybelabel** - Suggested companion rule.
- **style_keyword_new** - Suggested companion rule.
- **style_keyword_newline** - Suggested companion rule.
