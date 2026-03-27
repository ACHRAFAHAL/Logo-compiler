use std::fs;
use svg_fmt::*;

fn main() {
    let Start = BeginSvg { w: 300.0, h: 200.0 };

    let line1 = path()
        .move_to(100.0, 100.0)
        .line_to(200.0, 100.0)
        .stroke(Stroke::Color(blue(), 1.0));

    let line2 = path()
        .move_to(200.0, 100.0)
        .line_to(200.0, 200.0)
        .stroke(Stroke::Color(blue(), 1.0));

    let line3 = path()
        .move_to(200.0, 200.0)
        .line_to(100.0, 200.0)
        .stroke(Stroke::Color(blue(), 1.0));

    let line4 = path()
        .move_to(100.0, 200.0)
        .line_to(100.0, 100.0)
        .stroke(Stroke::Color(blue(), 1.0));

    let svg_content = format!(
        "{}\n    {}\n    {}\n    {}\n    {}\n{}",
        Start, line1, line2, line3, line4, EndSvg
    );

    let file_name = "square.svg";
    fs::write(file_name, svg_content).unwrap();

    println!("{} has been generated", file_name);
}