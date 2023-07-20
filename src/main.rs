mod draw_utils;

use std::{time::Duration, path::Path};

use draw_utils::{draw_circle, draw_cross, draw_winning_line};
use sdl2::{pixels::Color, rect::Rect, event::Event, keyboard::Keycode, mouse::MouseButton, render::TextureQuery};


fn main() {
    let game_size: u32 = 600;
    let box_size: u32 = game_size/5;
    let menu_size = 300;
    let font_path = "C:/Sources/TicTacToe/fonts/Roboto-Medium.ttf";
    let mut board: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut player = false;
    let mut end = false;
    let mut winning_line: (i32, i32, i32, i32) = (0, 0, 0, 0);

    let reset_button_rect = Rect::new(game_size as i32 + 50, 20, 200, 50);

    let context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let video_subsystem = context.video().unwrap();
    let font = ttf_context
        .load_font(Path::new(font_path), 30)
        .unwrap();

    let window = video_subsystem
        .window("Tic Tac Toe", game_size + menu_size, game_size)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    let mut event_pump = context.event_pump().unwrap();
    
    'running: loop {
        let texture_creator = canvas.texture_creator();
        canvas.clear();

        //Event handling

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    //Menu
                    if x >= reset_button_rect.x() && x <= reset_button_rect.x() + reset_button_rect.width() as i32 &&
                       y >= reset_button_rect.y() && y <= reset_button_rect.y() + reset_button_rect.height() as i32{
                        end = false;
                        board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
                        player = false;
                        winning_line = (0, 0, 0, 0);
                    }

                    if end == true {continue;}

                    //Game
                    if x >= game_size as i32 || y >= game_size as i32 {continue;}

                    let symbol = if player {1} else {2};
                    let i = (y / (game_size as i32 / 3)) as usize;
                    let j = (x / (game_size as i32 / 3)) as usize;
                    if board[i][j] == 0 {
                        board[i][j] = symbol;
                        
                        (end, winning_line) = check_win(board, end);
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

        //Symbols
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

        draw_winning_line(&mut canvas, game_size, winning_line.0, winning_line.1, winning_line.2, winning_line.3);

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
            canvas.copy(&texture, None, Some(Rect::new((game_size + width / 2 - 5) as i32, 20 + height as i32 / 4, width, height))).unwrap();        
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

fn check_win(board: [[i32; 3]; 3], end: bool) -> (bool, (i32, i32, i32, i32)){
    for (i, row) in board.iter().enumerate() {
        if row.iter().min() == row.iter().max() && row.iter().min() != Some(&0) {
            return (true, (0, i as i32, 2, i as i32));
        }
        else if board[0][i] != 0 && board[0][i] == board[1][i] && board[0][i] == board[2][i]{
            return (true, (i as i32, 0, i as i32, 2));
        }
    }

    if board[0][0] != 0 && board[0][0] == board[1][1] && board[0][0] == board[2][2]{
        return (true, (0, 0, 2, 2));
    }
    else if board[0][2] != 0 && board[0][2] == board[1][1] && board[0][2] == board[2][0]{
        return (true, (0, 2, 2, 0));
    }

    return (end, (0, 0, 0, 0));
}