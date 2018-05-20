# qw &ndash; query words

`qw` is a *grep-ish* tool for filtering text with a simple query language.
Unlike grep, queries are boolean expressions based on the occurances (or not)
of given strings. `qw` is strictly less flexible than grep and friends but
(hopefully) sometimes more convenient, since writing these kind of expressions
using regular expressions sucks.

## Examples

To find occurances of a literal string (wrap in quotes if your string contains
spaces or any of `|!()'"`)

    qw foo

In `qw`, *and* is concatenation, so

    qw 'foo bar'

matches any line containing both "foo" and "bar". (to match the literal string
"foo bar", use `qw '"foo bar"'`)

*or* is `|`, so

    qw 'foo | bar'

matches any line containing "foo" or "bar".

    qw '!foo'

matches any line *not* containing "foo".

Finally, anywhere that you can put a string, you can also put a whole
subexperssion, building arbitrarily complex queries. `!` takes precedence over
` ` (and), which takes precedence over `|`, and `()` can be used for grouping
in the usual way.

    qw '!foo (bar | baz)'

matches any line which doesn't contain "foo", but does contain either "bar" or
"baz".
