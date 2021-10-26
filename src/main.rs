use macroquad::prelude::*;

const SIDE: usize = 3;
const WINDOW_SIZE: i32 = 600;
const TILE_SIZE: f32 = (WINDOW_SIZE / SIDE as i32) as f32;

enum Player {
    Computer,
    Human,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    E,  // empty
    O,  // Computer
    X,  // Player
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

fn row_crossed(board: &[[Shape; SIDE];SIDE]) -> bool {
    for (y, row) in board.iter().enumerate() {
        if row[0] != Shape::E && row[0] == row[1] && row[1] == row[2] {
            return true;
        }
    }

    false
}

fn col_crossed(board: &[[Shape; SIDE];SIDE]) -> bool {
    for x in 0..SIDE {
        if board[0][x] != Shape::E && board[0][x] == board[1][x] && board[1][x] == board[2][x] {
            return true;
        }
    }

    false
}

fn diagonal_crossed(board: &[[Shape; SIDE];SIDE]) -> bool {
    if board[0][0] != Shape::E && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return true;
    }

    if board[0][2] != Shape::E && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return true;
    }

    false
}

fn check_game_over(board: &[[Shape; SIDE];SIDE]) -> bool {
    row_crossed(board) || col_crossed(board) || diagonal_crossed(board)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_string(),
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

    board[2][2] = Shape::X;
    board[0][1] = Shape::O;

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
