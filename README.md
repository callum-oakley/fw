# fw &ndash; find words

## TODO

- coloured matches
- simple query language with keywords `&&`, `||`, `!`, `(`, `)`, `'`, `"`
- help text
- proper readme

## Query language sketch

    reserved = "&" | "|" | "!" | "(" | ")" | "'" | "\'"

    string = "<arbitrary text other than \">" | '<arbitrary text other than \'>'

    expr = string | !expr | (expr && expr) | (expr || expr)

With the usual associativity of `&&`, `||`, and `!` (so that parens aren't
always required), and the rule that quotes are optional when a string doesn't
contain any reserved characters.

e.g.

    fw 'fox && jumps over || the lazy && !smelly cat && dog'

is equivalent to

    fw '(("fox" && "jumps over") || (("the lazy" && !"smelly cat") && "dog"))'

So the syntax, pre-expansion, is actually

    string = <arbitrary non-reserved text> | "<as above>" | '<as above>'

    expr = string | !expr | (expr) | expr && expr | expr || expr
