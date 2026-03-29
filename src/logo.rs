use svg_fmt::*;

use crate::ast::Ast;

pub struct Logo {
    x: f32,
    y: f32,
    angle: f32,
    pen_down: bool,
    svg_elements: Vec<String>,
}

impl Logo {
    pub fn new() -> Self {
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

    pub fn compile(&mut self, ast: &Ast) -> String {
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
