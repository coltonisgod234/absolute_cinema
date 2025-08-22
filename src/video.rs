use opencv::core::{Mat, MatTrait, MatTraitConst, MatTraitConstManual, Vec3b};

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

pub fn get_px(frame: &Mat, x: i32, y: i32) -> Vec3b {
    let data = frame.data(); // u8 slice
    let offset: i32 = (y * frame.cols() + x) * 3;

    unsafe {
        let r = *data.add((offset + 2) as usize);
        let g = *data.add((offset + 1) as usize);
        let b = *data.add(offset as usize);
        Vec3b::from([r, g, b])
    }
}
