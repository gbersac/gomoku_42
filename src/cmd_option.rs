use clap;

const DEFAULT_PLAYER : &'static str = "ia";
const DEFAULT_LAYERS : &'static str = "3";

#[derive(Debug)]
pub enum PlayerType {
    Solo,
    Multi,
    Ia
}

impl PlayerType {
	pub fn from_str(s: &str) -> PlayerType {
	    match s {
	        "ia"	=> PlayerType::Ia,
	        "solo"	=> PlayerType::Solo,
	        "multi" => PlayerType::Multi,
	        _		=> panic!("PlayerType cli option must be either ia, solo or multi")
	    }
	}
}


#[derive(Debug)]
pub struct CmdOption {
    player: PlayerType,
    layers: u32,
    human_help: bool
}

impl CmdOption {
	pub fn parse(m: &clap::ArgMatches) -> CmdOption {
		let playero = m.value_of("player").unwrap_or(DEFAULT_PLAYER);
		let layerso = m.value_of("layers").unwrap_or(DEFAULT_LAYERS);
		let no_helpo = !m.is_present("nohelper");
		if layerso.parse::<u32>().is_err() {
			panic!("Layers cli option must be an unsigned interger.");
		}
		CmdOption {
			player: PlayerType::from_str(playero),
			layers: layerso.parse::<u32>().unwrap(),
			human_help: no_helpo
		}
	}
}
