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
              | WHILE
              | FOR

FOR ::= "for" IDENTIFIER "in" "[" OR ".." ( "<" | ">" )  OR ( ";" OR )? "]" "do" DECLARATION* "end"

WHILE ::= "while" OR "do" DECLARATION* "end"

IF ::= "if" EQUALITY "then" DECLARATION* ( "end" | "else" DECLARATION* "end" )

BLOCK ::= "begin" DECLARATION* "end" ";"

PRINT_STATEMENT ::= "print""(" OR ")"";"

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

CONDITIONAL ::= "if" OR "then" OR "else" OR "end"
              | OR
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
