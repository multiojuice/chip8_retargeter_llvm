# Assignment Title

Brief overview (approx. one paragraph)...

Author(s):

- Name1
- Name2

## Assignment Details

---
---

# [Assignment Design](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#assignment_design) Activity

Design a novel programming assignment suitable for the first 1/3 of the course
(Rust Basics).  The assignment should be a bit larger than
[Programming#01](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog01),
but need not be quite as large as
[Programming#02](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog02),
[Programming#03](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog03),
or
[Programming#04](https://www.cs.rit.edu/~mtf/teaching/20205/psr/assignments.html#prog04),
although larger assignments are often more interesting.

Individual or Pair assignment.  Each individual student and one member of each
pair should complete the [myCourses
survey](https://mycourses.rit.edu/d2l/lms/survey/user/surveys_list.d2l?ou=888966)
by Tue. 03/09 to indicate how the assignment will be undertaken. A Git
repository on [`https://git.cs.rit.edu/psr2205`](https://git.cs.rit.edu/psr2205)
will be created for each individual/pair for the assignment.

The assignment should have the following structure:

- `README.md` or `README.adoc`: assignment writeup, in either
  [Markdown](https://git.cs.rit.edu/help/user/markdown.html) or
  [AsciiDoc](https://git.cs.rit.edu/help/user/asciidoc.html) format as supported
  by [`git.cs.rit.edu`](https://git.cs.rit.edu) (GitLab)
- `soln`: reference solution
  * `Crate.toml` and `Crate.lock`: `[workspace]` definition, if multi-crate
    assignment; _optional_
  * `assets`: directory of shared assets, if multi-crate assignment; _optional_
  * _`crate1`_: first (and possibly only) assignment crate
    + `Crate.toml` and `Crate.lock`: crate manifest
    + `src`: directory of source files, must include `lib.rs` or `main.rs` and
      may include additional files
    + `assets`: directory of crate-level assets; _optional_
  * _`crate2`_, ..., _`craten`_: second and additional crates, with directory
    structure similar to _`crate1`_; _optional
- `starter`: student starter code; should be a copy of the `soln` directory
  except that any code to be written for the assignment has been removed and
  replaced by `unimplemented!()` or equivalent
