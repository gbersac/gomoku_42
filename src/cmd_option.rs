use std::fs::File;
use std::io::Read;
use display::console::Player;
use clap;
use board::{GoBoard};

const DEFAULT_PLAYER: &'static str = "ia";
const DEFAULT_FRIEND: &'static str = "ia";
const DEFAULT_LAYERS: &'static str = "3";
const DEFAULT_INFO: &'static str = "true";

pub fn file_as_string(file_name: &str) -> String {
    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    let _  = f.read_to_string(&mut s);
    s
}

#[derive(Debug)]
pub struct CmdOption {
    pub player: Player,
    pub friend: Player,
    pub layers: u32,
    pub human_help: bool,
    pub info: bool,
    pub debug_map: bool,
    pub init_map: GoBoard
}

impl CmdOption {
    pub fn parse(m: &clap::ArgMatches) -> CmdOption {
        let playero = m.value_of("player").unwrap_or(DEFAULT_PLAYER);
        let friendo = m.value_of("friend").unwrap_or(DEFAULT_FRIEND);
        let layerso = m.value_of("layers").unwrap_or(DEFAULT_LAYERS);
        let no_helpo = !m.is_present("nohelper");
        let infoo = m.value_of("info").unwrap_or(DEFAULT_INFO);
        let debug_mapo = m.is_present("debug_map");
        let init_map = if m.is_present("init_map") {
            let file_name = m.value_of("init_map").unwrap();
            let file_str = file_as_string(file_name);
            GoBoard::parse_with_size(&file_str.to_string())
        } else {
            GoBoard::default()
        };
        if layerso.parse::<u32>().is_err() {
            panic!("Layers cli option must be an unsigned interger.");
        }
        CmdOption {
            player: Player::from_str(playero),
            friend: Player::from_str(friendo),
            layers: layerso.parse::<u32>().unwrap(),
            human_help: no_helpo,
            info: infoo.parse::<bool>().unwrap(),
            debug_map: debug_mapo,
            init_map: init_map
        }
    }
}
