use std::io;
use std::io::stdin;
use std::ops::Add;
extern crate rand;
use rand::Rng;
use crate::core::terrain::BlockType;
use crate::core::game::Game;

mod core;

fn main() {
    let mut game: Game = Game::create();
    while game.is_finished() {
        print!("{}[2J", 27 as char);
        run(&mut game);
    }
    if game.is_bomb_found == true {
        println!("You Lost...!");
    } else {
        println!("You Win...!");
    }
}

fn read_coords() -> (usize, usize) {
    let mut coords: Vec<usize> = Vec::new();
    let mut command: String = String::new();
    stdin().read_line(&mut command).expect("Error when reading command!");
    for c in command.trim().split(",") {
        let num: u32 = c.trim().parse().unwrap();
        coords.push(num as usize);
    }
    (coords[0] - 1, coords[1] - 1)
}

fn draw_screen(game: &Game) {
    print!("    ");
    for x in 0..game.terrain.blocks.len() {
        print!("  {}  ", num_to_string_with_zeros((x + 1) as u32));
    }
    println!(" ");
    for (x, vec) in game.terrain.blocks.iter().enumerate() {
        println!(" ");
        print!(" {} ", num_to_string_with_zeros((x + 1) as u32));
        for (y, block) in vec.iter().enumerate() {
            print!("  ");
            match block {
                BlockType::CLOSED => print!("|+|"),
                BlockType::BOMB => print!("|+|"),
                BlockType::OPEN(n) => print!("|{}|", n),
                (_) => print!("|?|")
            }
            print!(" ");
        }
        println!(" ");
    }
}

fn run(game: &mut Game) {
    draw_screen(game);
    println!(" --- Please Input Coords --- ");
    let coords: (usize, usize) = read_coords();
    game.open_block(coords);
}

fn num_to_string_with_zeros(num: u32) -> String {
    const MX_0: u32 = 2;
    let mut zero_count: u32 = 0;
    match num {
        0 => zero_count = 1,
        (_) => while (10 as i32).pow(zero_count) <= num as i32 {
            zero_count += 1;
        }
    } // DO NOT INPUT 100+ :D
    let zeros = String::from("0".repeat( (MX_0 - zero_count) as usize));
    format!("{}{}", zeros, num)
}