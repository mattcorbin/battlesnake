use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;

use crate::graph::{find_nearest_food, Space, SpaceType};
use crate::GameRequest;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => String::from("up"),
            Direction::Down => String::from("down"),
            Direction::Left => String::from("left"),
            Direction::Right => String::from("right"),
        }
    }
}

pub fn compute_move(game: &GameRequest) -> String {
    let graph = UnGraphMap::from(&game.board);
    let start = Space {
        location: game.you.head,
        space_type: SpaceType::EnemySnakeHead, // TODO: fix your head space type
    };

    let nearest_food = find_nearest_food(start, &graph);

    match astar(
        &graph,
        start,
        |finish| finish.location == nearest_food.location,
        |e| *e.2,
        |_| 0,
    ) {
        Some(path) => path.1[0].determine_direction(&path.1[1]).to_string(),
        None => Direction::Up.to_string(),
    }
}
