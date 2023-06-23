// This file must pass all rulesets.
// In the GitHub Actions CI, this file is checked against all files in
// `rulesets/*.toml` to ensure that they contain valid TOML configurations.
// Perform this check locally like:
//    for f in rulesets/*.toml; do
//      SVLINT_CONFIG="$r" cargo run -- \
//        testcases/application/resources/must_pass_all_rulesets.sv
//    done
