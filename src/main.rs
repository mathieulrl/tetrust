mod util;
mod display;
mod terminal;

use display::Display;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use util::*;
use rand::seq::SliceRandom;


const BOARD_WIDTH: u32 = 10;
const BOARD_HEIGHT: u32 = 20;
const HIDDEN_ROWS: u32 = 2;

#[derive(PartialEq)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    CtrlC,
    Char(char),
}

enum GameUpdate {
    KeyPress(Key),
    Tick,
}

//#[derive(PartialEq, Eq)]
enum GameOver {
    LockOut,
    BlockOut,
    TopOut,
}

impl GameOver {
    fn description(&self) -> &str {
        match self {
            GameOver::LockOut => panic!("The pieces are locked and cannot move."),
            GameOver::BlockOut => panic!("The playfield is completely blocked with pieces."),
            GameOver::TopOut => panic!("The pieces have reached the top of the playfield."),
        }
    }
}



#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Board {
    cells: [[Option<Color>; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],

}

impl Board {
    pub fn render(&self, display: &mut Display) {
        for y in HIDDEN_ROWS..BOARD_HEIGHT {
            display.set_text("|", 0, y, Color::Red, Color::Black);
            display.set_text("|", BOARD_WIDTH * 2 + 1, y, Color::Red, Color::Black);
        }
        for x in 0..(BOARD_WIDTH * 2 + 1) {
            display.set_text("-", x, BOARD_HEIGHT, Color::Red, Color::Black);
        }
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if let Some(color) = self.cells[row as usize][col as usize] {
                    let c = 1 + (col * 2);
                    display.set_text(" ", c, row, color, color);
                    display.set_text(" ", c + 1, row, color, color);
                }
            }
        }

       
    
    }

    pub fn lock_piece(&mut self, piece: &Piece, origin: Point) {
        piece.each_point(&mut |row, col| {
            let x = origin.x + (col as i32);
            let y = origin.y + (row as i32);
            self.cells[y as usize][x as usize] = Some(piece.color);
        });
    }

    pub fn collision_test(&self, piece: &Piece, origin: Point) -> bool {
        let mut found = false;
        piece.each_point(&mut |row, col| {
            if !found {
                let x = origin.x + col;
                let y = origin.y + row;
                if x < 0 || x >= (BOARD_WIDTH as i32) || y < 0 || y >= (BOARD_HEIGHT as i32) ||
                    self.cells[y as usize][x as usize] != None {
                  found = true;
                }
            }
        });

        found
    }

    /// Clears the board of any complete lines, shifting down rows to take their place.
    /// Returns the total number of lines that were cleared.
    fn clear_lines(&mut self) -> u32 {
        let mut cleared_lines: usize = 0;
        for row in (0..self.cells.len()).rev() {
            if (row as i32) - (cleared_lines as i32) < 0 {
                break;
            }

            if cleared_lines > 0 {
                self.cells[row] = self.cells[row - cleared_lines];
                self.cells[row - cleared_lines] = [None; BOARD_WIDTH as usize];
            }

            while !self.cells[row].iter().any(|x| *x == None) {
                cleared_lines += 1;
                self.cells[row] = self.cells[row - cleared_lines];
                self.cells[row - cleared_lines] = [None; BOARD_WIDTH as usize];
            }
        }

        cleared_lines as u32
    }
}

struct Piece {
    color: Color,
    shape: Vec<Vec<u8>>,
}

impl Clone for Piece {
    fn clone(&self) -> Piece {
        let mut p = Piece{
            color: self.color,
            shape: Vec::with_capacity(self.shape.len())
        };
        for row in &self.shape {
            p.shape.push(row.clone());
        }
        p
    }
}

impl Piece {
    pub fn new_o() -> Piece {
        Piece{
            color: Color::Cyan,
            shape: vec![vec![1, 1],
                        vec![1, 1]]
        }
    }

    pub fn new_l() -> Piece {
        Piece{
            color: Color::Orange,
            shape: vec![vec![0, 0, 1],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_j() -> Piece {
        Piece{
            color: Color::Blue,
            shape: vec![vec![1, 0, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_t() -> Piece {
        Piece{
            color: Color::Purple,
            shape: vec![vec![0, 1, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_s() -> Piece {
        Piece{
            color: Color::Green,
            shape: vec![vec![0, 1, 1],
                        vec![1, 1, 0],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_z() -> Piece {
        Piece{
            color: Color::Red,
            shape: vec![vec![1, 1, 0],
                        vec![0, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_i() -> Piece {
        Piece{
            color: Color::Cyan,
            shape: vec![vec![0, 0, 0, 0],
                        vec![1, 1, 1, 1],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0]]
        }
    }

    fn rotate(&mut self, direction: Direction) {
        let size = self.shape.len();

        for row in 0..size/2 {
            for col in row..(size - row - 1) {
                let t = self.shape[row][col];

                match direction {
                    Direction::Left => {
                        self.shape[row][col] = self.shape[col][size - row - 1];
                        self.shape[col][size - row - 1] = self.shape[size - row - 1][size - col - 1];
                        self.shape[size - row - 1][size - col - 1] = self.shape[size - col - 1][row];
                        self.shape[size - col - 1][row] = t;
                    },
                    Direction::Right => {
                        self.shape[row][col] = self.shape[size - col - 1][row];
                        self.shape[size - col - 1][row] = self.shape[size - row - 1][size - col - 1];
                        self.shape[size - row - 1][size - col - 1] = self.shape[col][size - row - 1];
                        self.shape[col][size - row - 1] = t;
                    }
                }
            }
        }
    }

    fn each_point(&self, callback: &mut dyn FnMut(i32, i32)) {
        let piece_width = self.shape.len() as i32;
        for row in 0..piece_width {
            for col in 0..piece_width {
                if self.shape[row as usize][col as usize] != 0 {
                    callback(row, col);
                }
            }
        }
    }
}

/// Implements a queue of randomized tetrominoes.
///
/// Instead of a purely random stream of tetromino types, this queue generates a random ordering of all
/// possible types and ensures all of those pieces are used before re-generating a new random set. This helps
/// avoid pathological cases where purely random generation provides the same piece type repeately in a row,
/// or fails to provide a required piece for a very long time.
struct PieceBag {
    pieces: [Option<Piece>;7] 
}

impl PieceBag {
    fn new() -> PieceBag {
        let mut p = PieceBag{
            pieces: [None, None, None, None, None, None, None]
        };
        p.fill_bag();
        p
    }

    /// Removes and returns the next piece in the queue.
    fn pop(&mut self) -> Piece {
        if let Some(piece) = self.pieces[0].take() {
            // Shift the remaining pieces to the front
            for i in 0..(6) {
                self.pieces[i] = self.pieces[i + 1].take();
            }
            // Fill the last slot with a new piece
            if self.pieces[6].is_none() {
                self.fill_bag();
            }
            piece.clone()
        } else {
            self.fill_bag();
            self.pop()
        }
    }

    /// Returns a copy of the next piece in the queue.
    fn peek(&self) -> Piece {
        match &self.pieces[0] {
            Some(p) => p.clone(),
            None => panic!("No next piece in piece bag")
        }
    }

    /// Generates a random ordering of all possible pieces and adds them to the piece queue.
    fn fill_bag(&mut self) {
        //use rand::Rng;

        let mut pieces: [Option<Piece>;7] = [
            Some(Piece::new_o()),
            Some(Piece::new_l()),
            Some(Piece::new_j()),
            Some(Piece::new_t()),
            Some(Piece::new_s()),
            Some(Piece::new_z()),
            Some(Piece::new_i())
        ];

        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..7).collect();
        indices.shuffle(&mut rng);
    
        for i in 0..7 {
            if let Some(piece) = pieces[indices[i]].take() {
                self.pieces[i] = Some(piece.clone());
            }
        }
}
}

struct Game {
    board: Board,
    piece_bag: PieceBag,
    piece: Piece,
    piece_position: Point,
    score: u32,
    level: u32,       
    total_lines: u32, 
    game_over: bool,
}

impl Game {
    fn new() -> Game {
        let mut piece_bag = PieceBag::new();
        let piece = piece_bag.pop();

        let mut game = Game {
            board: Board{
                cells: [[None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize]
            },
            piece_bag: piece_bag,
            piece: piece,
            piece_position: Point{ x: 0, y: 0 },
            level: 0,           
            score: 0,          
            total_lines: 0,    
            game_over: false,
        };



        game.place_new_piece();
        game
    }



    /// Returns the new position of the current piece if it were to be dropped.
    fn find_dropped_position(&self) -> Point {
        let mut origin = self.piece_position;
        while !self.board.collision_test(&self.piece, origin) {
            origin.y += 1;
        }
        origin.y -= 1;
        origin
    }

    fn game_over_condition(&self) -> bool {
        // If the game is already marked as game over, no need to check again
        if self.game_over {
            return true;
        }
    
        // Check if a new piece cannot be placed
        let new_piece = self.piece_bag.peek();
        let origin = Point {
            x: ((BOARD_WIDTH - (new_piece.shape.len() as u32)) / 2) as i32,
            y: 0,
        };
        if self.board.collision_test(&new_piece, origin) {
            return true;
        }
    
        false
    }


    /// Draws the game to the display.
    fn render(&self, display: &mut Display) {
        // Render the board
        self.board.render(display);

        // Render the level
        let left_margin = BOARD_WIDTH * 2 + 5;
        display.set_text("Level: 1", left_margin, 3, Color::Red, Color::Black);
        let score_line = format!("Score: {}",self.score);
        display.set_text(&score_line, left_margin, 4, Color::Red, Color::Black);
        
        // Define left_margin before using it
        //let left_margin = BOARD_WIDTH * 2 + 5;

 

       /* // Create strings as owned `String` instances
        let level_text = format!("Level: {}", self.level);
        let score_text = format!("Score: {}", self.score);
        let lines_cleared_text = format!("Lines Cleared: {}", self.total_lines);

        // Render the level
        display.set_text(&level_text, left_margin, 3, Color::Red, Color::Black);

        // Render the score
        display.set_text(&score_text, left_margin, 4, Color::Red, Color::Black);

        // Render the lines cleared
        display.set_text(&lines_cleared_text, left_margin, 5, Color::Red, Color::Black);
                        
*/

        // Render the currently falling piece
        let x = 1 + (2 * self.piece_position.x);
        self.render_piece(display, &self.piece, Point{ x: x, y: self.piece_position.y });

        // Render a ghost piece
        let ghost_position = self.find_dropped_position();
        self.render_piece(display, &self.piece, Point{ x: x, y: ghost_position.y });

        // Render the next piece
        display.set_text("Next piece:", left_margin, 7, Color::Red, Color::Black);
        let next_piece = self.piece_bag.peek();
        self.render_piece(display, &next_piece, Point{ x: (left_margin as i32) + 2, y: 9 });
    }

    fn display_game_over_screen(&self, display: &mut Display) {
        display.clear_buffer();

        // Render a game over message
        display.set_text("Game Over!", 10, 10, Color::Red, Color::Black);

        // Display the player's score
        let score_text = format!("Your Score: {}", self.score);
        display.set_text(&score_text, 10, 12, Color::Red, Color::Black);

        // Prompt the player to restart or exit
        display.set_text("Press 'R' to restart or 'Q' to quit.", 10, 14, Color::Red, Color::Black);

        display.render();
    }


    fn render_piece(&self, display: &mut Display, piece: &Piece, origin: Point) {
        let color = piece.color;

        piece.each_point(&mut |row, col| {
            let x = (origin.x + 2 * col) as u32;
            let y = (origin.y + row) as u32;
            display.set_text(" ", x, y, color, color);
            display.set_text(" ", x + 1, y, color, color);
        });
    }

    /// Moves the current piece in the specified direction. Returns true if the piece could be moved and
    /// didn't collide.
    fn move_piece(&mut self, x: i32, y: i32) -> bool {
        let new_position = Point{
            x: self.piece_position.x + x,
            y: self.piece_position.y + y,
        };
        if self.board.collision_test(&self.piece, new_position) {
            false
        } else {
            self.piece_position = new_position;
            true
        }
    }

    /// Rotates the current piece in the specified direction. Returns true if the piece could be rotated
    /// without any collisions.
    fn rotate_piece(&mut self, direction: Direction) -> bool {
        let mut new_piece = self.piece.clone();
        new_piece.rotate(direction);

        if self.board.collision_test(&new_piece, self.piece_position) {
            false
            //GAME OVER
        } else {
            self.piece = new_piece;
            true
        }
    }

    /// Positions the current piece at the top of the board. Returns true if the piece can be placed without
    /// any collisions.
    fn place_new_piece(&mut self) -> bool {
        let origin = Point{
            x: ((BOARD_WIDTH - (self.piece.shape.len() as u32)) / 2) as i32,
            y: 0,
        };
        if self.board.collision_test(&self.piece, origin) {
            false
        } else {
            self.piece_position = origin;
            true
        }
    }

    /// Advances the game by moving the current piece down one step. If the piece cannot move down, the piece
    /// is locked and the game is set up to drop the next piece.  Returns true if the game could be advanced,
    /// false if the player has lost.
    fn advance_game(&mut self) -> bool {
        if !self.move_piece(0, 1) {
            self.board.lock_piece(&self.piece, self.piece_position);

            let lines_cleared = self.board.clear_lines();
            if lines_cleared > 0 {
                // Update the score based on the number of lines cleared
                self.score += match lines_cleared {
                    1 => 40,   // Scoring for clearing one line
                    2 => 100,  // Scoring for clearing two lines
                    3 => 300,  // Scoring for clearing three lines
                    4 => 1200, // Scoring for clearing four lines 
                    _ => 0,    // Default scoring for other cases
                };

            self.total_lines += lines_cleared;

            if lines_cleared > 0 && self.total_lines >= self.level * 10 {
                // Level up every 10 lines cleared
                self.level += 1;
            }

        }


            self.piece = self.piece_bag.pop();

            if !self.place_new_piece() {

                if self.piece_position.y <= HIDDEN_ROWS as i32 {
                    //GameOver::TopOut.description();
                    self.game_over = true;
                    return false;
                } else if self.board.collision_test(&self.piece, self.piece_position) {
                    //GameOver::LockOut.description();
                    self.game_over = true;
                    return false;
                } else {
                    //GameOver::BlockOut.description();
                    self.game_over = true;
                    return false;
                }

                
            }

        }

        true
    }

    /// Drops the current piece to the lowest spot on the board where it fits without collisions and
    /// advances the game.
    fn drop_piece(&mut self) -> bool {
        while self.move_piece(0, 1) {}
        self.advance_game()
    }

    fn keypress(&mut self, key: Key) {
        match key {
            Key::Left => self.move_piece(-1, 0),
            Key::Right => self.move_piece(1, 0),
            Key::Down => self.advance_game(),
            Key::Up => self.rotate_piece(Direction::Left),
            Key::Space => self.drop_piece(),
            Key::Char('q') => self.rotate_piece(Direction::Left),
            Key::Char('e') => self.rotate_piece(Direction::Right),
            _ => false,
        };
    }


    fn play(&mut self, display: &mut Display) {
        let (tx_event, rx_event) = mpsc::channel();

        // Spawn a thread which sends periodic game ticks to advance the piece
        {
            // Formula: speed (in milliseconds) = 1000 - (level * 50)
            let level_speed = 1000 - (self.level * 50);

            let tx_event = tx_event.clone();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(level_speed.into()));
                    tx_event.send(GameUpdate::Tick).unwrap();
                };
            });
        }

        // Spawn a thread which listens for keyboard input
        {
            let tx_event = tx_event.clone();
            thread::spawn(move || {
                let stdin = &mut std::io::stdin();

                loop {
                    match get_input(stdin) {
                        Some(k) => tx_event.send(GameUpdate::KeyPress(k)).unwrap(),
                        None => ()
                    }
                }
            });
        }

        // Main game loop. The loop listens and responds to timer and keyboard updates received on a channel
        // as sent by the threads spawned above.
loop {
        display.clear_buffer();
        if self.game_over {
            self.display_game_over_screen(display);
        } else {
            self.render(display);
        }
        display.render();

        match rx_event.recv() {
            Ok(update) => {
                match update {
                    GameUpdate::KeyPress(key) => {
                        if !self.game_over {
                            match key {
                                Key::Char('z') | Key::CtrlC => {
                                    if self.game_over {
                                        break;
                                    }
                                }
                                k => {
                                    if self.game_over {
                                        // If the game is over, pressing 'R' restarts the game
                                        if k == Key::Char('r') {
                                            *self = Game::new(); // Restart the game
                                            self.game_over = false;
                                            continue;
                                        }
                                    } else {
                                        self.keypress(k);
                                    }
                                }
                            };
                        }
                    }
                    GameUpdate::Tick => {
                        if !self.game_over {
                            self.advance_game();
                        }
                    }
                }
            }
            Err(err) => panic!("{}", err),
        }
    }
}
}

fn get_input(stdin: &mut std::io::Stdin) -> Option<Key> {
    use std::io::Read;

    let c = &mut [0u8];
    match stdin.read(c) {
        Ok(_) => {
            match std::str::from_utf8(c) {
                Ok("w") => Some(Key::Up),
                Ok("a") => Some(Key::Left),
                Ok("s") => Some(Key::Down),
                Ok("d") => Some(Key::Right),
                Ok(" ") => Some(Key::Space),
                Ok("\x03") => Some(Key::CtrlC),
                // Escape sequence started - must read two more bytes.
                Ok("\x1b") => {
                    let code = &mut [0u8; 2];
                    match stdin.read(code) {
                        Ok(_) => {
                            match std::str::from_utf8(code) {
                                Ok("[A") => Some(Key::Up),
                                Ok("[B") => Some(Key::Down),
                                Ok("[C") => Some(Key::Right),
                                Ok("[D") => Some(Key::Left),
                                _ => None
                            }
                        },
                        Err(msg) => panic!("could not read from standard in: {}", msg)
                    }
                },
                Ok(n) => Some(Key::Char(n.chars().next().unwrap())),
                _ => None
            }
        },
        Err(msg) => panic!("could not read from standard in: {}", msg)
    }
}

fn main() {
    let display = &mut Display::new(BOARD_WIDTH * 2 + 100, BOARD_HEIGHT + 2);
    let game = &mut Game::new();

    let _restorer = terminal::set_terminal_raw_mode();

    game.play(display);
}