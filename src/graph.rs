use std::cmp::Ordering;
use std::collections::HashMap;

use petgraph::graphmap::UnGraphMap;

use crate::moves::Direction;
use crate::{Battlesnake, Board, Coord};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SpaceType {
    Empty,
    Food,
    Hazard,
    Occupied,
    EnemySnakeHead,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Space {
    pub location: Coord,
    pub space_type: SpaceType,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.location.x, self.location.y)
    }
}

impl Space {
    pub fn determine_direction(&self, other: &Space) -> Direction {
        let mut direction = Direction::Up;
        if self.location.x == other.location.x {
            match (self.location.y).cmp(&other.location.y) {
                Ordering::Greater => direction = Direction::Down,
                Ordering::Less => direction = Direction::Up,
                Ordering::Equal => direction = Direction::Up,
            }
        } else if self.location.y == other.location.y {
            match (self.location.x).cmp(&other.location.x) {
                Ordering::Greater => direction = Direction::Left,
                Ordering::Less => direction = Direction::Right,
                Ordering::Equal => direction = Direction::Right,
            }
        }
        direction
    }
}

fn can_win_fight(board: &Board, enemy_snake_head: &Coord) -> bool {
    board.snakes[0].health
        > board
            .snakes
            .iter()
            .filter(|x| x.head == *enemy_snake_head)
            .collect::<Vec<&Battlesnake>>()[0]
            .health
}

impl From<&Board> for UnGraphMap<Space, usize> {
    fn from(board: &Board) -> Self {
        let mut graph = UnGraphMap::new();

        let mut space_map: HashMap<Coord, Space> = HashMap::new();
        for x in 0..board.width {
            for y in 0..board.height {
                let coord = Coord { x, y };
                let space = Space {
                    location: coord,
                    space_type: SpaceType::Empty,
                };
                space_map.insert(coord, space);
            }
        }
        for food in &board.food {
            let mut space = space_map
                .get_mut(food)
                .expect("food coord must exist in map");
            space.space_type = SpaceType::Food;
        }
        for snake in &board.snakes {
            for segment in &snake.body {
                let mut space = space_map
                    .get_mut(segment)
                    .expect("segment coord must exist in map");
                space.space_type = SpaceType::Occupied;
            }
            let mut space = space_map
                .get_mut(&snake.head)
                .expect("head coord must exist in map");
            space.space_type = SpaceType::EnemySnakeHead;
        }
        for hazard in &board.hazards {
            let mut space = space_map
                .get_mut(hazard)
                .expect("segment coord must exist in map");
            space.space_type = SpaceType::Hazard;
        }

        for x in 0..board.width {
            for y in 0..board.height {
                let coord = Coord { x, y };
                let current_node = space_map.get(&coord).expect("coord must exist in map");
                if current_node.space_type == SpaceType::Occupied {
                    continue;
                }
                let adjacent_spots = vec![
                    Coord { x: x - 1, y },
                    Coord { x: x + 1, y },
                    Coord { x, y: y - 1 },
                    Coord { x, y: y + 1 },
                ];
                for spot in adjacent_spots.iter() {
                    if let Some(node) = space_map.get(spot) {
                        let weight;
                        match node.space_type {
                            SpaceType::Occupied => continue,
                            SpaceType::EnemySnakeHead => {
                                if can_win_fight(&board, &node.location) {
                                    weight = 0;
                                } else {
                                    weight = 2;
                                }
                            }
                            SpaceType::Hazard => weight = 1,
                            _ => weight = 0,
                        }
                        graph.add_edge(*current_node, *node, weight);
                    }
                }
            }
        }
        graph
    }
}

pub fn find_nearest_food(start: Space, graph: &UnGraphMap<Space, usize>) -> Space {
    let mut min_dist: f64 = -1.0;
    let mut nearest_food: Space = Space {
        location: Coord { x: 0, y: 0 },
        space_type: SpaceType::Empty,
    };
    for node in graph.nodes() {
        if node.space_type == SpaceType::Food {
            let diff_of_squares = (node.location.x - start.location.x).pow(2)
                + (node.location.y - start.location.y).pow(2);
            let distance = f64::sqrt(diff_of_squares as f64);
            if min_dist < 0.0 || distance < min_dist {
                min_dist = distance;
                nearest_food = node;
            }
        }
    }
    nearest_food
}
