# Syntax grammar

The grammar below replaces some lexical grammar rules with explicit literals (where such replacement is trivial and always correct, for example, for keywords) for better readability.

`NL` is trivia everywhere except:

1. as a statement terminator;
2. inside a block, before/after statements.

In all other positions (after binary/prefix operators, after `(` `[` `{` `,` `::` `|`) the parser skips `NL`.

```ebnf
# File

file = [ stmts ] EOF .

# Statements

stmts      = stmt { terminator stmt } [ terminator ] .
terminator = ( ";" | NL ) { NL } .

stmt = use_stmt
     | decl
     | expr_stmt .

expr_stmt = expr [ assign_op expr ] .

use_stmt  = "use" use_tree .
use_tree  = ident [ "." "*" | "." use_group | "as" simple_ident ] .
use_group = "{" [ use_tree { "," use_tree } [ "," ] ] "}" .

# Declarations

decl = egg_decl
     | nominal_decl
     | fn_decl
     | bind_decl
     | type_decl
     | extend_decl .

egg_decl = [ vis_mod ] "egg" simple_ident [ egg_body ] .
egg_body = "{" [ stmts ] "}" .

nominal_decl = class_decl | trait_decl | enum_decl .

class_decl = [ vis_mod ] [ class_mod ] "class" simple_ident [ generics ] [ bounds ] [ where_clause ] [ member_body ] .
class_mod  = "open" | "abstract" .
trait_decl = [ vis_mod ] "trait" simple_ident [ generics ] [ bounds ] [ where_clause ] [ member_body ] .
enum_decl  = [ vis_mod ] "enum" simple_ident [ generics ] [ bounds ] [ where_clause ] [ enum_body ] .

member_body = "{" [ member_decls ] "}" .
enum_body   = "{" [ enum_ctors ] [ member_decls ] "}" .

enum_ctors   = [ "|" ] enum_ctor { "|" enum_ctor } [ terminator ] .
enum_ctor    = simple_ident [ ctor_payload ] .
ctor_payload = "(" [ type { "," type } [ "," ] ] ")" .

member_decls = member_decl { terminator member_decl } [ terminator ] .
member_decl  = fn_decl | init_decl | bind_decl | type_decl .

init_decl = [ vis_mod ] "init" param_list [ throws ] [ where_clause ] block_expr .

fn_decl = [ vis_mod ] [ "static" ] [ fn_mod ] "fn" simple_ident [ generics ] param_list [ ret_type ] [ throws ] [ where_clause ] [ fn_body ] .
fn_mod  = "override" | "abstract" .
fn_body = block_expr | ":=" expr .

param_list = "(" [ params ] ")" .
params     = param { "," param } [ "," ] .
param      = simple_ident ":" type [ "=" expr ] .

ret_type      = "->" type .
throws        = "!" exception_set .
exception_set = [ "|" ] type { "|" type } .

bind_decl = [ vis_mod ] [ "static" ] [ bind_mod ] simple_ident ( ":" type [ bind_op expr ] | bind_op expr )
          | tuple_pattern bind_op expr .

type_decl = [ vis_mod ] "type" simple_ident [ generics ] "=" type .

extend_decl = "extend" [ generics ] type [ bounds ] [ where_clause ] extend_body .
extend_body = "{" [ member_decls ] "}" .

# Generics, bounds, where

generics      = "[" generic_param { "," generic_param } [ "," ] "]" .
generic_param = simple_ident [ "<:" type_bound ] [ "=" type ] .

bounds     = "<:" type_bound .
type_bound = type { "&" type } .

where_clause = "where" where_pred { "," where_pred } [ "," ] .
where_pred   = type "<:" type_bound .

# Types

type          = optional_type .
optional_type = primary_type { "?" } .
primary_type  = type_path | tuple_type | fn_type .

type_path  = ident [ type_args ] .
type_args  = "[" type { "," type } [ "," ] "]" .
tuple_type = "(" [ type { "," type } [ "," ] ] ")" .
fn_type    = "fn" "(" [ type { "," type } [ "," ] ] ")" "->" type [ throws ] .

# Expressions

expr         = catch_expr .
catch_expr   = range_expr [ "catch" [ simple_ident ] block_expr ] .
range_expr   = or_expr [ range_op [ or_expr ] ]
             | range_op [ or_expr ] .

or_expr      = and_expr  { "|" and_expr } .
and_expr     = cmp_expr  { "&" cmp_expr } .
cmp_expr     = ord_expr  { cmp_op ord_expr } .
ord_expr     = term_expr [ "<=>" term_expr ] .
term_expr    = factor_expr { term_op factor_expr } .
factor_expr  = prefix_expr { factor_op prefix_expr } .
prefix_expr  = prefix_op prefix_expr | postfix_expr .
postfix_expr = primary_expr { call_suffix | trailing_lambda | index_suffix | field_suffix | generic_suffix | try_suffix } .

primary_expr = block_expr
             | lambda_expr
             | paren_expr
             | array_expr
             | lit_expr
             | ref_expr
             | super_expr
             | if_expr
             | match_expr
             | loop_expr
             | while_expr
             | for_expr
             | return_expr
             | break_expr
             | continue_expr
             | throw_expr .

block_expr = "{" [ stmts ] "}" .

lambda_expr   = "{" [ lambda_params ] "=>" [ stmts ] "}" .
lambda_params = lambda_param { "," lambda_param } [ "," ] .
lambda_param  = simple_ident [ ":" type ] .

paren_expr = "(" [ expr { "," expr } [ "," ] ] ")" .
array_expr = "[" [ expr { "," expr } [ "," ] ] "]" .
lit_expr   = IntLit | FloatLit | StringLit | "true" | "false" .
ref_expr   = simple_ident .
super_expr = "super" .

if_expr    = "if" ( pattern ":=" expr | expr ) block_expr
           { "elif" ( pattern ":=" expr | expr ) block_expr }
           [ "else" block_expr ] .
match_expr = "match" expr "{" [ match_arms ] "}" .
loop_expr  = "loop" block_expr .
while_expr = "while" ( pattern ":=" expr | expr ) block_expr .
for_expr   = "for" pattern "in" expr [ "if" expr ] block_expr .

return_expr   = "return" [ expr ] .
break_expr    = "break" [ expr ] .
continue_expr = "continue" .
throw_expr    = "throw" expr .

# Patterns

match_arms = match_arm { terminator match_arm } [ terminator ] .
match_arm  = pattern [ "if" expr ] "=>" expr .

pattern         = or_pattern .
or_pattern      = primary_pattern { "|" primary_pattern } .
primary_pattern = wildcard_pattern
                | literal_pattern
                | tuple_pattern
                | path_pattern .
wildcard_pattern = "_" .
literal_pattern  = lit_expr [ range_op lit_expr ] .
tuple_pattern    = "(" [ pattern { "," pattern } [ "," ] ] ")" .
path_pattern     = ident [ "(" [ pattern { "," pattern } [ "," ] ] ")" ] .

# Suffixes

call_suffix     = "(" [ call_args ] ")" .
call_args       = call_arg { "," call_arg } [ "," ] .
call_arg        = [ simple_ident ":" ] expr .
trailing_lambda = lambda_expr .
index_suffix    = "[" expr "]" .
field_suffix    = "." simple_ident .
generic_suffix  = "::" "[" type { "," type } [ "," ] "]" .
try_suffix      = "?" .

# Operators

bind_op    = ":=" .
assign_op  = "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" .

range_op   = ".." | "..=" .
cmp_op     = "<" | "<=" | ">" | ">=" | "==" | "!=" | "===" .
term_op    = "+" | "-" .
factor_op  = "*" | "/" | "%" .
prefix_op  = "!" | "-" .

# Modifiers

vis_mod  = "pub" .
bind_mod = "const" | "mut" .

# Identifiers

ident        = simple_ident { "." simple_ident } .
simple_ident = Ident | RawIdent .
```
