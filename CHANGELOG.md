# Change Log

## [Unreleased](https://github.com/dalance/svlint/compare/v0.9.3...Unreleased) - ReleaseDate

## [v0.9.3](https://github.com/dalance/svlint/compare/v0.9.2...v0.9.3) - 2024-06-03

* [Added] New rule that ensures module/interface/package/program identifier matches the filename it's in [#283](https://github.com/dalance/svlint/pull/283)
* [Added] New Rule: Implicit case default [#281](https://github.com/dalance/svlint/pull/281)
* [Added] New Rule: Forbid unpacked array declarations [#280](https://github.com/dalance/svlint/pull/280)

## [v0.9.2](https://github.com/dalance/svlint/compare/v0.9.1...v0.9.2) - 2023-12-27

* [Changed] Update sv-parser
* [Added] New rule procedural_continuous_assignment [#276](https://github.com/dalance/svlint/pull/276)

## [v0.9.1](https://github.com/dalance/svlint/compare/v0.9.0...v0.9.1) - 2023-11-20

* [Added] New rule: package_item_not_in_package [#272](https://github.com/dalance/svlint/pull/272)
* [Added] Feature: Include files and incdirs from the environment. [#271](https://github.com/dalance/svlint/pull/271)
* [Added] New rules for compatibility with Verilog 2001 [#270](https://github.com/dalance/svlint/pull/270)
* [Added] Improvement: Add description of related projects to CONTRIBUTING. [#265](https://github.com/dalance/svlint/pull/265)

## [v0.9.0](https://github.com/dalance/svlint/compare/v0.8.0...v0.9.0) - 2023-08-09

* [Added] New rules for leading spaces of binary operators [#263](https://github.com/dalance/svlint/pull/263)
* [Breaking Changed] Bugfix: Change textrules API to pass line: Option<&str>. [#261](https://github.com/dalance/svlint/pull/261)
* [Changed] Improvement: Bundle rulesets in release ZIP files. [#260](https://github.com/dalance/svlint/pull/260)
* [Added] Feature: Provide wrapper scripts for svls, in addition to svlint. [#259](https://github.com/dalance/svlint/pull/259)
* [Changed] Improvement: Cleanup the command-line interface. [#258](https://github.com/dalance/svlint/pull/258)

## [v0.8.0](https://github.com/dalance/svlint/compare/v0.7.2...v0.8.0) - 2023-06-26

* [Breaking Changed] Feature: Update plugin API. [#255](https://github.com/dalance/svlint/pull/255)
* [Added] Improvement: Add 2 examples of user-specific rulesets. [#254](https://github.com/dalance/svlint/pull/254)
* [Changed] Improvement Add document CONTRIBUTING.md, based on Verilator's one. [#250](https://github.com/dalance/svlint/pull/250)
* [Added] Feature: Textrules [#247](https://github.com/dalance/svlint/pull/247)
* [Changed] textrules-move Move "rules" to "syntaxrules" in preparation for adding textrules [#246](https://github.com/dalance/svlint/pull/246)

## [v0.7.2](https://github.com/dalance/svlint/compare/v0.7.1...v0.7.2) - 2023-05-02

* [Fixed] Bugfix: insert plugins in enabled hashmap during load [#243](https://github.com/dalance/svlint/pull/243)
* [Added] New Rules: Addressing 4 of the outstanding new-rule issues. [#242](https://github.com/dalance/svlint/pull/242)

## [v0.7.1](https://github.com/dalance/svlint/compare/v0.7.0...v0.7.1) - 2023-03-23

* [Changed] Update sv-parser

## [v0.7.0](https://github.com/dalance/svlint/compare/v0.6.1...v0.7.0) - 2023-03-22

* [Fixed] Bugfix: Addresses #210, #225, #235. [#238](https://github.com/dalance/svlint/pull/238)
* [Added] Feature: PDF version(s) of the manual. [#236](https://github.com/dalance/svlint/pull/236)
* [Changed] Improvement: Don't emit control codes to STDOUT for non-colored output. [#229](https://github.com/dalance/svlint/pull/229)
* [Changed] Improvement: Nicer error message on file of invalid UTF-8. [#227](https://github.com/dalance/svlint/pull/227)
* [Added] Feature: Add some pre-configured and explained "rulesets" (TOML files) with wrapper scripts. [#224](https://github.com/dalance/svlint/pull/224)
* [Changed] Improvement: Split testcase examples in documentation. [#220](https://github.com/dalance/svlint/pull/220)
* [Changed] Improvement: Update mdgen and its workflow. [#215](https://github.com/dalance/svlint/pull/215)

## [v0.6.1](https://github.com/dalance/svlint/compare/v0.6.0...v0.6.1) - 2022-12-16

* [Changed] Distinguish between parse/preprocess errors. Bump sv-parser. [#196](https://github.com/dalance/svlint/pull/196)
* [Added] New Rule: action_block_with_side_effect [#203](https://github.com/dalance/svlint/pull/203)
* [Added] New Rule: parameter_in_generate, similar to existing rule parameter_in_package. [#202](https://github.com/dalance/svlint/pull/202)
* [Added] New Rules: style_trailingwhitespace, style_operator_(arithmetic|boolean|integer|unary) [#201](https://github.com/dalance/svlint/pull/201)
* [Added] New Option: --dump-syntaxtree [#204](https://github.com/dalance/svlint/pull/204)
* [Changed] Improvement: Testcase consistency and split tests. [#205](https://github.com/dalance/svlint/pull/205)
* [Changed] Improvement: Simplify build.rs [#212](https://github.com/dalance/svlint/pull/212)
* [Fixed] Bugfix/Improvement: Unittests for --dump-filelist, and fix two bugs. [#214](https://github.com/dalance/svlint/pull/214)
* [Fixed] Bugfix: Correct keyword regex used by style_keyword_*. [#213](https://github.com/dalance/svlint/pull/213)
* [Added] New Rules: Regex Naming Convention [#195](https://github.com/dalance/svlint/pull/195)

## [v0.6.0](https://github.com/dalance/svlint/compare/v0.5.6...v0.6.0) - 2022-11-07

* [Changed] Relax style rule on assign keyword. [#191](https://github.com/dalance/svlint/pull/191)
* [Changed] Add fuller explanations about each rule. [#170](https://github.com/dalance/svlint/pull/170)
* [Fixed] Bugfix #182 explicit_if_else, explicit_case_default [#197](https://github.com/dalance/svlint/pull/197)

## [v0.5.6](https://github.com/dalance/svlint/compare/v0.5.5...v0.5.6) - 2022-08-01

* [Fixed] renamed rules in example [#180](https://github.com/dalance/svlint/issues/180)
* [Changed] Update sv-parser

## [v0.5.5](https://github.com/dalance/svlint/compare/v0.5.4...v0.5.5) - 2022-07-06

* [Added] Print preprocessor output to STDOUT with -E. [#175](https://github.com/dalance/svlint/pull/175)

## [v0.5.4](https://github.com/dalance/svlint/compare/v0.5.3...v0.5.4) - 2022-06-07

* [Changed] Swap verilog_filelist_parser [#171](https://github.com/dalance/svlint/pull/171)
* [Added] Silent flag silences .toml not found message [#172](https://github.com/dalance/svlint/pull/172)
* [Added] Add rules to check for whitespace/style consistency [#174](https://github.com/dalance/svlint/pull/174)

## [v0.5.3](https://github.com/dalance/svlint/compare/v0.5.2...v0.5.3) - 2022-05-12

* [Fixed] control comment is not checked in linter.check

## [v0.5.2](https://github.com/dalance/svlint/compare/v0.5.1...v0.5.2) - 2022-04-07

* [Added] take configuration file from environment variable [#137](https://github.com/dalance/svlint/pull/137)
* [Added] new flag --dump-filelist to help debug filelist issues [#138](https://github.com/dalance/svlint/pull/138)
* [Fixed] fixes for errors in prefix_\* [#147](https://github.com/dalance/svlint/pull/147)
* [Changed] apply prefix_module only to module declarations [#155](https://github.com/dalance/svlint/pull/155)

## [v0.5.1](https://github.com/dalance/svlint/compare/v0.5.0...v0.5.1) - 2022-02-08

* [Added] sequential_block_in_always_\* rules [#119](https://github.com/dalance/svlint/pull/119)
* [Added] parameter type rules [#121](https://github.com/dalance/svlint/pull/121)
* [Added] explicit default/else rules [#125](https://github.com/dalance/svlint/pull/125)

## [v0.5.0](https://github.com/dalance/svlint/compare/v0.4.18...v0.5.0) - 2022-01-18

* [Breaking Changed] `prefix_label` in option is `"l_"` by default. To check only that a label exists, set this to `""`.
* [Added] prefix_\* rules [#112](https://github.com/dalance/svlint/pull/112)
* [Added] \*camelcase\* rules [#112](https://github.com/dalance/svlint/pull/112)
* [Changed] Replace structopt with clap

## [v0.4.18](https://github.com/dalance/svlint/compare/v0.4.17...v0.4.18) - 2021-03-05

* [Changed] Update sv-parser

## [v0.4.17](https://github.com/dalance/svlint/compare/v0.4.16...v0.4.17) - 2021-03-04

* [Added] blocking_assignment_in_always_ff rule [#57](https://github.com/dalance/svlint/issues/57)
* [Added] non_blocking_assignment_in_always_comb rule [#57](https://github.com/dalance/svlint/issues/57)
* [Added] default_nettype_none rule [#20](https://github.com/dalance/svlint/issues/20)

## [v0.4.16](https://github.com/dalance/svlint/compare/v0.4.15...v0.4.16) - 2021-01-29

* [Changed] Update sv-parser

## [v0.4.15](https://github.com/dalance/svlint/compare/v0.4.14...v0.4.15) - 2021-01-29

* [Fixed] multiline detection

## [v0.4.14](https://github.com/dalance/svlint/compare/v0.4.13...v0.4.14) - 2021-01-29

* [Fixed] multiline detection

## [v0.4.13](https://github.com/dalance/svlint/compare/v0.4.12...v0.4.13) - 2021-01-28

* [Changed] Obsolete rules return error status
* [Changed] Update sv-parser

## [v0.4.12](https://github.com/dalance/svlint/compare/v0.4.11...v0.4.12) - 2021-01-08

* [Changed] Update sv-parser

## [v0.4.11](https://github.com/dalance/svlint/compare/v0.4.10...v0.4.11) - 2021-01-08

* [Changed] Update sv-parser

## [v0.4.10](https://github.com/dalance/svlint/compare/v0.4.9...v0.4.10) - 2021-01-05

* [Changed] Update sv-parser

## [v0.4.9](https://github.com/dalance/svlint/compare/v0.4.8...v0.4.9) - 2020-12-24

* [Changed] Update sv-parser

## [v0.4.8](https://github.com/dalance/svlint/compare/v0.4.7...v0.4.8) - 2020-11-25

## [v0.4.7](https://github.com/dalance/svlint/compare/v0.4.6...v0.4.7) - 2020-04-03

* [Changed] Update sv-parser

## [v0.4.6](https://github.com/dalance/svlint/compare/v0.4.5...v0.4.6) - 2020-03-23

* [Fixed] typo of reason [#17](https://github.com/dalance/svlint/pull/17)

## [v0.4.5](https://github.com/dalance/svlint/compare/v0.4.4...v0.4.5) - 2020-03-13

* [Changed] Update sv-parser

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
