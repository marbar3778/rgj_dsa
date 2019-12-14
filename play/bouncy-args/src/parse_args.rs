#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32, // need to be made public to use in main.rs
}

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
    SmallInteger(u32),
    WidthTooSmall(u32),
    HeightTooSmall(u32),
}

pub fn parse_args() -> Result<Frame, ParseError> {
    use self::ParseError::*;

    let mut args = std::env::args().skip(1);

    let width_str = match args.next() {
        None => return Err(TooFewArgs),
        Some(width_str) => width_str,
    };

    let height_str = match args.next() {
        None => return Err(TooFewArgs),
        Some(height_str) => height_str,
    };

    match args.next() {
        Some(_) => return Err(TooManyArgs),
        None => (),
    }

    let width = match width_str.parse() {
        Err(_) => return Err(InvalidInteger(width_str)),
        Ok(width) => width,
    };

    if width < 2 {
        return Err(WidthTooSmall(width));
    };

    let height = match height_str.parse() {
        Err(_) => return Err(InvalidInteger(height_str)),
        Ok(height) => height,
    };
    
    if height < 2 {
        return Err(WidthTooSmall(width));
    };

    Ok(Frame {
        width,
        height,
    })
}
