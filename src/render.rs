pub mod renderer {
    use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureQuery, TextureCreator}, video::{Window, WindowContext}, ttf::Font};

    use crate::draw_utils::{draw_circle, draw_cross, draw_winning_line};

    pub fn render(canvas: &mut Canvas<Window>, font: &mut Font<'_, '_>, texture_creator: TextureCreator<WindowContext>, board: [[i32; 3]; 3], game_size: u32, menu_size: u32, end: bool, player: bool, winning_line: (i32, i32, i32, i32), reset_button_rect: Rect){
        let box_size: u32 = game_size/5;

        canvas.clear();

        render_background(canvas, game_size, menu_size);
        render_grid(canvas, game_size);
        render_symbols(canvas, board, game_size, box_size);
        draw_winning_line(canvas, game_size, winning_line.0, winning_line.1, winning_line.2, winning_line.3);
        render_button(canvas, reset_button_rect);
        render_text(canvas, font, texture_creator, game_size, menu_size, end, player);

        canvas.present();
    }
    
    fn render_background(canvas: &mut Canvas<Window>, game_size: u32, menu_size: u32){
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        //Game
        canvas.fill_rect(Rect::new(0, 0, game_size, game_size)).unwrap();
        //Menu
        canvas.fill_rect(Rect::new(game_size as i32, 0, menu_size, game_size)).unwrap();
    }

    fn render_grid(canvas: &mut Canvas<Window>, game_size: u32){
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
    }

    fn render_symbols(canvas: &mut Canvas<Window>, board: [[i32; 3]; 3], game_size: u32, box_size: u32){
        canvas.set_draw_color(Color::RGB(200, 200, 200));

        for (i, row) in board.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                let centered_x = j as i32 * (game_size as i32 / 3) + (game_size as i32 / 3) / 2;
                let centered_y = i as i32 * (game_size as i32 / 3) + (game_size as i32 / 3) / 2;
                
                if *element == 0 {continue;}
                else if *element == 1 {
                    draw_circle(canvas, centered_x, centered_y, (box_size as i32 / 3) * 2);
                    draw_circle(canvas, centered_x, centered_y, (box_size as i32 / 3) * 2 + 1);
                }
                else if *element == 2 {
                    draw_cross(canvas, centered_x - box_size as i32 / 2, centered_y - box_size as i32 / 2, box_size as i32, box_size as i32, 5);
                }
            }
        }
    }

    fn render_button(canvas: &mut Canvas<Window>, reset_button_rect: Rect){
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(reset_button_rect).unwrap();
    }

    fn render_text(canvas: &mut Canvas<Window>, font: &mut Font<'_, '_>, texture_creator: TextureCreator<WindowContext>, game_size: u32, menu_size: u32, end: bool, player: bool) {
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
    }
}
