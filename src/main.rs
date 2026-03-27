use santiago::lexer::LexerRules;
use santiago::grammar::Grammar;

#[derive(Debug, Clone, PartialEq)]
enum Ast {
    Program(Vec<Ast>),
    Command(Vec<Ast>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i64),
}

struct Logo {
    x: f64,
    y: f64,
    angle: f64,
    pen_down: bool,
    svg_content: String,
}


fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // Movement commands
        "DEFAULT" | "FORWARD"   = string "forward"  ;
        "DEFAULT" | "BACKWARD"  = string "backward" ;
        "DEFAULT" | "LEFT"      = string "left"      ;
        "DEFAULT" | "RIGHT"     = string "right"     ;

        // Numbers
        "DEFAULT" | "NUMBER"    = pattern r"[0-9]+"  ;

        // Spaces, tabulations and returns to line are to be ignored
        "DEFAULT" | "WS"        = pattern r"\s+" => |lexer| lexer.skip();
    )
}

fn grammar() -> Grammar<Ast > {
    santiago::grammar!(
        // <program> ::= <command> <program> or empty
        "program" => rules "command" "program" => Ast::Program;
        "program" => empty => |_| Ast::Program(vec![]);

        // <command> ::= <order> <number>
        "command" => rules "order" "number" => Ast::Command;

        // <order> ::= "forward" or "backward" or "left" or "right"
        "order" => lexemes "FORWARD"  => |_| Ast::Forward;
        "order" => lexemes "BACKWARD" => |_| Ast::Backward;
        "order" => lexemes "LEFT"     => |_| Ast::Left;
        "order" => lexemes "RIGHT"    => |_| Ast::Right;

        // <number> ::= [0-9]+
        "number" => lexemes "NUMBER" => |lexemes| {
            let n = lexemes[0].raw.parse::<i64>().unwrap();
            Ast::Number(n)
        };
    )
}

impl Ast {
    fn eval(ast: &Ast) {
        match ast {
            Ast::Program(nodes) => {
                if nodes.len() == 0 {
                    println!("Stop");
                }
                else {
                    for n in nodes {
                        Self::eval(n)
                    }
                }
            }
            Ast::Command(nodes) => {
                for n in nodes {
                    Self::eval(n)
                }
            }
            Ast::Number(val) => print!("{} \n", val),
            Ast::Forward => print!("Advance of "),
            Ast::Left => print!("Turn "),
            _ => {}
        }
    }
}

impl Logo {
    fn new() -> Self {
        let header = r#"<?xml version="1.0" encoding="utf-8"?><svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="300" height="200">"#;
        Self {
            x: 150.0,
            y: 100.0,
            angle: 0.0,
            pen_down: true,   // pen down by default
            svg_content: String::from(header),
        }
    }

    fn move_turtle(&mut self, distance: f64) {
        let rad = self.angle.to_radians();
        let new_x = self.x + distance * rad.cos();
        let new_y = self.y + distance * rad.sin();

        if self.pen_down {
            let path = format!(
                "    <path d=\"M {} {} L {} {}\" stroke=\"blue\" fill=\"transparent\"/>\n",
                self.x, self.y, new_x, new_y
            );
            self.svg_content.push_str(&path);
        }

        self.x = new_x;
        self.y = new_y;
    }

    fn compile(&mut self, ast: &Ast) -> String {
        match ast {
            Ast::Program(nodes) => {
                for n in nodes {
                    self.compile(n);
                }
            }
            Ast::Command(nodes) => {
                if nodes.len() == 2 {
                    if let Ast::Number(val) = nodes[1] {
                        let distance = val as f64;
                        match &nodes[0] {
                            Ast::Forward => self.move_turtle(distance),
                            Ast::Backward => self.move_turtle(-distance),
                            Ast::Left => self.angle -= distance,
                            Ast::Right => self.angle += distance,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        format!("{}</svg>", self.svg_content)
    }
}
fn main() {
    let mut compiler = Logo::new();

    let input = "forward 100 left 90 forward 100 left 90 forward 100 left 90 forward 100 left 90 ";
    let grammar = grammar();
    let lex_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lex_rules, &input).unwrap();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).unwrap();
    let ast = parse_trees[0].as_abstract_syntax_tree();

    Ast::eval(&ast);

    let result_svg = compiler.compile(&ast);
    println!("{}", result_svg);
    std::fs::write("drawing.svg", result_svg).unwrap();
}
