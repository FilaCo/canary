# Syntax grammar

The grammar below replaces some lexical grammar rules with explicit literals (where such replacement in trivial and always correct, for example, for keywords) for better readability.

```ebnf
file = { NL } [ stmts ] EOF .

stmts      = stmt { terminator stmt } [ terminator ] .
terminator = ( ";" | NL ) { NL } .

stmt = use_stmt
     | decl
     | expr .

use_stmt  = "use" use_tree .
use_tree  = ident [ "::" "*" | "::" use_group | "as" simple_ident ] .
use_group = "{" [ use_tree { "," use_tree } [ "," ] ] "}" .

decl = egg_decl
     | trait_decl
     | class_decl
     | struct_decl
     | enum_decl
     | fn_decl
     | binding_decl
     | type_alias_decl
     | extend_decl .

egg_decl = "egg" simple_ident [ [ NL ] egg_body ] .
egg_body = "{" { NL } { stmt semi } { NL } "}" .

trait_decl = [ vis_mod ] "trait" simple_ident .

class_decl = [ vis_mod ] "class" simple_ident .

binding_stmt = expr assign_op expr .

expr         = range_expr .
range_expr   = disj_expr [ range_op [ disj_expr ] ]
             | range_op [ disj_expr ] .

disj_expr    = conj_expr { "|" { NL } conj_expr } .
conj_expr    = eq_expr { "&" { NL } eq_expr } .
eq_expr      = cmp_expr [ eq_op { NL } cmp_expr ] .
cmp_expr     = term_expr [ cmp_op { NL } term_expr ] .
term_expr    = factor_expr { term_op { NL } factor_expr } .
factor_expr  = prefix_expr { factor_op { NL } prefix_expr } .
prefix_expr  = prefix_op prefix_expr | postfix_expr .
postfix_expr = primary_expr { call_suffix | index_suffix | field_suffix | generic_suffix } .
primary_expr = paren_expr
             | lit_expr .

paren_expr = "(" { NL } expr { NL } ")" .
lit_expr   = IntLit 
           | FloatLit
           | "true"
           | "false" .

range_op   = ".." | "..=" .
eq_op      = "==" | "!=" .
cmp_op     = "<" | ">" | "<=" | ">=" | "<=>" .
term_op    = "+" | "-" .
factor_op  = "*" | "/" | "%" .
prefix_op  = "!" | "-" .

call_suffix    = "(" { NL } [ expr { "," expr } [ "," ] ] ")" .
index_suffix   = "[" { NL } expr { NL } "]" .
field_suffix   = "." simple_ident .
generic_suffix = "::" "[" { NL } type { "," { NL } type } [ "," ] "]" .

type = ident [ "[" { NL } type { "," { NL } type } [ "," ] "]" ] .

vis_mod = "pub" .

ident        = simple_ident { "::" simple_ident } .
simple_ident = Ident | RawIdent .

```
