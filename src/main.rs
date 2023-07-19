use std::time::Duration;

use sdl2::{pixels::Color, rect::Rect, event::Event, keyboard::Keycode, mouse::MouseButton};

fn main() {
    let size: u32 = 600;
    let box_size: u32 = 50;
    let mut board: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut player = false;
    let mut end = false;

    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("Tic Tac Toe", size, size)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if end == true {continue;}

                    let symbol = if player {1} else {2};
                    let i = (y / (size as i32 / 3)) as usize;
                    let j = (x / (size as i32 / 3)) as usize;
                    if board[i][j] == 0{
                        board[i][j] = symbol;
                        
                        end = check_win(board, player, end);

                        player = !player;
                    }
                }
                _ => {}
            }
        }

        //Background
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.fill_rect(Rect::new(0, 0, size, size)).unwrap();

        //Board
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new((size / 3) as i32, 0, 2, size)).unwrap();
        canvas.fill_rect(Rect::new(((size / 3) * 2) as i32, 0, 2, size)).unwrap();

        canvas.fill_rect(Rect::new(0, (size / 3) as i32, size, 2)).unwrap();
        canvas.fill_rect(Rect::new(0, ((size / 3) * 2) as i32, size, 2)).unwrap();

        for (i, row) in board.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                let x = j as i32 * (size as i32 / 3) + (size as i32 / 3) / 2 - box_size as i32 / 2;
                let y = i as i32 * (size as i32 / 3) + (size as i32 / 3) / 2 - box_size as i32 / 2;
                
                if *element == 0 {continue;}
                else if *element == 1 {
                    canvas.set_draw_color(Color::RGB(0, 200, 0));
                }
                else if *element == 2 {
                    canvas.set_draw_color(Color::RGB(0, 0, 200));
                }
                canvas.fill_rect(Rect::new(x, y, box_size, box_size)).unwrap();
            }
        }
        
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn check_win(board: [[i32; 3]; 3], player: bool, end: bool) -> bool{
    for (i, row) in board.iter().enumerate() {
        if (row.iter().min() == row.iter().max() && row.iter().min() != Some(&0)) |
           (board[0][i] != 0 && board[0][i] == board[1][i] && board[0][i] == board[2][i]){
            if end == true {return end;}
            println!("Player {} won!", player as i32 + 1);
            return true;
        }
    }

    if (board[0][0] != 0 && board[0][0] == board[1][1] && board[0][0] == board[2][2]) |
       (board[0][2] != 0 && board[0][2] == board[1][1] && board[0][2] == board[2][0]){
        if end == true {return end;}
        println!("Player {} won!", player as i32 + 1);
        return true;
    }

    return end;
}