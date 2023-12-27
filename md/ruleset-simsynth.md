
The set of checks which detect potential mismatches between simulation and
synthesis.

Unlike the rules in, for example, **ruleset-style**, the rules in this ruleset
do not depend on each other or combine to check additional properties.
See the explanations of individual rules for their details.

```toml
syntaxrules.blocking_assignment_in_always_ff = true
syntaxrules.blocking_assignment_in_always_latch = true
syntaxrules.non_blocking_assignment_in_always_comb = true
syntaxrules.case_default = true
syntaxrules.enum_with_type = true
syntaxrules.function_with_automatic = true
syntaxrules.keyword_forbidden_priority = true
syntaxrules.keyword_forbidden_unique = true
syntaxrules.keyword_forbidden_unique0 = true
syntaxrules.general_always_no_edge = true
syntaxrules.operator_case_equality = true
syntaxrules.procedural_continuous_assignment = true
```

