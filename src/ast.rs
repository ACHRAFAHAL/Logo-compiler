#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
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

impl Ast {
    pub fn eval(ast: &Ast) {
        match ast {
            Ast::Program(nodes) => {
                if nodes.is_empty() {
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
