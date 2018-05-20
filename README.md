# qw &ndash; query words

## TODO

- coloured matches
- simple query language with keywords `|`, `!`, `(`, `)`, `'`, `"`
- search files and output file names (like rg)
- help text
- proper readme

## Query language sketch

    reserved = "|" | "!" | "(" | ")" | "'" | "\'"

    string = "<arbitrary text other than \">" | '<arbitrary text other than \'>'

    expr = string | !expr | (expr expr) | (expr | expr)

(concatenation is "and") With the usual associativity of and, `|`, and `!` (so
that parens aren't always required), and the rule that quotes are optional when
a string doesn't contain any reserved characters.
