use board::Tile;

pub struct Team {
    color: Tile,
    captured: u32,
}

impl Team {
	fn new(color: Tile) -> Team {
	    Team {
	    	color: color,
	    	captured: 0,
	    }
	}

	/// Create all the teams of the game.

    pub fn new_teams() -> (Team, Team) {
    	(Team::new(Tile::BLACK), Team::new(Tile::WHITE))
    }

    pub fn get_tile(&self) -> Tile {
    	self.color
    }
}
