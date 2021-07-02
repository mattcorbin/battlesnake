use rand::Rng;

use crate::{Coord, GameRequest};

#[derive(Debug)]
struct Directions {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

fn list_of_potential_directions(potential_directions: Directions) -> Vec<&'static str> {
    let mut moves = Vec::new();
    if potential_directions.up {
        moves.push("up");
    }
    if potential_directions.down {
        moves.push("down");
    }
    if potential_directions.left {
        moves.push("left");
    }
    if potential_directions.right {
        moves.push("right");
    }
    return moves
}

fn find_move(moves: Vec<&str>, potential_directions: Directions) -> String {
    println!("{:?}\n{:?}\n", moves, potential_directions);
    let mut rng = rand::thread_rng();
    let possible_directions = list_of_potential_directions(potential_directions);

    if possible_directions.len() == 0 {
        // we ded
        return "up".to_string();
    }
    let mv = possible_directions[rng.gen_range(0..possible_directions.len())];
    match mv {
        "up" => {
            return mv.to_string();
        }
        "down" => {
            return mv.to_string();
        }
        "right" => {
            return mv.to_string();
        }
        "left" => {
            return mv.to_string();
        }
        _ => return "up".to_string(),
    }
}

fn nearby(head_loc: &Coord, coord: &Coord) -> bool {
    if head_loc.x == coord.x {
        if (head_loc.y - coord.y).abs() == 1 {
            return true;
        }
    } else if head_loc.y == coord.y {
        if (head_loc.x - coord.x).abs() == 1 {
            return true;
        }
    }
    false
}

pub fn compute_move(game: GameRequest) -> String {
    let moves = vec!["up", "down", "left", "right"];
    let mut potential_directions = Directions {
        up: true,
        down: true,
        right: true,
        left: true,
    };
    let head_loc = game.you.head;
    if head_loc.x == 0 {
        println!("left false due to border");
        potential_directions.left = false
    }
    if head_loc.x == game.board.width - 1 {
        println!("right false due to border");
        potential_directions.right = false
    }
    if head_loc.y == 0 {
        println!("down false due to border");
        potential_directions.down = false
    }
    if head_loc.y == game.board.height - 1 {
        println!("up false due to border");
        potential_directions.up = false
    }
    for snake in game.board.snakes {
        for segment in snake.body {
            if !nearby(&head_loc, &segment) {
                continue;
            }
            if head_loc.x - segment.x == 1 {
                println!("left false due to snake");
                potential_directions.left = false
            } else if head_loc.x - segment.x == -1 {
                println!("right false due to snake");
                potential_directions.right = false
            }
            if head_loc.y - segment.y == 1 {
                println!("down false due to snake");
                potential_directions.down = false
            } else if head_loc.y - segment.y == -1 {
                println!("up false due to snake");
                potential_directions.up = false
            }
        }
        if game.you.length > snake.length && game.you.id != snake.id {
            if head_loc.x - snake.head.x == 1 {
                potential_directions.left = true
            } else if head_loc.x - snake.head.x == -1 {
                potential_directions.right = true
            }
            if head_loc.y - snake.head.y == 1 {
                potential_directions.down = true
            } else if head_loc.y - snake.head.y == -1 {
                potential_directions.up = true
            }
        }
    }
    find_move(moves, potential_directions)
}
