

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;


pub struct GameState {
    pub game_mode: GameMode,
    pub map: Map,
    pub teams: Vec<Team>,
    pub surface: Surface,
    pub mobs: Vec<Mob>,
}

impl GameState {
    pub fn new(game_mode: GameMode, map: Map, teams: Vec<Team>, surface: Surface) -> GameState {
        GameState {
            game_mode: game_mode,
            map: map,
            teams: teams,
            surface: surface,
        }
    }

    pub fn update(&mut self) {
        // Update all units
        for team in &mut self.teams {
            for player in &mut team.players {
                player.update();
            }
            for mob in &mut team.mobs {
                mob.update();
            }
        }
    }

    pub fn run(&mut self) {
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();
        let mut thread = thread::spawn(move || {
            loop {
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(_) => {
                        thread::sleep(Duration::from_millis(1000));
                    }
                }
            }
        });
        self.update();
        tx.send(()).unwrap();
        thread.join().unwrap();
    }
}