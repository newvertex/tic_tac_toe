use macroquad::prelude::*;

const SIDE: usize = 3;
const WINDOW_TITLE: &str = "Tic Tac Toe";
const WINDOW_SIZE: i32 = 600;
const TILE_SIZE: i32 = WINDOW_SIZE / SIDE as i32;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    E,  // empty
    O,  // Computer
    X,  // Player
}

enum DiagonalType {
    Left,
    Right,
}

enum CrossType {
    None,
    Row(usize),
    Column(usize),
    Diagonal(DiagonalType),
}

struct Tile {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    margin: f32,
    color: Color,
    shape: Shape,
}

impl Tile {
    fn new(x: f32, y: f32, w: f32, h: f32, margin: f32, color: Color) -> Self {
        Self {
            x: (x * w) + (x * margin),
            y: (y * h) + (y * margin),
            w,
            h,
            margin,
            color,
            shape: Shape::E,
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }

    fn update(&mut self) {
        let (x, y) = mouse_position();
        if x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h {
            self.color.a = 200.0;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.on_clicked();
            }
        } else {
            self.color.a = 256.0;
        }
    }

    fn on_clicked(&mut self) {

    }
}

fn draw_board(board: &[[Shape; SIDE];SIDE]) {
    for (y, row) in board.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let pos_x = (x as f32 * TILE_SIZE) + 1.0;
            let pos_y = (y as f32 * TILE_SIZE) + 1.0;

            let color = match col {
                Shape::E => RED,
                Shape::O => YELLOW,
                Shape::X => SKYBLUE,
            };

            draw_rectangle(pos_x, pos_y, TILE_SIZE - 2.0, TILE_SIZE - 2.0, color);
        }
    }
}

fn check_cross(board: &[[Shape; SIDE];SIDE]) -> (CrossType, Shape) {
    for y in 0..board.len() {
        if board[y][0] != Shape::E && board[y][0] == board[y][1] && board[y][1] == board[y][2] {
            return (CrossType::Row(y), board[y][0]);
        }
    }

    for x in 0..board.len() {
        if board[0][x] != Shape::E && board[0][x] == board[1][x] && board[1][x] == board[2][x] {
            return (CrossType::Column(x), board[0][x]);
        }
    }

    if board[0][0] != Shape::E && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return (CrossType::Diagonal(DiagonalType::Left), board[0][0]);
    }

    if board[0][2] != Shape::E && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return (CrossType::Diagonal(DiagonalType::Right), board[0][2]);
    }

    (CrossType::None, Shape::E)
}

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board: [[Shape; SIDE];SIDE] = [[Shape::E; SIDE]; SIDE];
    let mut players = [Shape::X, Shape::O];
    let mut move_count = 0;

    loop {
        // Update
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            let x = match x as i32 {
                1..=199 => 0,
                200..=399 => 1,
                400..=600 => 2,
                _ => 100,
            };
            let y = match y as i32 {
                1..=199 => 0,
                200..=399 => 1,
                400..=600 => 2,
                _ => 100,
            };

            board[y][x] = if board[y][x] != Shape::E {
                Shape::E
            } else {
                Shape::O
            };
        }


        // Render
        clear_background(WHITE);

        draw_board(&board);
        
        next_frame().await
    }
}
