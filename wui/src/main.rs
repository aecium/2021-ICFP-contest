#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use std::fs;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::sync::atomic::{AtomicBool, Ordering};
use rocket::State;
use rocket::fairing::AdHoc;
use rocket::http::Status;

struct SolverRunning {
    running: AtomicBool
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SolverStatus {
    done: bool,
    line: String,
}

struct Tx(flume::Sender<SolverStatus>);
struct Rx(flume::Receiver<SolverStatus>);

#[get("/push/<event>")]
fn push(event: String, tx: &State<Tx>) -> Result<(), Status> {
    let status = SolverStatus{done: true, line: event};
    tx.0.try_send(status).map_err(|_| Status::ServiceUnavailable)
}

#[get("/pop")]
fn pop(rx: &State<Rx>) -> Option<Json<SolverStatus>> {
    let status = rx.0.recv().ok();
    return match status {
        Some(s) => Some(Json(s)),
        None => None,
    }
    //rx.0.try_recv().ok()
}


#[get("/status/run")]
fn status_run(status: &State<SolverRunning>) -> Value {
    status.running.fetch_or(true, Ordering::Relaxed);
    let running = status.running.load(Ordering::Relaxed);
    json!({"running": running})
}

#[get("/status/stop")]
fn status_stop(status: &State<SolverRunning>) -> Value {
    status.running.fetch_and(false, Ordering::Relaxed);
    let running = status.running.load(Ordering::Relaxed);
    json!({"running": running})
}

#[get("/status")]
fn status(status: &State<SolverRunning>) -> Value {
    let running = status.running.load(Ordering::Relaxed);
    json!({"running": running})
}


#[get("/sandwich")]
fn sandwich() -> &'static str {
    "🥪"
}

#[get("/problem/<id>/solve/<solver>")]
fn solve(id: usize, solver: &str, tx: &State<Tx>) -> Result<Value, Error> {
    let child;
    if solver == "rotate-and-shift" {
        child = Command::new("/usr/bin/go")
        .arg("run")
        .arg("main.go")
        .arg("-p")
        .arg(format!("{}",id))
        .current_dir("/mnt/e/workspace/2021-ICFP-contest/go/rotate-and-shift")
        .stdout(Stdio::piped())
        .spawn()?;
    }else {
        child = Command::new("../target/release/icfp_2021")
                    .arg("solve")
                    .arg(format!("../problems/{}.json", id))
                    .arg(solver)
                    .stdout(Stdio::piped())
                    .spawn()?;
    }

    let stdout = child.stdout.ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;
    let reader = BufReader::new(stdout);

    let mut lines = "".to_string();
    reader
        .lines()
        .for_each(|line| {
            let line = line.unwrap();
            println!("{}", line);
            lines.push('\n');
            lines.push_str(&line);
            let status = SolverStatus{done: false, line: line.to_string()};
            tx.0.try_send(status).map_err(|_| Status::ServiceUnavailable);
        });

    let status = SolverStatus{done: true, line: "".to_string()};
    tx.0.try_send(status).map_err(|_| Status::ServiceUnavailable);

    Result::Ok(json!({"id": id, "solver": solver, "output": lines}))
}

#[get("/taco")]
fn taco() -> &'static str {
    "🌮"
}

#[get("/problems/list")]
fn list_problems() -> Value {
    json!({"files": directory_files("../problems")})
}

#[get("/solutions/list")]
fn list_solutions() -> Value {
    json!({"files": directory_files("../solutions")})
}

fn directory_files(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut result = vec![];

    for path in paths{
        if let Ok(path) = path {
            if path.file_type().unwrap().is_file() {
                let name = path.file_name().as_os_str().to_str().unwrap().to_string();
                if name.ends_with(".json") {
                    result.push(name);
                }
            }
        }
    }
    result
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = flume::bounded(32);
    rocket::build()
        .manage(SolverRunning { running: AtomicBool::new(false) })
        .manage(Tx(tx))
        .manage(Rx(rx))
        .mount("/queue", routes![push, pop])
        .mount("/", routes![status, status_run, status_stop])
        .mount("/", routes![sandwich])
        .mount("/", routes![taco])
        .mount("/", routes![solve])
        .mount("/", routes![list_problems])
        .mount("/", routes![list_solutions])
        .mount("/problems", FileServer::from(relative!("../problems")))
        .mount("/solutions", FileServer::from(relative!("../solutions")))
        .mount("/", FileServer::from(relative!("../web")).rank(11))
}
