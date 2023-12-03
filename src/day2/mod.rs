use std::io::{Error, ErrorKind};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn new(red: u32, green: u32, blue: u32) -> Game {
        Game { red, green, blue }
    }

    pub fn set_value(&mut self, color: &str, value: &u32) {
        match color {
            "red" => {
                self.red = *value;
            }
            "green" => {
                self.green = *value;
            }
            "blue" => {
                self.blue = *value;
            }
            _ => {}
        }
    }
}

pub fn parse_game(game_output: &str) -> Result<Vec<Game>, Error> {
    let mut games = vec![];
    let mut digit = String::from("");
    let mut cube_color = String::from("");
    let mut game = Game {
        red: 0,
        green: 0,
        blue: 0,
    };
    let mut last_alphabetic = false;
    for g in game_output.chars() {
        if g.is_numeric() {
            println!("digit: {}", g);
            if last_alphabetic && digit.len() > 0 {
                let parsed_digit = match digit.parse::<u32>() {
                    Ok(d) => d,
                    Err(e) => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Error parsing digit {}: {}", g, e),
                        ))
                    }
                };
                game.set_value(&cube_color, &parsed_digit);
                digit = String::from("");
                cube_color = String::from("");
                last_alphabetic = false;
            }
            digit.push(g)
        }
        if g.is_alphabetic() {
            cube_color.push(g);
            last_alphabetic = true;
        }

        if g == ';' {
            digit = String::from("");
            cube_color = String::from("");
            games.push(game);
            game = Game {
                red: 0,
                green: 0,
                blue: 0,
            };
        }
    }
    return Ok(games);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game_output = "red: 1; green: 2; blue: 3; red: 4; green: 5; blue: 6;";
        let games = parse_game(game_output).unwrap();
        println!("games: {:?}", games);
        assert_eq!(games.len(), 2);
        assert_eq!(games[0].red, 1);
        assert_eq!(games[0].green, 2);
        assert_eq!(games[0].blue, 3);
        assert_eq!(games[1].red, 4);
        assert_eq!(games[1].green, 5);
        assert_eq!(games[1].blue, 6);
    }
}
