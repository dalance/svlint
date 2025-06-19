This rule checks the whitespace immediately following the `matches` keyword.
The `matches` keyword can be used inside the condition of an if statement,
in which case there should be one space between the keyword and the following
symbol, i.e. `matches (tagged ...)`.
The `matches` keyword can also be used as part of a case statement, in which
case there should be a newline between the keyword and the following identifier,
i.e. `case (a) matches\ntagged ...:`

See also:
- **style_keyword_0or1space** - Suggested companion rule.
- **style_keyword_0space** - Suggested companion rule.
- **style_keyword_1or2space** - Suggested companion rule.
- **style_keyword_1space** - Suggested companion rule.
- **style_keyword_1spaceornewline** - Suggested companion rule.
- **style_keyword_construct** - Suggested companion rule.
- **style_keyword_datatype** - Potential companion rule.
- **style_keyword_end** - Suggested companion rule.
- **style_keyword_indent** - Suggested companion rule.
- **style_keyword_maybelabel** - Suggested companion rule.
- **style_keyword_new** - Suggested companion rule.
- **style_keyword_newline** - Suggested companion rule.
