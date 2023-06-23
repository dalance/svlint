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
  Write this in `md/explanation-$RULENAME.md`, preferably keeping a similar
  format to similar existing rules.
3. Write at least one testcase which the new rule passes in
  `testcases/pass/$RULENAME.sv`.
  If you have more than one testcase which must pass, they should all go into
  the same file but be separated by a comment line of 80 forward slashes like
  `////////////////////////////////////////////////////////////////////////////////`.
4. Write at least one testcase which the new rule fails in
  `testcases/fail/$RULENAME.sv`.
  Again, you can separate multiple testcases with a comment line of 80 forward
  slash characters.
  For an example, see
  [`testcases/fail/generate_case_with_label.sv`](https://github.com/dalance/svlint/blob/master/testcases/fail/generate_case_with_label.sv).
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
