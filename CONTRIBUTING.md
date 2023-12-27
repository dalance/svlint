Contributing To Svlint
======================

Thanks for using svlint!
We welcome your contributions in whatever form.

This contributing document contains some suggestions that may make
contributions flow more efficiently.


Did you find a bug?
-------------------

- Please **ensure the bug was not already reported** by searching
  [GitHub Issues](https://github.com/dalance/svlint/issues).
- Please **download the latest development GitHub version**, build, and see
  if the issue has been fixed.
- If you're unable to find an open issue addressing the problem, open a new
  [Issue](https://github.com/dalance/svlint/issues).
- Be sure to include a **code sample** demonstrating the bug and expected
  behavior that is not occurring.


Did you write a patch that fixes a bug?
---------------------------------------

- Please open a new [Pull Request](https://github.com/dalance/svlint/pulls).
- Stick to the coding conventions of surrounding code.
  If in doubt, use `rustfmt`.
- Svlint uses GitHub Actions to provide continuous integration.
  You may want to enable Actions on your GitHub branch to ensure your changes
  keep the tests passing.
- To check your changes locally, cd to the root of the svlint repository and
  execute:
  - `cargo test`: Should report that all tests are ok and exit with code `0`.
    This will be run by the GitHub Action
   [`.github/workflows/regression.yml`](https://github.com/dalance/svlint/blob/master/.github/workflows/regression.yml)
  - `cargo run --bin=mdgen`: Should exit with code `0` and all changes
    reported by `git status` should be committed.
    This will be run by the GitHub Action
   [`.github/workflows/mdgen.yml`](https://github.com/dalance/svlint/blob/master/.github/workflows/mdgen.yml)
  - (optional) `make MANUAL-dev`: Should generate `MANUAL-dev.pdf` and exit
    with code `0`.
    Requires [pandoc](https://pandoc.org/MANUAL.html) to be installed locally.
- Your source-code contributions must be certified as open source, under the
  [Developer Certificate of Origin](https://developercertificate.org/).
  On your first contribution, please add your name to the list of contributors
  at the end of this file.


Adding A New Rule
-----------------

1. Decide on a descriptive name, and use that instead of the `$RULENAME`
  placeholder in this list of steps.
2. Write a short description in Markdown about what the new rule checks and why
  it might be used.
  Write this in `md/(text|syntax)rules-explanation-$RULENAME.md`, preferably
  keeping a similar format to similar existing rules.
3. Write at least one testcase which the new rule passes in
  `testcases/(text|syntax)rules/pass/$RULENAME.sv`.
  If you have more than one testcase which must pass, they should all go into
  the same file but be separated by a comment line of 80 forward slashes like
  `////////////////////////////////////////////////////////////////////////////////`.
4. Write at least one testcase which the new rule fails in
  `testcases/(text|syntax)rules/fail/$RULENAME.sv`.
  Again, you can separate multiple testcases with a comment line of 80 forward
  slash characters.
  For an example, see
  [`testcases/syntaxrules/fail/generate_case_with_label.sv`](https://github.com/dalance/svlint/blob/master/testcases/syntaxrules/fail/generate_case_with_label.sv).
5. Implement the rule in `src/rules/$RULENAME.rs`.
  This includes writing a short hint and reason to be displayed to the user.
  - Both the hint and reason should be as short as possible (maximum 80
    characters), to display nicely in text editors which use
    [svls](https://github.com/dalance/svls).
  - Hint should be a command telling the user *what to do* to pass the rule.
  - Reason should be a 1-sentence summary of *why the rule exists*.
6. Test the implementation using `cargo test`.
  You should see all tests passing as "ok".
7. Update any relevant rulesets by editing `md/ruleset-*.md`.
8. Regenerate documentation and ruleset files using `cargo run --bin=mdgen`.
  All modified files should be commited, notably `MANUAL.md` and `rulesets/*`.
9. Ensure all work is on a dedicated branch of your GitHub fork, and that
  you can see GitHub Actions passing
  (`https://github.com/$USERNAME/svlint/actions`).
10. Open a [Pull Request](https://github.com/dalance/svlint/pulls) by comparing
  your branch to dalance's master branch
  (`https://github.com/dalance/svlint/compare/master...$USERNAME:svlint:$BRANCHNAME`).


Related Projects
----------------

Svlint depends on several (upstream) projects which are listed definitively in
`Cargo.toml`, but 2 are notable when considering changes to svlint:

- [sv-parser](https://docs.rs/sv-parser/latest/sv_parser/), developed mainly
  by @dalance:
  Preprocess and parse SystemVerilog source files into the Rust structure
  [`SyntaxTree`](https://docs.rs/sv-parser/latest/sv_parser/struct.SyntaxTree.html),
  sticking very closely to the
  [BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) given in
  IEEE 1800-2017 Annex A.
- [sv-filelist-parser](https://github.com/supleed2/sv-filelist-parser),
  developed initially by @Raamakrishnan and extended by @supleed2:
  Parse filelist files into the Rust structure
  [`Filelist`](https://github.com/supleed2/sv-filelist-parser/blob/main/src/file_parser.rs#L12).

Svlint is also depended upon by several (downstream) projects:

- [svls](https://github.com/dalance/svls):
  An [LSP](https://en.wikipedia.org/wiki/Language_Server_Protocol) server for
  integrating svlint directly into text editors.
- [svls-vscode](https://github.com/dalance/svls-vscode):
  A VSCode
  [extension](https://marketplace.visualstudio.com/items?itemName=dalance.svls-vscode)
  that integrates svls.
- [svlint-plugin-sample](https://github.com/dalance/svlint-plugin-sample):
  A working example of how to make an svlint plugin, i.e. a separately compiled
  dynamically loaded binary object that implements a collection of rules.
  Mostly useful for people who need specific rules which be cannot shared
  publically.
- [svlint-action](https://github.com/dalance/svlint-action):
  A [GitHub Action](https://docs.github.com/en/actions) which can be used in a
  [CI/CD](https://en.wikipedia.org/wiki/CI/CD) workflow in SystemVerilog
  projects developed on GitHub.
- [svlint\_installer](https://github.com/DaveMcEwan/svlint_installer):
  An installer for organisations that wish to build/install svlint and svls
  from source with
  [environment modules](https://modules.readthedocs.io/en/latest/index.html).
  Mostly useful for professional SystemVerilog developers.


Contributors
------------

The contributors listed below have certified their svlint contributions
under the Developer Certificate of Origin <https://developercertificate.org/>.

- Naoya Hatta (@dalance)
- David McEwan (@DaveMcEwan)
- Aadi Desai (@supleed2)
- A. Raamakrishnan (@Raamakrishnan)
- Thomas Heuschling (@skjdbg)
- Andreas K. Berg (@akberg)
- Damien Pretet (@dpretet)
- Taichi Ishitani (@taichi-ishitani)
- Sosuke Hosokawa (@so298)
- Jan Remes (@remes-codasip)
