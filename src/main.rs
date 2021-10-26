use macroquad::prelude::*;

const WIDTH: usize = 3;
const HEIGHT: usize = 3;
const WINDOW_TITLE: &str = "Tic Tac Toe";
const WINDOW_SIZE: i32 = 600;
const TILE_WIDTH: f32 = (WINDOW_SIZE / WIDTH as i32) as f32;
const TILE_HEIGHT: f32 = (WINDOW_SIZE / HEIGHT as i32) as f32;
const TILE_MARGIN: f32 = 2.0;
const TILE_COLOR: Color = RED;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Shape {
    E,  // empty
    O,  // Computer
    X,  // Player
}

#[derive(Debug)]
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

    fn update(&mut self, player: &Shape, moves_count: &mut usize) {
        let (x, y) = mouse_position();
        if x >= self.x + 0.1 && x <= self.x + self.w - 0.1 && y >= self.y + 0.1 && y <= self.y + self.h - 0.1 {
            self.color.a = 0.9;

            if is_mouse_button_pressed(MouseButton::Left) {
                self.shape = *player;
                *moves_count += 1;
                self.color = match player {
                    Shape::O => YELLOW,
                    Shape::X => BLUE,
                    _ => TILE_COLOR,
                };
            }
        } else {
            self.color.a = 1.0;
        }
    }
}

fn create_board(width: usize, height: usize) -> Vec<Tile> {
    let mut board: Vec<Tile> = Vec::with_capacity(height * width);

    for y in 0..height {
        for x in 0..width {
            board.push(Tile::new(
                x as f32, y as f32,
                TILE_WIDTH, TILE_HEIGHT,
                TILE_MARGIN,
                TILE_COLOR));
        }
    }

    board
}

fn draw_board(board: &Vec<Tile>) {
    for tile in board {
        tile.draw();
    }
}

fn update_board(board: &mut Vec<Tile>, player: &Shape, moves_count: &mut usize) {
    for tile in board {
        if tile.shape == Shape::E {
            tile.update(player, moves_count);
        }
    }
}

fn check_cross(board: &Vec<Tile>, width: usize, height: usize) -> (CrossType, Shape) {
    // check rows
    for y in 0..height {
        let y = y * width;
        if board[y + 0].shape != Shape::E
            && board[y + 0].shape == board[y + 1].shape
            && board[y + 1].shape == board[y + 2].shape {
            return (CrossType::Row(y), board[y + 0].shape);
        }
    }

    // check columns
    for x in 0..width {
        if board[0 * width + x].shape != Shape::E
            && board[0 * width + x].shape == board[1 * width + x].shape
            && board[1 * width + x].shape == board[2 * width + x].shape {
            return (CrossType::Column(x), board[0 * width + x].shape);
        }
    }

    // check diagonal top-left to bottom-right
    if board[0 * width + 0].shape != Shape::E
        && board[0 * width + 0].shape == board[1 * width + 1].shape
        && board[1 * width + 1].shape == board[2 * width + 2].shape {
        return (CrossType::Diagonal(DiagonalType::Left), board[0 * width + 0].shape);
    }

    // check diagonal top-right to bottom-left
    if board[0 * width + 2].shape != Shape::E
        && board[0 * width + 2].shape == board[1 * width + 1].shape
        && board[1 * width + 1].shape == board[2 * width + 0].shape {
        return (CrossType::Diagonal(DiagonalType::Right), board[0 * width + 2].shape);
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
    let mut board = create_board(WIDTH, HEIGHT);
    let players = [Shape::X, Shape::O];
    let mut moves_count = 0;
    let mut turn;
    
    loop {
        // Update
        turn = moves_count % players.len();
        update_board(&mut board, &players[turn], &mut moves_count);
        match check_cross(&board, WIDTH, HEIGHT) {
            (CrossType::Row(row), shape) => {
                println!("Cross on row: {}, Winner: {:?}", row, shape);
            },
            (CrossType::Column(column), shape) => {
                println!("Cross on column: {}, Winner: {:?}", column, shape);
            },
            (CrossType::Diagonal(side), shape) => {
                println!("Cross on diagonal: {:?} side, Winner: {:?}", side, shape);
            },
            _ => {},
        }

        // Render
        clear_background(WHITE);

        draw_board(&board);
        
        next_frame().await
    }
}
