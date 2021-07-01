use rand::Rng;

use crate::{Coord, GameRequest};

#[derive(Debug)]
struct Directions {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

fn find_move(moves: Vec<&str>, directions: Directions) -> String {
    println!("{:?}\n{:?}\n", moves, directions);
    let mut rng = rand::thread_rng();
    if moves.len() == 0 {
        // we ded
        return "up".to_string();
    }
    let index: usize = rng.gen_range(0..moves.len());
    let mv = moves[index].clone();
    let move_selection;
    match mv {
        "up" => {
            if directions.up {
                move_selection = mv.to_string();
            } else {
                let new_moves: Vec<&str> = moves.into_iter().filter(|&item| item != mv).collect();
                move_selection = find_move(new_moves, directions);
            }
        }
        "down" => {
            if directions.down {
                move_selection = mv.to_string();
            } else {
                let new_moves: Vec<&str> = moves.into_iter().filter(|&item| item != mv).collect();
                move_selection = find_move(new_moves, directions);
            }
        }
        "right" => {
            if directions.right {
                move_selection = mv.to_string();
            } else {
                let new_moves: Vec<&str> = moves.into_iter().filter(|&item| item != mv).collect();
                move_selection = find_move(new_moves, directions);
            }
        }
        "left" => {
            if directions.left {
                move_selection = mv.to_string();
            } else {
                let new_moves: Vec<&str> = moves.into_iter().filter(|&item| item != mv).collect();
                move_selection = find_move(new_moves, directions);
            }
        }
        _ => move_selection = "up".to_string(),
    }
    return move_selection;
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
    let mut directions = Directions {
        up: true,
        down: true,
        right: true,
        left: true,
    };
    let head_loc = game.you.head;
    if head_loc.x == 0 {
        println!("left false due to border");
        directions.left = false
    }
    if head_loc.x == game.board.width - 1 {
        println!("right false due to border");
        directions.right = false
    }
    if head_loc.y == 0 {
        println!("down false due to border");
        directions.down = false
    }
    if head_loc.y == game.board.height - 1 {
        println!("up false due to border");
        directions.up = false
    }
    for snake in game.board.snakes {
        for segment in snake.body {
            if !nearby(&head_loc, &segment) {
                continue;
            }
            if head_loc.x - segment.x == 1 {
                println!("left false due to snake");
                directions.left = false
            } else if head_loc.x - segment.x == -1 {
                println!("right false due to snake");
                directions.right = false
            }
            if head_loc.y - segment.y == 1 {
                println!("down false due to snake");
                directions.down = false
            } else if head_loc.y - segment.y == -1 {
                println!("up false due to snake");
                directions.up = false
            }
        }
        if game.you.length > snake.length && game.you.id != snake.id {
            if head_loc.x - snake.head.x == 1 {
                directions.left = true
            } else if head_loc.x - snake.head.x == -1 {
                directions.right = true
            }
            if head_loc.y - snake.head.y == 1 {
                directions.down = true
            } else if head_loc.y - snake.head.y == -1 {
                directions.up = true
            }
        }
    }
    find_move(moves, directions)
}
