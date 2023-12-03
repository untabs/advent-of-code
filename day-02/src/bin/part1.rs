use std::fs;

fn main() {
    let file_path = "input1.txt";
    let input = fs::read_to_string(file_path).unwrap();
    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    let data = input.split("\n").collect::<Vec<&str>>();
    let mut id_sum = 0;

    for game in data {
        if game == "" || game == "\n" {
            continue;
        }
        let mut is_valid = true;
        let id = game.split(": ").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1];
        let sets = game.split(": ").collect::<Vec<&str>>()[1]
            .split("; ")
            .collect::<Vec<&str>>();
        for set in sets {
            let cubes = set.split(", ").collect::<Vec<&str>>();
            for cube in cubes {
                let number_compare: i32 = if cube.contains("red") {
                    12
                } else if cube.contains("green") {
                    13
                } else {
                    14
                };
                // println!("{(cube.chars()).retain(|x| x.is_numeric())}");
                let mut number = 0;
                let mut power = 1;
                for char in cube.chars() {
                    if !char.is_numeric() {
                        break;
                    }
                    number *= power;
                    number += char.to_string().parse::<i32>().unwrap();
                    power *= 10;
                }
                if number > number_compare {
                    is_valid = false;
                }
            }
        }
        if is_valid {
            id_sum += id.to_string().parse::<i32>().unwrap();
        }
    }
    return id_sum.to_string();
}

#[cfg(test)]
mod test {
    #[test]
    fn given_io() {
        use super::*;

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!("8", result)
    }
    #[test]
    fn one_line() {
        use super::*;

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = part1(input);
        assert_eq!("1", result);
    }
}
