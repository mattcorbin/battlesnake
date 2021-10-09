#[macro_use]
extern crate rocket;

use rand::{thread_rng, Rng};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Rocket};

use crate::moves::*;

mod graph;
mod moves;

//     Y
//   .-^-.
//   /     \      .- ~ ~ -.
//  ()     ()    /   _ _   `.                     _ _ _
//   \_   _/    /  /     \   \                . ~  _ _  ~ .
//     | |     /  /       \   \             .' .~       ~-. `.
//     | |    /  /         )   )           /  /             `.`.
//     \ \_ _/  /         /   /           /  /                `'
//      \_ _ _.'         /   /           (  (
//                      /   /             \  \
//                     /   /               \  \
//                    /   /                 )  )
//                   (   (                 /  /
//                    `.  `.             .'  /
//                      `.   ~ - - - - ~   .'
//                         ~ . _ _ _ _ . ~

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    id: String,
    timeout: i32,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Battlesnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Coord>,
    head: Coord,
    length: i32,
    shout: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Board {
    height: isize,
    width: isize,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattlesnakeInfoResponse {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRequest {
    game: Game,
    turn: isize,
    board: Board,
    you: Battlesnake,
}

#[derive(Debug, Serialize, Deserialize)]
struct MoveResponse {
    #[serde(rename = "move")]
    mv: String,
    shout: Option<String>,
}

#[get("/")]
fn index() -> status::Custom<content::Json<String>> {
    let battlesnake_info_response = BattlesnakeInfoResponse {
        apiversion: "1".to_string(),
        author: "mattcorbin".to_string(),
        color: "#f05627".to_string(),
        head: "scarf".to_string(),
        tail: "round-bum".to_string(),
        version: "1.0.0".to_string(),
    };
    status::Custom(
        Status::Ok,
        content::Json(
            serde_json::to_string(&battlesnake_info_response)
                .expect("failed to serialize battlesnake info response"),
        ),
    )
}

#[post("/start", format = "json", data = "<_game_request>")]
fn start(_game_request: Json<GameRequest>) {}

#[post("/move", format = "json", data = "<game_request>")]
fn mv(game_request: Json<GameRequest>) -> status::Custom<content::Json<String>> {
    let mv = compute_move(&game_request.into_inner());
    let mut rng = thread_rng();
    let shout;
    match rng.gen_range(0..10) {
        0 => shout = Some("bork bork".to_string()),
        _ => shout = None,
    }
    let move_response = MoveResponse { mv, shout };
    status::Custom(
        Status::Ok,
        content::Json(
            serde_json::to_string(&move_response).expect("failed to serialize move response"),
        ),
    )
}

#[post("/end", format = "json", data = "<_game_request>")]
fn end(_game_request: Json<GameRequest>) {}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, start, mv, end])
}
