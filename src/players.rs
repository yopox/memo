use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::iter::Map;
use crate::stats::{Carac, Competence};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Player {
    name: String,
    max_hp: u16,
    hp: u16,
    stats: HashMap<Carac, u16>,
    abilities: HashMap<Competence, u16>,
    items: Vec<String>
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            max_hp: 10,
            hp: 10,
            stats: HashMap::new(),
            abilities: HashMap::new(),
            items: vec![]
        }
    }

    fn init(&mut self, str: u16, dex: u16, end: u16, int: u16, cha: u16) {
        self.stats.insert(Carac::Force, str);
        self.stats.insert(Carac::Dexterite, dex);
        self.stats.insert(Carac::Endurance, end);
        self.stats.insert(Carac::Intelligence, int);
        self.stats.insert(Carac::Charisme, cha);

        for comp in Competence::iter() {
            let (s1, s2) = comp.associated_carac();
            self.abilities.insert(comp, (*self.stats.get(&s1).unwrap() + *self.stats.get(&s2).unwrap()) * 2);
        }
    }
}

#[test]
fn init() {
    let mut players = HashMap::new();
    let mut skamos = Player::new("Skamos".to_string());
    skamos.init(11, 10, 8, 12, 10);
    players.insert("Skama".to_string(), skamos);
    save(&players);
}

pub fn save(players: &HashMap<String, Player>) {
    match ron::to_string(&players) {
        Ok(s) => {
            fs::write("data.ron", s);
        },
        Err(_) => {
            println!("Couldn't write data.ron !");
        }
    }
}

pub fn load() -> Option<HashMap<String, Player>> {
    return match fs::read_to_string("data.ron") {
        Ok(s) => match ron::from_str(s.as_str()) {
            Ok(hm) => {
                Some(hm)
            }
            Err(_) => {
                println!("Couldn't deserialize data.ron.");
                None
            }
        }
        Err(_) => {
            println!("Couldn't load data.ron.");
            None
        }
    }
}