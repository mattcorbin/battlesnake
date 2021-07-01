#[macro_use]
extern crate rocket;

use rand::{Rng, thread_rng};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    id: String,
    timeout: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Coord {
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
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattlesnakeInfoResponse {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GameRequest {
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
        color: "#092de3".to_string(),
        head: "pixel".to_string(),
        tail: "pixel".to_string(),
    };
    status::Custom(
        Status::Ok,
        content::Json(
            serde_json::to_string(&battlesnake_info_response).expect("failed to serialize battlesnake info response")
        ),
    )
}

#[post("/start", format = "json", data = "<game_request>")]
fn start(game_request: Json<GameRequest>) {
    println!("{:?}", game_request)
}

#[post("/move", format = "json", data = "<game_request>")]
fn mv(game_request: Json<GameRequest>) -> status::Custom<content::Json<String>> {
    println!("{:?}", game_request);
    let mut rng = rand::thread_rng();
    let moves = vec![
        "up".to_string(),
        "down".to_string(),
        "left".to_string(),
        "right".to_string(),
    ];
    let index: usize = rng.gen_range(0..4);
    println!("move: {}", moves[index]);
    let move_response = MoveResponse {
        mv: moves[index].clone(),
        shout: None,
    };
    status::Custom(
        Status::Ok,
        content::Json(serde_json::to_string(&move_response).expect("failed to serialize move response")
        ),
    )
}

#[post("/end", format = "json", data = "<game_request>")]
fn end(game_request: Json<GameRequest>) {
    println!("{:?}", game_request)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/start", routes![start])
        .mount("/move", routes![mv])
        .mount("/end", routes![end])
}
