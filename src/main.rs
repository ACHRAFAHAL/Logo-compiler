mod ast;
mod language;
mod logo;

use ast::Ast;
use language::parse_input;
use logo::Logo;

fn main() {
    let mut compiler = Logo::new();

    let input = "penup backward 100 pendown repeat 5 [ forward 15 penup forward 15 pendown ]\
    ";
    let ast = parse_input(input);

    Ast::eval(&ast);

    let result_svg = compiler.compile(&ast);
    println!("{}", result_svg);
    std::fs::write("drawing.svg", result_svg).unwrap();
}
