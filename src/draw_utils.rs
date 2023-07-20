use sdl2::{render::Canvas, rect::Point, video::Window};

pub fn draw_circle(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32){
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

pub fn draw_cross(canvas: &mut Canvas<Window>, x: i32, y: i32, width: i32, height: i32, thickness: i32){
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

pub fn draw_winning_line(canvas: &mut Canvas<Window>,game_size: u32, board_x0: i32, board_y0: i32, board_x1: i32, board_y1: i32){
    if board_x0 == board_x1 && board_y0 == board_y1 {return;}
    let column_size = game_size as i32 / 3;

    if board_x0 == board_x1 {
        canvas.draw_line(
            Point::new(board_x0 * column_size + column_size / 2, board_y0 * column_size), 
            Point::new(board_x1 * column_size + column_size / 2, (board_y1 + 1) * column_size)
        ).unwrap();
    } 
    else if board_y0 == board_y1 {
        canvas.draw_line(
            Point::new(board_x0 * column_size, board_y0 * column_size + column_size / 2), 
            Point::new((board_x1 + 1) * column_size, board_y1 * column_size + column_size / 2)
        ).unwrap();
    }
    else if board_x1 > board_x0 && board_y1 > board_y0 {
        canvas.draw_line(
            Point::new(board_x0 * column_size, board_y0 * column_size), 
            Point::new(board_x1 * column_size + column_size, board_y1 * column_size + column_size)
        ).unwrap();
    }
    else {
        canvas.draw_line(
            Point::new(board_x0 * column_size, (board_y0 + 1) * column_size), 
            Point::new((board_x1 + 1) * column_size, board_y1 * column_size)
        ).unwrap();
    }
}
