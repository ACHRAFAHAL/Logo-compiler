use santiago::grammar::Grammar;
use santiago::lexer::LexerRules;

use crate::ast::Ast;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // Movement commands
        "DEFAULT" | "FORWARD"   = string "forward"  ;
        "DEFAULT" | "BACKWARD"  = string "backward" ;
        "DEFAULT" | "LEFT"      = string "left"      ;
        "DEFAULT" | "RIGHT"     = string "right"     ;
        "DEFAULT" | "PENUP"     = string "penup";
        "DEFAULT" | "PENDOWN"   = string "pendown";
        "DEFAULT" | "REPEAT"    = string "repeat";
        "DEFAULT" | "LBRACKET"  = string "[";
        "DEFAULT" | "RBRACKET"  = string "]";

        // Numbers
        "DEFAULT" | "NUMBER"    = pattern r"[0-9]+";

        // Spaces, tabulations and line breaks are ignored
        "DEFAULT" | "WS"        = pattern r"\s+" => |lexer| lexer.skip();
    )
}

pub fn grammar() -> Grammar<Ast> {
    santiago::grammar!(
        // <program> ::= <command> <program> or empty
        "program" => rules "command" "program" => Ast::Program;
        "program" => empty => |_| Ast::Program(vec![]);

        // <command> ::= <order> <number> or <state> or <loop> or <block>
        "command" => rules "order" "number" => Ast::Command;
        "command" => rules "state"          => Ast::Command;
        "command" => rules "loop"           => Ast::Command;
        "command" => rules "block"          => Ast::Command;

        // <order> ::= "forward" or "backward" or "left" or "right"
        "order" => lexemes "FORWARD"  => |_| Ast::Forward;
        "order" => lexemes "BACKWARD" => |_| Ast::Backward;
        "order" => lexemes "LEFT"     => |_| Ast::Left;
        "order" => lexemes "RIGHT"    => |_| Ast::Right;

        // <state> ::= "penup" or "pendown"
        "state" => lexemes "PENUP"   => |_| Ast::PenUp;
        "state" => lexemes "PENDOWN" => |_| Ast::PenDown;

        // <loop> ::= "repeat" <number> <block>
        "loop" => rules "kw_repeat" "number" "block" => |nodes| {
            Ast::Loop(vec![nodes[1].clone(), nodes[2].clone()])
        };
        "kw_repeat" => lexemes "REPEAT" => |_| Ast::PenUp;

        // <block> ::= "[" <program> "]"
        "block" => rules "lbracket" "program" "rbracket" => |nodes| {
            Ast::Block(vec![nodes[1].clone()])
        };
        "lbracket" => lexemes "LBRACKET" => |_| Ast::PenUp;
        "rbracket" => lexemes "RBRACKET" => |_| Ast::PenUp;

        // <number> ::= [0-9]+
        "number" => lexemes "NUMBER" => |lexemes| {
            let n = lexemes[0].raw.parse::<i64>().unwrap();
            Ast::Number(n)
        };
    )
}

pub fn parse_input(input: &str) -> Ast {
    let grammar = grammar();
    let lex_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lex_rules, input).unwrap();
    let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
    parse_trees[0].as_abstract_syntax_tree()
}
