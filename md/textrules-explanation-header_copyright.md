Check that a file contains a copyright header something like this:

```
// Copyright (c) 1984 George Orwell
```

The format is specified with a regular expression and 3 parameters:

- `copyright_linenum`: Line number that must contain the copyright notice,
  beginning at number 1, like you normally see in a text editor.
- `copyright_year`: String containing the year(s) the work was created.
  In the above example, that would be "1984".
- `copyright_holder`: String containing the name(s) of the copyright holder(s).
  In the above example, that would be "George Orwell" .

The regex allows for simple permutations such as C-style comments and using
uppercase.

See also:
- <https://en.wikipedia.org/wiki/Copyright_notice>
- <https://en.wikipedia.org/wiki/MIT_License>
