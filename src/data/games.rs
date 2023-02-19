use super::shapes::Shape;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub struct Game {
    pub local_rotations: i32,
    pub shape_index: i32,
    pub shape_collection: HashMap<i32, Shape>,
}


impl Game {
    pub fn new() -> Self {
        let mut shape_collection: HashMap<i32, Shape> = HashMap::new();
        shape_collection.insert(0, Shape::new(0));
        Game {
            local_rotations: shape_collection[&0].rotations,
            shape_index: 0,
            shape_collection: shape_collection,
        }
    }

    //&self.shape_index
    pub fn get_shape(&mut self, index: i32) -> &mut Shape {
        if let std::collections::hash_map::Entry::Vacant(e) = self.shape_collection.entry(index) {
            e.insert(Shape::new(index));
            self.shape_collection.get_mut(&index).unwrap()
        } else {
            self.shape_collection.get_mut(&index).unwrap()
        }
    }

    pub fn get_shape_read_only(& mut self, index: i32) -> &Shape {
        if let std::collections::hash_map::Entry::Vacant(e) = self.shape_collection.entry(index) {
            e.insert(Shape::new(index));
            self.shape_collection.get(&index).unwrap()
        } else {
            self.shape_collection.get(&index).unwrap()
        }
    }

    fn get_range(start: i32, end: i32) -> Vec<i32> {
        if start > end {
            (end..=start).collect()
        } else {
            (start..=end).collect()
        }
    }


    pub fn swipe_left(&mut self, amount: i32) {
        self.shape_index = self.get_shape(self.shape_index).index + amount;
        for i in Self::get_range(self.get_shape(self.shape_index).index, self.get_shape(self.shape_index).index + amount) {
            match self.shape_collection.get(&i) {
                None => continue,
                Some(shape) => self.local_rotations += shape.rotations,
            }
        }
    }


    pub fn swipe_right(&mut self, amount: i32) {
        self.shape_index = self.get_shape(self.shape_index).index + amount;
        for i in Self::get_range(self.get_shape(self.shape_index).index, self.get_shape(self.shape_index).index - amount) {
            match self.shape_collection.get(&i) {
                None => continue,
                Some(shape) => self.local_rotations -= shape.rotations,
            }
        }
    }

    pub fn check_pattern_exists(&mut self, pattern: Vec<i32>) -> bool {
        self.shape_collection
            .values()
            .map(|value| &value.rotations)
            .zip(&pattern)
            .filter(|&(a, b)| a == b)
            .count()
            == pattern.len()
    }

    fn ask_input(input_text: String, error_text: String) -> Result<String, std::io::Error> {
        print!("{input_text}");
        if let Err(err) = io::stdout().flush() {
            println!("{error_text}, {err}");
            return Ok(
                Self::ask_input(input_text, error_text).expect("Function can only return OK(). ")
            );
        };
        let mut response = String::new();
        if let Err(err) = io::stdin().read_line(&mut response) {
            println!("{error_text}, {err}");
            return Ok(
                Self::ask_input(input_text, error_text).expect("Function can only return OK(). ")
            );
        };
        Ok(response)
    }

    pub fn print_game_snippet(&mut self) {
        let left_cube = self.get_shape(1).rotations;
        let middle_cube = self.get_shape(0).rotations;
        let right_cube = self.get_shape(-1).rotations;

        println!("       ____ ____ _____    ");
        println!("      /____/____/____/|     ",);
        println!(
            "/⎺⎺⎺⎺ | {0:^2} | {1:^2} | {2:^2} |/⎺⎺⎺⎺/",
            left_cube, middle_cube, right_cube,
        );
        println!("⎺⎺⎺⎺⎺ ⎺⎺⎺⎺⎺ ⎺⎺⎺⎺ ⎺⎺⎺⎺ ⎺⎺⎺⎺⎺");
    }

    pub fn game_loop(&mut self) {
        println!("Starting game...");
        println!("Please Select an action.");
        println!("Type: \"up x\" to rotate the middle cube up x times.");
        println!("Type: \"down x\" to rotate the middle cube down x times.");
        println!("Type: \"left x\" to move the current cube left x times.");
        println!("Type: \"right x\" To move the current cube right x times.");
        println!("Type: \"quit\" To exit the program.\n");
        self.print_game_snippet();
        loop{
            //println!("{}", self.shape.index);
            self.do_action();
            self.print_game_snippet();
            if self.check_pattern_exists(vec![1, 1, 1]) {
                println!("You win!!!");
                std::process::exit(0);
            }
        }
    }

    fn do_action(&mut self) {
        let mut chosen_action = String::new();
        let re = Regex::new(r"^(up|down|left|right) \d+|^(?i:quit)$").unwrap();
        loop {
            chosen_action = Self::ask_input(
                "Please input an action: ".to_string(),
                "Proccessing action went wrong. My bad.".to_string(),
            )
            .expect("Function can only return OK(). ")
            .strip_suffix("\r\n")
            .unwrap()
            .to_string();
            if re.is_match(&chosen_action) {
                let chosen_action_split = chosen_action.split(" ").collect::<Vec<&str>>();
                match chosen_action_split[0] {
                    "up" => self.get_shape(self.shape_index).swipe_up(
                        chosen_action_split[1]
                            .parse::<i32>()
                            .expect("I just confirmed there's a number with regex."),
                    ),
                    "down" => self.get_shape(self.shape_index).swipe_down(
                        chosen_action_split[1]
                            .parse::<i32>()
                            .expect("I just confirmed there's a number with regex."),
                    ),
                    "left" => self.swipe_left(
                        chosen_action_split[1]
                            .parse::<i32>()
                            .expect("I just confirmed there's a number with regex."),
                    ),
                    "right" => self.swipe_right(
                        chosen_action_split[1]
                            .parse::<i32>()
                            .expect("I just confirmed there's a number with regex."),
                    ),
                    "quit" => {
                        println!("Closing program...");
                        std::process::exit(0)
                    }
                    _ => println!("Proccessing action went wrong. My bad."),
                }
                println!("");
                break
            } else {
                println!("Invalid action selected. Please try again.");
                continue;
            }
        }
    }

}

