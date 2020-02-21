# Change Log

## [Unreleased](https://github.com/dalance/svlint/compare/v0.4.4...Unreleased) - ReleaseDate

## [v0.4.4](https://github.com/dalance/svlint/compare/v0.4.3...v0.4.4) - 2020-02-21

* [Changed] Update verilog-filelist-parser

## [v0.4.3](https://github.com/dalance/svlint/compare/v0.4.2...v0.4.3) - 2020-02-20

* [Added] --github-actions option

## [v0.4.2](https://github.com/dalance/svlint/compare/v0.4.1...v0.4.2) - 2020-02-19

* [Added] generate_keyword_required rule
* [Added] genvar_declaration_out_loop rule
* [Added] Check for unknown rules
* [Added] Check rule renaming
* [Breaking Changed] Rename from generate_keyword to generate_keyword_forbidden
* [Breaking Changed] Rename from genvar_declaration to genvar_declaration_in_loop

## [v0.4.1](https://github.com/dalance/svlint/compare/v0.4.0...v0.4.1) - 2020-02-19

## [v0.4.0](https://github.com/dalance/svlint/compare/v0.3.3...v0.4.0) - 2020-02-19

* [Changed] Update sv-parser
* [Changed] Rule interface to support statefule rule
* [Changed] Improve error print

## [v0.3.3](https://github.com/dalance/svlint/compare/v0.3.2...v0.3.3) - 2020-02-09

* [Changed] Update sv-parser

## [v0.3.2](https://github.com/dalance/svlint/compare/v0.3.1...v0.3.2) - 2020-01-28

* [Fixed] parse error with -1 option [#11](https://github.com/dalance/svlint/issues/11)

## [v0.3.1](https://github.com/dalance/svlint/compare/v0.3.0...v0.3.1) - 2020-01-27

* [Added] --ignore-include option [#10](https://github.com/dalance/svlint/issues/10)
* [Changed] --define can have value like `--define NAME=VALUE` [#9](https://github.com/dalance/svlint/issues/9)

## [v0.3.0](https://github.com/dalance/svlint/compare/v0.2.21...v0.3.0) - 2020-01-23

* [Changed] Update sv-parser and Migrate to anyhow

## [v0.2.21](https://github.com/dalance/svlint/compare/v0.2.20...v0.2.21) - 2020-01-22

* [Changed] Update sv-parser

## [v0.2.20](https://github.com/dalance/svlint/compare/v0.2.19...v0.2.20) - 2019-12-12

* [Changed] Update sv-parser

## [v0.2.19](https://github.com/dalance/svlint/compare/v0.2.18...v0.2.19) - 2019-12-11

* [Added] Environment variable with paren in filelist

## [v0.2.18](https://github.com/dalance/svlint/compare/v0.2.17...v0.2.18) - 2019-12-03

## [v0.2.17](https://github.com/dalance/svlint/compare/v0.2.16...v0.2.17) - 2019-12-02

* [Changed] Update sv-parser

## [v0.2.16](https://github.com/dalance/svlint/compare/v0.2.15...v0.2.16) - 2019-11-26

* [Changed] Update sv-parser

## [v0.2.15](https://github.com/dalance/svlint/compare/v0.2.14...v0.2.15) - 2019-11-25

* [Changed] Enable all rules when .svlint.toml is not found

## [v0.2.14](https://github.com/dalance/svlint/compare/v0.2.13...v0.2.14) - 2019-11-15

## [v0.2.13](https://github.com/dalance/svlint/compare/v0.2.12...v0.2.13) - 2019-11-14

* [Added] interface_port_with_modport rule

## [v0.2.12](https://github.com/dalance/svlint/compare/v0.2.11...v0.2.12) - 2019-11-13

## [v0.2.11](https://github.com/dalance/svlint/compare/v0.2.10...v0.2.11) - 2019-11-13

## [v0.2.10](https://github.com/dalance/svlint/compare/v0.2.9...v0.2.10) - 2019-11-13

* [Added] plugin support

## [v0.2.9](https://github.com/dalance/svlint/compare/v0.2.8...v0.2.9) - 2019-11-12

* [Added] '+define' in filelist support

## [v0.2.8](https://github.com/dalance/svlint/compare/v0.2.7...v0.2.8) - 2019-11-08

* [Fixed] environment variable in +incdir/-f
* [Fixed] --include is ignored when --filelist exists

## [v0.2.7](https://github.com/dalance/svlint/compare/v0.2.6...v0.2.7) - 2019-11-08

* [Added] '+incdir'/'-f' in filelist support
* [Fixed] --version string
* [Fixed] space/tab in filelist

## [v0.2.6](https://github.com/dalance/svlint/compare/v0.2.5...v0.2.6) - 2019-11-07

* [Added] environment variable expansion in filelist
* [Changed] multiple filelist support

## [v0.2.5](https://github.com/dalance/svlint/compare/v0.2.4...v0.2.5) - 2019-11-06

* [Added] case_default / function_same_as_system_function rule
* [Added] --update option
* [Changed] update sv-parser to 0.3.7

## [v0.2.4](https://github.com/dalance/svlint/compare/v0.2.3...v0.2.4) - 2019-11-05

* [Changed] update sv-parser to 0.3.4 ( #5 )

## [v0.2.3](https://github.com/dalance/svlint/compare/v0.2.2...v0.2.3) - 2019-11-01

* [Added] reason of rules

## [v0.2.2](https://github.com/dalance/svlint/compare/v0.2.1...v0.2.2) - 2019-11-01

## [v0.2.1](https://github.com/dalance/svlint/compare/v0.2.0...v0.2.1) - 2019-11-01

* [Added] level_sensitive_always rule
* [Changed] auto generatoin of rule's mod/config
* [Changed] update sv-parser to 0.3.3

## [v0.2.0](https://github.com/dalance/svlint/compare/v0.1.0...v0.2.0) - 2019-10-30

* [Added] non_ansi_module rule
* [Changed] all rules are disabled by default
