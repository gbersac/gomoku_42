use board::Tile;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Team {
    color: Tile,
    captured: u32,
}

impl Team {

	/// The `new` constructor function returns the Team.

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

    pub fn get_ennemy_tile(&self) -> Tile {
    	self.color.ennemy()
    }

    pub fn captured(&self) -> u32 {
        self.captured
    }

    pub fn add_captured(&mut self, nb_captured: u32) {
        self.captured += nb_captured;
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Team
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        match self.get_tile() {
            Tile::BLACK => write!(f, "black"),
            Tile::WHITE => write!(f, "white"),
            _ => panic!("forbiden team tile"),
        };
        Ok(())
    }
}
