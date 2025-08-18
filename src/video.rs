use opencv::core::Mat;

pub trait Renderable {
    fn render(&self);
}

pub trait Renderer {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>>;
}

// implement these traits
impl Renderable for String {
    fn render(&self) {
        print!("\x1B[H{}", self);
    }
}
