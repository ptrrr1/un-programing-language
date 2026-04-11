PROGRAM ::= DECLARATION* EOF;

<!-- Declarations  -->
```
DECLARATION ::= VAR_DECL
                | STATEMENT

VAR_DECL ::= "let" IDENTIFIER ":=" EXPR ";"
```


<!-- Statements  -->
```
STATEMENT ::= EXPR_STATEMENT
              | PRINT_STATEMENT
              | BLOCK
              | IF

IF ::= "if" EQUALITY "then" STATEMENT ( "end" | "else" STATEMENT "end" )

BLOCK ::= "begin" DECLARATION* "end" ";"

PRINT_STATEMENT ::= "print""("EXPRESSION ")"";"

EXPR_STATEMENT ::= EXPRESSION";"
```


<!-- Expressions  -->
```
EXPRESSION ::= ASSIGNMENT

ASSIGNMENT ::= IDENTIFIER "=" ASSIGNMENT
              | OR

OR ::= AND ( "or" AND )*

AND ::= EQUALITY ( "and" EQUALITY )*

EQUALITY ::= COMPARISON ( ( "==" | "!=" ) COMPARISON )*

COMPARISION ::= TERM ( ( "<" | "<=" | ">" | ">=" ) TERM )*

TERM ::= FACTOR ( ( "+" | "-" ) FACTOR )*

FACTOR ::= UNARY ( ( "/" | "*" ) UNARY )\*

UNARY ::= ( "not" | "-" ) UNARY | PRIMARY

PRIMARY ::= LITERAL
          | STRING
          | BOOL
          | NIL
          | "(" EXPRESSION ")"
          | IDENTIFIER
          | CONDITIONAL

CONDITIONAL ::= "if" EQUALITY "then" EQUALITY "else" EQUALITY "end"
              | EQUALITY
```

<!--
Precedence (Lowest to Highest)
=
== !=
> >= < <=
- +
/ *
- not
-->
