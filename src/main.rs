use std::time::Duration;

use sdl2::{pixels::Color, rect::Rect};

fn main() {
    let width: u32 = 500;
    let height: u32 = 500;

    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("Hello, World!", width, height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.fill_rect(Rect::new(0, 0, width, height)).unwrap();

    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.fill_rect(Rect::new((width / 3) as i32, 0, 2, height)).unwrap();
    canvas.fill_rect(Rect::new(((width / 3) * 2) as i32, 0, 2, height)).unwrap();

    canvas.fill_rect(Rect::new(0, (width / 3) as i32, width, 2)).unwrap();
    canvas.fill_rect(Rect::new(0, ((width / 3) * 2) as i32, width, 2)).unwrap();
    
    canvas.present();
    std::thread::sleep(Duration::new(3, 0));
}
