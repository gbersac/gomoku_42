extern crate piston;

use self::piston::window::Size;
use std::fmt::{Formatter, Display, Error};

pub struct _Event {
    overed: bool,
    coordinate_cell: Size,
    dimension: Size,
}

impl _Event {
    pub fn new (
        sizes: Size,
    ) -> Self {
        let mut event: Self = Default::default();

        event.set_dimension(sizes);
        event
    }

    pub fn set_dimension (
        &mut self,
        dimension: Size,
    ) {
        self.dimension = dimension;
    }

    pub fn get_dimension (
        &self,
    ) -> Size {
        self.dimension
    }

    pub fn set_coordinate (
        &mut self,
        coordinate: Size,
    ) {
        self.coordinate_cell = coordinate;
    }

    pub fn get_coordinate (
        &self,
    ) -> (usize, usize) {
        let coordinate = self.coordinate_cell;

        (coordinate.width as usize, coordinate.height as usize)
    }

    fn set_over (
        &mut self,
        mouse: bool,
    ) {
        self.overed = mouse;
    }

    pub fn check_inside_window (
        &mut self,
        coordinate: Size,
        length: u32,
    ) -> Option<Size> {
        let mouse:bool = 0u32 < coordinate.width
                      && coordinate.width <  self.dimension.width
                      && 0u32 < coordinate.height
                      && coordinate.height < self.dimension.height;

        if mouse {
            self.set_over(true);
            let coordinate_cell_new: Size = Size::from([
                coordinate.width / {self.dimension.width / length},
                coordinate.height / {self.dimension.height / length}
            ]);
            if coordinate_cell_new.width != self.coordinate_cell.width
            || coordinate_cell_new.height != self.coordinate_cell.height {
                return Some(coordinate_cell_new);
            }
        }
        else {
            self.set_over(false);
        }
        None
    }
}

impl Display for _Event {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let _ = write!(f, stringify!(self.coordinate_cell.width));
		let _ = write!(f, stringify!(self.coordinate_cell.height));
		Ok(())
	}
}

impl Default for _Event {
    fn default() -> Self {
        _Event {
          overed: false,
          coordinate_cell: Size::from([0u32; 2]),
          dimension: Size::from([0; 2]),
        }
    }
}
