use std::fmt::{Display, Formatter};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {

    Game { 
        frame: Frame {
            width: 60,
            height: 30,
        }, 
        ball: Ball {
            x: 2, 
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        }}
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+");
            for _ in 0..self.frame.width {
                write!(fmt, "-");
            }
            write!(fmt, "+\n")
        };
        top_bottom(fmt)?;
        for row in 0..self.frame.height {
            write!(fmt, "|");
                for col in 0..self.frame.width {
                    let c = if row == self.ball.y && col == self.ball.x {
                        'o'
                    } else {
                        ' '
                    };
                    write!(fmt, "{}", c);
                }
            write!(fmt, "|\n");
        }
        top_bottom(fmt)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        } else if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height -1 {
            self.vert_dir = VertDir::Up;
        } else {
            
        }
    }

    fn mv( &mut self) {
        match self.horiz_dir {
            HorizDir::Right => self.x += 1,
            HorizDir::Left => self.x -= 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

fn main() {
    let mut game = Game::new();
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_duration);
    }
}
