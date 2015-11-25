extern crate piston;

use std::fmt::{Formatter, Display, Error};

#[derive(Debug, Clone)]
pub struct Mouse {
    overed: bool,
    coordinate_cell: (u32, u32),
    dimension: (u32, u32),
}

impl Mouse {
    pub fn new (
        sizes: (u32, u32),
    ) -> Self {
        let mut event: Self = Default::default();

        event.set_dimension(sizes);
        event
    }

    pub fn set_dimension (
        &mut self,
        dimension: (u32, u32),
    ) {
        self.dimension = dimension;
    }

    pub fn get_dimension (
        &self,
    ) -> (u32, u32) {
        self.dimension
    }

    pub fn set_coordinate (
        &mut self,
        coordinate: (u32, u32),
    ) {
        self.coordinate_cell = coordinate;
    }

    pub fn get_coordinate (
        &self,
    ) -> (u32, u32) {
        self.coordinate_cell
    }

    fn set_over (
        &mut self,
        mouse: bool,
    ) {
        self.overed = mouse;
    }

    pub fn get_over (
        &mut self,
    ) -> bool {
        self.overed
    }

    pub fn check_inside_window (
        &mut self,
        coordinate: (u32, u32),
        length: u32,
    ) -> Option<(u32, u32)> {
        let mouse:bool = 0u32 < coordinate.0
                      && coordinate.0 <  self.dimension.0
                      && 0u32 < coordinate.1
                      && coordinate.1 < self.dimension.1;

        if mouse {
            self.set_over(true);
            let coordinate_cell_new = (
                coordinate.0 / {self.dimension.0 / length},
                coordinate.1 / {self.dimension.1 / length}
            );
            if coordinate_cell_new.0 != self.coordinate_cell.0
            || coordinate_cell_new.1 != self.coordinate_cell.1 {
                return Some(coordinate_cell_new);
            }
        }
        else {
            self.set_over(false);
        }
        None
    }
}

impl Display for Mouse {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let _ = write!(f, stringify!(self.coordinate_cell.width));
		let _ = write!(f, stringify!(self.coordinate_cell.height));
		Ok(())
	}
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
          overed: false,
          coordinate_cell: (0u32, 0u32),
          dimension: (0u32, 0u32),
        }
    }
}
