use santiago::lexer::LexerRules;
use santiago::grammar::Grammar;
use svg_fmt::*;

#[derive(Debug, Clone, PartialEq)]
enum Ast {
    Program(Vec<Ast>),
    Command(Vec<Ast>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i64),
    PenUp,
    PenDown,
    Block(Vec<Ast>),
    Loop(Vec<Ast>),
}

struct Logo {
    x: f32,
    y: f32,
    angle: f32,
    pen_down: bool,
    svg_elements: Vec<String>,
}


fn lexer_rules() -> LexerRules {
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

        // <loop> ::= "repeat" <number> <command>
        "loop" => rules "kw_repeat" "number" "block" => |nodes| {
            Ast::Loop(vec![nodes[1].clone(), nodes[2].clone()])
        };
        "kw_repeat" => lexemes "REPEAT" => |_| Ast::PenUp;

        // <block> ::= "[" <program "]"
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

impl Ast {
    fn eval(ast: &Ast) {
        match ast {
            Ast::Program(nodes) => {
                if nodes.len() == 0 {
                    println!("Stop");
                } else {
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
            Ast::Backward => print!("Backward of "),
            Ast::Left => print!("Turn left "),
            Ast::Right => print!("Turn right "),
            Ast::PenUp => println!("Pen up"),
            Ast::PenDown => println!("Pen down"),
            Ast::Loop(nodes) => {
                if let Ast::Number(count) = &nodes[0] {
                    println!("Repeat {} times ", count);
                    Self::eval(&nodes[1]);
                    println!("End repeat");
                }
            }
            Ast::Block(nodes) => {
                Self::eval(&nodes[0]);
            }
        }
    }
}

impl Logo {
    fn new() -> Self {
        Self {
            x: 150.0,
            y: 100.0,
            angle: 0.0,
            pen_down: true,
            svg_elements: Vec::new(),
        }
    }

    fn move_turtle(&mut self, distance: f32) {
        let rad = self.angle.to_radians();
        let new_x = self.x + distance * rad.cos();
        let new_y = self.y + distance * rad.sin();

        if self.pen_down {
            let p = path()
                .move_to(self.x, self.y)
                .line_to(new_x, new_y)
                .stroke(Stroke::Color(blue(), 1.0))
                .fill(Fill::None);

            self.svg_elements.push(p.to_string());
        }

        self.x = new_x;
        self.y = new_y;
    }

    fn compile_node(&mut self, ast: &Ast) {
        match ast {
            Ast::Program(nodes) => {
                for n in nodes {
                    self.compile_node(n);
                }
            }
            Ast::Command(nodes) => {
                if nodes.len() == 2 {
                    if let Ast::Number(val) = &nodes[1] {
                        let distance = *val as f32;
                        match &nodes[0] {
                            Ast::Forward => self.move_turtle(distance),
                            Ast::Backward => self.move_turtle(-distance),
                            Ast::Left => self.angle -= distance,
                            Ast::Right => self.angle += distance,
                            _ => {}
                        }
                    }
                } else if nodes.len() == 1 {
                    self.compile_node(&nodes[0]);
                }
            }
            Ast::Loop(loop_nodes) => {
                if let Ast::Number(count) = &loop_nodes[0] {
                    for _ in 0..*count {
                        self.compile_node(&loop_nodes[1]);
                    }
                }
            }
            Ast::Block(block_nodes) => {
                self.compile_node(&block_nodes[0]);
            }
            Ast::PenUp => self.pen_down = false,
            Ast::PenDown => self.pen_down = true,
            _ => {}
        }
    }

    fn compile(&mut self, ast: &Ast) -> String {
        self.compile_node(ast);
        let start = BeginSvg { w: 300.0, h: 200.0 };
        let mut final_svg = format!("{}\n", start);

        for e in &self.svg_elements {
            final_svg.push_str(&format!("    {}\n", e));
        }

        final_svg.push_str(&format!("{}", EndSvg));

        final_svg
    }
}
fn main() {
    let mut compiler = Logo::new();

    let input = "penup backward 100 pendown repeat 5 [ forward 15 penup forward 15 pendown ]\
    ";
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
