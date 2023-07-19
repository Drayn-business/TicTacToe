use std::{time::Duration, path::Path};

use sdl2::{pixels::Color, rect::{Rect, Point}, event::Event, keyboard::Keycode, mouse::MouseButton, render::{Canvas, TextureQuery}, video::Window};

fn main() {
    let game_size: u32 = 600;
    let box_size: u32 = game_size/5;
    let menu_size = 300;
    let font_path = "C:/Sources/TicTacToe/fonts/monospace.medium.ttf";
    let mut board: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut player = false;
    let mut end = false;

    let reset_button_rect = Rect::new(game_size as i32 + 50, 20, 200, 50);

    let context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let video_subsystem = context.video().unwrap();
    let font = ttf_context.load_font(Path::new(font_path), 30).unwrap();

    let window = video_subsystem.window("Tic Tac Toe", game_size + menu_size, game_size)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    let mut event_pump = context.event_pump().unwrap();
    
    'running: loop {
        let texture_creator = canvas.texture_creator();
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if end == true {continue;}

                    //Menu
                    if x >= reset_button_rect.x() && x <= reset_button_rect.x() + reset_button_rect.width() as i32 &&
                       y >= reset_button_rect.y() && y <= reset_button_rect.y() + reset_button_rect.height() as i32{
                        end = false;
                        board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
                        player = false;
                    }

                    //Game
                    if x >= game_size as i32 || y >= game_size as i32 {continue;}

                    let symbol = if player {1} else {2};
                    let i = (y / (game_size as i32 / 3)) as usize;
                    let j = (x / (game_size as i32 / 3)) as usize;
                    if board[i][j] == 0{
                        board[i][j] = symbol;
                        
                        end = check_win(board, end);

                    }
                    if end == false {player = !player;}
                    
                }
                _ => {}
            }
        }

        //Game

        //Background
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.fill_rect(Rect::new(0, 0, game_size, game_size)).unwrap();

        //Board
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        let line_strength: u32 = 2;

        canvas.fill_rect(Rect::new(0, 0, line_strength, game_size)).unwrap();
        canvas.fill_rect(Rect::new((game_size / 3) as i32, 0, line_strength, game_size)).unwrap();
        canvas.fill_rect(Rect::new(((game_size / 3) * 2) as i32, 0, line_strength, game_size)).unwrap();
        canvas.fill_rect(Rect::new((game_size - line_strength) as i32, 0, line_strength, game_size)).unwrap();

        canvas.fill_rect(Rect::new(0, 0, game_size, line_strength)).unwrap();
        canvas.fill_rect(Rect::new(0, (game_size / 3) as i32, game_size, line_strength)).unwrap();
        canvas.fill_rect(Rect::new(0, ((game_size / 3) * 2) as i32, game_size, line_strength)).unwrap();
        canvas.fill_rect(Rect::new(0, (game_size - line_strength) as i32, game_size, line_strength)).unwrap();

        for (i, row) in board.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                let centered_x = j as i32 * (game_size as i32 / 3) + (game_size as i32 / 3) / 2;
                let centered_y = i as i32 * (game_size as i32 / 3) + (game_size as i32 / 3) / 2;
                
                if *element == 0 {continue;}
                else if *element == 1 {
                    draw_circle(&mut canvas, centered_x, centered_y, (box_size as i32 / 3) * 2);
                    draw_circle(&mut canvas, centered_x, centered_y, (box_size as i32 / 3) * 2 + 1);
                }
                else if *element == 2 {
                    draw_cross(&mut canvas, centered_x - box_size as i32 / 2, centered_y - box_size as i32 / 2, box_size as i32, box_size as i32, 5);
                }
            }
        }

        //Menu

        //Background
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.fill_rect(Rect::new(game_size as i32, 0, menu_size, game_size)).unwrap();

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(reset_button_rect).unwrap();

        //Text
        {
            let surface = font
                .render("Reset game")
                .blended(Color::RGB(30, 30, 30))
                .unwrap();
    
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
    
            let TextureQuery { width, height, .. } = texture.query();
            canvas.copy(&texture, None, Some(Rect::new((game_size + 50 + height / 3) as i32, 20 + height as i32 / 4, width, height))).unwrap();        
        }

        if end == true {
            let surface = font
                .render(format!("Player {} won!", player as i32 + 1).as_str())
                .blended(Color::RGB(200, 200, 200))
                .unwrap();
    
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
    
            let TextureQuery { width, height, .. } = texture.query();
        
            canvas.copy(&texture, None, Some(Rect::new((game_size + menu_size / 2 - width / 2) as i32, game_size as i32 / 2, width, height))).unwrap();
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn check_win(board: [[i32; 3]; 3], end: bool) -> bool{
    for (i, row) in board.iter().enumerate() {
        if (row.iter().min() == row.iter().max() && row.iter().min() != Some(&0)) |
           (board[0][i] != 0 && board[0][i] == board[1][i] && board[0][i] == board[2][i]){
            return true;
        }
    }

    if (board[0][0] != 0 && board[0][0] == board[1][1] && board[0][0] == board[2][2]) |
       (board[0][2] != 0 && board[0][2] == board[1][1] && board[0][2] == board[2][0]){
        return true;
    }

    return end;
}

fn draw_circle(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32){
    let mut x = radius - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (radius << 1);

    while x >= y {
        canvas.draw_point(Point::new(center_x + x, center_y + y)).unwrap();
        canvas.draw_point(Point::new(center_x + y, center_y + x)).unwrap();
        canvas.draw_point(Point::new(center_x - y, center_y + x)).unwrap();
        canvas.draw_point(Point::new(center_x - x, center_y + y)).unwrap();
        canvas.draw_point(Point::new(center_x - x, center_y - y)).unwrap();
        canvas.draw_point(Point::new(center_x - y, center_y - x)).unwrap();
        canvas.draw_point(Point::new(center_x + y, center_y - x)).unwrap();
        canvas.draw_point(Point::new(center_x + x, center_y - y)).unwrap();

        if err <= 0
        {
            y += 1;
            err += dy;
            dy += 2;
        }
        
        if err > 0
        {
            x -= 1;
            dx += 2;
            err += dx - (radius << 1);
        }
    }
}

fn draw_cross(canvas: &mut Canvas<Window>, x: i32, y: i32, width: i32, height: i32, thickness: i32){
    let x0 = x - thickness / 2;
    let y0 = y - thickness / 2;
    let x1 = x + width - thickness / 2;
    let y1 = y + height - thickness / 2;

    if thickness < 1 {return;}
    else {
        canvas.draw_line(Point::new(x0, y0), Point::new(x1, y1)).unwrap();
        canvas.draw_line(Point::new(x1, y0), Point::new(x0, y1)).unwrap();
        if thickness > 1 {
            for i in 1..thickness/2 {
                canvas.draw_line(Point::new(x0 + i, y0 - i), Point::new(x1 + i, y1 - i)).unwrap();
                canvas.draw_line(Point::new(x0 - i, y0 + i), Point::new(x1 - i, y1 + i)).unwrap();
                canvas.draw_line(Point::new(x1 - i, y0 - i), Point::new(x0 - i, y1 - i)).unwrap();
                canvas.draw_line(Point::new(x1 + i, y0 + i), Point::new(x0 + i, y1 + i)).unwrap();
            }
        }
    }
}