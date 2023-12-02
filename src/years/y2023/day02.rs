use adventofcode_lmh01_lib::read_file;
use miette::{Result, miette};

// too high - 2265

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day02.txt")?;
    let mut games = Vec::new();
    for line in content {
        match Game::try_from(line.split(':').collect::<Vec<&str>>()[1]) {
            Ok(game) => games.push(game),
            Err(e) => return Err(miette!("Unable to create game: {}", e)),
        }
    }
    let mut sum = 0;
    for (id, game) in games.iter().enumerate() {
        // add id +1 because game id's start at 1 and not at 0
        if game.validate(13, 14, 12) {
            sum += id+1;
        }
    }
    println!("GameID sum: {sum}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let mut content = read_file("input/y2023/day02.txt")?;
    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    draws: Vec<Draw>,
}

impl Game {

    /// Validate if the input numbers are <= the used numbers
    fn validate(&self, green: u8, blue: u8, red: u8) -> bool {
        for draw in &self.draws {
            if !draw.validate(green, blue, red) {
                return false
            }
        }
        true
    }
}

impl TryFrom<&str> for Game {
    type Error = String;

    /// Input has to be something like this: `3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green``
    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let chunks = value.split(';').collect::<Vec<&str>>();
        let mut draws = Vec::new();
        for chunk in chunks {
            match Draw::try_from(chunk) {
                Ok(draw) => draws.push(draw),
                Err(e) => return Err(format!("Unable to create game, draw is invalid: {e}")),
            }
        }
        Ok( Self {
            draws,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Draw {
    green: u8,
    blue: u8,
    red: u8,
}

impl Draw {

    fn new(green: u8, blue: u8, red: u8) -> Self {
        Self {
            green,
            blue,
            red
        }
    }

    /// Validate if the input numbers are <= the used numbers
    fn validate(&self, green: u8, blue: u8, red: u8) -> bool {
        self.green <= green && self.blue <= blue && self.red <= red
    }
}

impl TryFrom<&str> for Draw {
    type Error = String;

    /// Input has to be something like this: `3 blue, 4 red`
    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        let chunks = value.split(',').collect::<Vec<&str>>();
        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;
        for chunk in chunks {
            let pair = chunk.trim().split(' ').collect::<Vec<&str>>();
            let num = pair[0].trim().parse::<u8>().unwrap();
            match pair[1].trim() {
                "green" => green = num,
                "blue" => blue = num,
                "red" => red = num,
                _ => (),
            }
        }

        Ok(Self {
            green,
            blue,
            red,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::years::y2023::day02::{Game, Draw};


    #[test]
    fn test_game_try_from() {
        assert_eq!(Game::try_from("1 green, 2 blue, 3 red; 4 green, 5 blue, 6 red; 7 green, 8 blue, 9 red"), Ok(Game { draws: vec![Draw::new(1, 2, 3), Draw::new(4, 5, 6), Draw::new(7, 8, 9)]}))
    }

    #[test]
    fn test_game_validate() {
        assert!(Game::try_from("1 green, 2 blue, 3 red; 4 green, 5 blue, 6 red; 7 green, 8 blue, 9 red").unwrap().validate(10, 10, 10));
        assert!(!Game::try_from("1 green, 2 blue, 3 red; 4 green, 5 blue, 6 red; 7 green, 8 blue, 9 red").unwrap().validate(6, 6, 6));
    }

    #[test]
    fn test_draw_try_from() {
        assert_eq!(Draw::try_from("1 blue, 2 red, 3 green"), Ok(Draw::new(3, 1, 2)));
        assert_eq!(Draw::try_from("2 red, 5 green, 10 blue"), Ok(Draw::new(5, 10, 2)));
        assert_eq!(Draw::try_from("4 green, 12 red, 20 blue"), Ok(Draw::new(4, 20, 12)));
    }

    #[test]
    fn test_draw_validate() {
        assert!(Draw::new(9, 20, 30).validate(10, 20, 30));
        assert!(!Draw::new(10, 20, 30).validate(9, 20, 30));
    }

}