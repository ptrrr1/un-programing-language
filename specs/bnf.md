PROGRAM ::= DECLARATION* EOF;

<!-- Declarations  -->
```
DECLARATION ::= FUNC_DECL
              | VAR_DECL
              | STATEMENT

FUNC_DECL ::= "fn" FUNCTION

FUNCTION ::= IDENTIFIER "(" PARAMETERS?  ")" DECLARATION* "end"

PARAMETERS ::= IDENTIFIER ( "," IDENTIFIER )*

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
              | RETURN
              | BREAK
              | CONTINUE

CONTINUE ::= "continue" ";"

BREAK ::= "break" ";"

RETURN ::= "return" LAMBDA? ";"

FOR ::= "for" IDENTIFIER "in" "[" OR ".." ( "<" | ">" )  OR ( ";" OR )? "]" "do" DECLARATION* "end"

WHILE ::= "while" OR "do" DECLARATION* "end"

IF ::= "if" OR "then" DECLARATION* ( "end" | "else" DECLARATION* "end" )

BLOCK ::= "begin" DECLARATION* "end" ";"

PRINT_STATEMENT ::= "print""(" OR ")"";"

EXPR_STATEMENT ::= EXPRESSION";"
```


<!-- Expressions  -->
```
EXPRESSION ::= ASSIGNMENT

ASSIGNMENT ::= IDENTIFIER "=" ASSIGNMENT
              | LAMBDA

LAMBDA ::= "fn" "(" PARAMETERS?  ")" LAMBDA
              | OR

OR ::= AND ( "or" AND )*

AND ::= EQUALITY ( "and" EQUALITY )*

EQUALITY ::= COMPARISON ( ( "==" | "!=" ) COMPARISON )*

COMPARISION ::= TERM ( ( "<" | "<=" | ">" | ">=" ) TERM )*

TERM ::= FACTOR ( ( "+" | "-" ) FACTOR )*

FACTOR ::= UNARY ( ( "/" | "*" ) UNARY )\*

UNARY ::= ( "not" | "-" ) UNARY | CALL

CALL ::= PRIMARY ( "(" ARGUMENTS?  ")" )*

ARGUMENTS ::= OR ( "," OR )*

PRIMARY ::= LITERAL
          | STRING
          | BOOL
          | NIL
          | "(" EXPRESSION ")"
          | IDENTIFIER
          | CONDITIONAL

CONDITIONAL ::= "if" OR "then" LAMBDA  "else" LAMBDA "end"
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
