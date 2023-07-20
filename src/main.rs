mod draw_utils;
mod render;

use std::{time::Duration, path::Path};

use draw_utils::draw_winning_line;
use sdl2::{rect::Rect, event::Event, keyboard::Keycode, mouse::MouseButton};
use render::renderer::{render_background, render_grid, render_symbols, render_button, render_text};


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
    let mut font = ttf_context
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

        canvas.clear();

        render_background(&mut canvas);
        render_grid(&mut canvas, game_size);
        render_symbols(&mut canvas, board, game_size, box_size);
        draw_winning_line(&mut canvas, game_size, winning_line.0, winning_line.1, winning_line.2, winning_line.3);
        render_button(&mut canvas, reset_button_rect);
        render_text(&mut canvas, &mut font, texture_creator, game_size, menu_size, end, player);

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