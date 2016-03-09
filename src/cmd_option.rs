use display::console::Player;
use clap;

const DEFAULT_PLAYER: &'static str = "ia";
const DEFAULT_FRIEND: &'static str = "ia";
const DEFAULT_LAYERS: &'static str = "3";
const DEFAULT_INFO: &'static str = "true";

#[derive(Debug)]
pub struct CmdOption {
    pub player: Player,
    pub friend: Player,
    pub layers: u32,
    pub human_help: bool,
    pub info: bool,
    pub debug_map: bool,
}

impl CmdOption {
    pub fn parse(m: &clap::ArgMatches) -> CmdOption {
        let playero = m.value_of("player").unwrap_or(DEFAULT_PLAYER);
        let friendo = m.value_of("friend").unwrap_or(DEFAULT_FRIEND);
        let layerso = m.value_of("layers").unwrap_or(DEFAULT_LAYERS);
        let no_helpo = !m.is_present("nohelper");
        let infoo = m.value_of("info").unwrap_or(DEFAULT_INFO);
        let debug_mapo = m.is_present("debug_map");
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
        }
    }
}
