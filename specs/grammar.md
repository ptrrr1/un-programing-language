expression ::= literal | unary | binary | grouping ;

literal ::= NUMBER | STRING | BOOL | NIL ;

grouping ::= "(" expression ")" ;

unary ::= ( "-" | "not" ) expression ;

binary ::= expression operator expression ;

operator ::= "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
