# Shunting Yard
I was interested in common parsing strategies for dealing with precedence in
infix expressions.

This is by no means something that should be used and is over complicated. The
`Stack` and `Queue` implementations are unnecessary.

This currently assumes the expression to be parsed does not contain
delimiters, however it should be easy to add in `should_stack`. A more
complete version can be found in further projects.
