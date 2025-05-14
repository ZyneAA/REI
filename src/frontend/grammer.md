## Declarations
declaration  → classDecl| funDecl | varDecl | statement ;\\
class_decl   → "class" IDENTIFIER ( "<" IDENTIFIER )? 
               "{" function* "}" ;\
fn_decl      → "fn" function ;\
var_decl     → "let" IDENTIFIER ( "=" expression )? ";" ;\

## Statements
statement    → expr_stmt
              | for_stmt
              | if_stmt
              | print_stmt
              | println_stmt
              | return_stmt
              | while_stmt
              | block ;\
expr_stmt    → expression ";" ;\
for_stmt     → "for" "(" ( var_decl | expr_stmt | ";"
                expression? ";"
                expression? ")" statement ;\
if_stmt      → "if" "(" expression ")" statement
               ( "else" statement )? ;\
print_stmt   → "print" expression ";" ;\
println_stmt → "println" expression ";" ;\
return_stmt  → "return" expression? ";" ;\
while_stmt   → "while" "(" expression ")" statement );\
loop_stmt    → "loop" "(" ver_decl ";" expression | Digit ".." expression | Digit ")"
               statement;\
block        → "{" declaration* "}" ;\

## Expressions
expression   → assignment ;\
assignment   → ( call "." )? IDENTIFIER "=" assignment
              | logic_or ;\
logic_or     → logic_and ( "or" logic_and )* ;\
logic_and    → equality ( "and" equality )* ;\
equality     → comparison ( ( "!=" | "==" ) comparison )* ;\
comparison   → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;\
term         → factor ( ( "-" | "+" ) factor )* ;\
factor       → unary ( ( "/" | "*" ) unary )* ;\
unary        → ( "!" | "-" ) unary | call ;\
call         → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;\
primary      → "true" | "false" | "null" | "base"\
              | NUMBER | STRING | IDENTIFIER\

## Lexical Grammar
Number       → DIGIT+ ( "." DIGIT+ )? ;\
String       → "\"" <any char except "\"">* "\"" ;\
Identifier   → ALPHA ( ALPHA | DIGIT )* ;\
Alpha        → "a" ... "z" | "A" ... "Z" | "_" ;\
Digit        → "0" ... "9" ;\
