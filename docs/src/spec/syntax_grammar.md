# Syntax grammar

The grammar below replaces some lexical grammar rules with explicit literals (where such replacement in trivial and always correct, for example, for keywords) for better readability.

`NL` is trivia everywhere except:

1. as a statement terminator
2. inside a block before/after stmts.

In all other positions (after binary/prefix operators, after `(` `[` `{` `,`) the parser skips `NL`.

```ebnf
file = [ stmts ] EOF .

stmts      = stmt { terminator stmt } [ terminator ] .
terminator = ( ";" | NL ) { NL }.

stmt = use_stmt
     | decl
     | expr .

use_stmt  = "use" use_tree .
use_tree  = ident [ "::" "*" | "::" use_group | "as" simple_ident ] .
use_group = "{" [ use_tree { "," use_tree } [ "," ] ] "}" .

decl = egg_decl
     | nominal_decl
     | fn_decl
     | bind_decl
     | type_decl
     | extend_decl .

egg_decl = [ vis_mod ] "egg" simple_ident [ egg_body ] .
egg_body = "{" [ stmts ] "}" .

nominal_decl = [ vis_mod ] nominal_kw simple_ident [ generics ] [ bounds ] [ where_clause ] [ nominal_body ] .
nominal_kw   = "enum" | "class" | "trait" .
nominal_body = enum_body | class_body | trait_body .

enum_body = "{" [ enum_ctors ] [ member_decls ] "}" .

fn_decl = [ vis_mod ] "fn" simple_ident [ generics ] param_list [ ret_type ] [ where_clause ] [ context_params ] [ fn_body ] .
fn_body = .

bind_decl = [ vis_mod ] [ bind_mod ] simple_ident [ type ] [ assign_op expr ] .

type_decl = [ vis_mod ] "type" simple_ident [ generics ] assign_op type .

extend_decl = "extend" [ type_params ] ident [ generic_suffix ] [ where_clause ] extend_body .
extend_body = .

expr         = range_expr .
range_expr   = disj_expr [ range_op [ disj_expr ] ]
             | range_op [ disj_expr ] .

disj_expr    = conj_expr { "|" conj_expr } .
conj_expr    = eq_expr { "&" eq_expr } .
eq_expr      = cmp_expr [ eq_op cmp_expr ] .
cmp_expr     = term_expr [ cmp_op term_expr ] .
term_expr    = factor_expr { term_op factor_expr } .
factor_expr  = prefix_expr { factor_op prefix_expr } .
prefix_expr  = prefix_op prefix_expr | postfix_expr .
postfix_expr = primary_expr { call_suffix | index_suffix | field_suffix | generic_suffix } .
primary_expr = block_expr
             | paren_expr
             | lit_expr
             | ref_expr 
             | return_expr .

block_expr = "{" [ stmts ] expr "}" .
paren_expr = "(" expr ")" .
lit_expr   = IntLit 
           | FloatLit
           | "true"
           | "false" .

ref_expr    = simple_ident .
return_expr = "return" expr .

bind_op    = ":=" .
assign_op  = "="
           | "+="
           | "-="
           | "*="
           | "/="
           | "&="
           | "|=" .

range_op   = ".." | "..=" .
eq_op      = "==" | "!=" .
cmp_op     = "<" | ">" | "<=" | ">=" | "<=>" .
term_op    = "+" | "-" .
factor_op  = "*" | "/" | "%" .
prefix_op  = "!" | "-" .

call_suffix    = "(" [ expr { "," expr } [ "," ] ] ")" .
index_suffix   = "[" expr "]" .
field_suffix   = "." simple_ident .
generic_suffix = "::" "[" type { "," type } [ "," ] "]" .

vis_mod = "pub" .
bind_mod = "const" | "mut" .
type_mod = "?" .

ident        = simple_ident { "::" simple_ident } .
simple_ident = Ident | RawIdent .
```
