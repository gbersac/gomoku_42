use board::GoBoard;
use board::Tile;

pub struct Decision {
    layer: usize,
}

impl Decision {
    pub fn new (layer: usize) -> Self {
        Decision {
            layer: layer,
        }
    }

    pub fn choosen (
        board: GoBoard,
        tile: Tile,
    ) -> (i32, i32) {
        (0, 0)
    }
}

impl Default for Decision {

	/// The `new` constructor function returns the empty Decision.

    fn default() -> Self {
		Decision {
			layer: 0,
		}
    }
}
