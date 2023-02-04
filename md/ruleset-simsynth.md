
The set of checks which detect potential mismatches between simulation and
synthesis.

Unlike the rules in, for example, **ruleset-style**, the rules in this ruleset
do not depend on each other or combine to check additional properties.
See the explanations of individual rules for their details.

```toml
rules.blocking_assignment_in_always_ff = true
rules.non_blocking_assignment_in_always_comb = true
rules.case_default = true
rules.enum_with_type = true
rules.function_with_automatic = true
rules.keyword_forbidden_priority = true
rules.keyword_forbidden_unique = true
rules.keyword_forbidden_unique0 = true
rules.level_sensitive_always = true
```

