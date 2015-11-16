extern crate std;

use board::{GoBoard, Team, Tile};

pub type HeuristicFn = fn(board: GoBoard, team: Team) -> i32;

/// Returns a numerical value which approximate how close the board is to
/// victory for the team.

struct Alternate<T,
                 S0: Iterator<Item = T>,
                 S1: Iterator<Item = T>,
                 S2: Iterator<Item = T>,
                 S3: Iterator<Item = T>,
                 S4: Iterator<Item = T>,
                 S5: Iterator<Item = T>>
{
    segment_0: S0,
    segment_1: S1,
    segment_2: S2,
    segment_3: S3,
    segment_4: S4,
    segment_5: S5,
    toggle: usize,
}

/// The `alternate` constructor function returns a iterator
/// on multi-list.

fn alternate<II0, II1, II2, II3, II4, II5>(ii0: II0,
                                           ii1: II1,
                                           ii2: II2,
                                           ii3: II3,
                                           ii4: II4,
                                           ii5: II5)
                                           -> Alternate<II0::Item,
                                                        II0::IntoIter,
                                                        II1::IntoIter,
                                                        II2::IntoIter,
                                                        II3::IntoIter,
                                                        II4::IntoIter,
                                                        II5::IntoIter>
    where II0: IntoIterator,
          II1: IntoIterator<Item = II0::Item>,
          II2: IntoIterator<Item = II0::Item>,
          II3: IntoIterator<Item = II0::Item>,
          II4: IntoIterator<Item = II0::Item>,
          II5: IntoIterator<Item = II0::Item>
{
    Alternate {
        segment_0: ii0.into_iter(),
        segment_1: ii1.into_iter(),
        segment_2: ii2.into_iter(),
        segment_3: ii3.into_iter(),
        segment_4: ii4.into_iter(),
        segment_5: ii5.into_iter(),
        toggle: 0usize,
    }
}


impl <T,
    S0: Iterator<Item = T>,
    S1: Iterator<Item = T>,
    S2: Iterator<Item = T>,
    S3: Iterator<Item = T>,
    S4: Iterator<Item = T>,
    S5: Iterator<Item = T>,
> Iterator for Alternate <T, S0, S1, S2, S3, S4, S5> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.toggle = if self.toggle < 5 {
            self.toggle + 1
        } else {
            0
        };
        match self.toggle {
            0 => self.segment_0.next(),
            1 => self.segment_1.next(),
            2 => self.segment_2.next(),
            3 => self.segment_3.next(),
            4 => self.segment_4.next(),
            5 => self.segment_5.next(),
            _ => unreachable!(),
        }
    }
}

fn free_three (
    list: &Vec<Tile>,
) -> isize {
    let (result, pawn, count) = list.iter().fold((0, Tile::FREE, 0), |(result, pawn, count), item| {
            match (*item, pawn, count) {
                (_, _, 4) => return (std::isize::MAX, Tile::FREE, 0),
                (_, _, -4) => return (std::isize::MIN, Tile::FREE, 0),
                (item, Tile::FREE, _) => (result, item, 1),
                (item, pawn, count) if item == pawn => (result, item, count + 1),
                (Tile::BLACK, _, count) => (result + {count * {count+1}}/2, Tile::BLACK, 1),
                (Tile::WHITE, _, count) => (result - {count * {count+1}}/2, Tile::WHITE, 1),
                (_, Tile::WHITE, count) => (result + {count * {count+1}}/2, Tile::FREE, 0),
                (_, Tile::BLACK, count) => (result - {count * {count+1}}/2, Tile::FREE, 0),
            }
        }
    );
    result + match pawn {
        Tile::FREE => 0,
        Tile::WHITE => (count * (count+1)) / 2,
        Tile::BLACK => -(count * (count+1)) / 2,
    }
}

fn captures (
    list: &Vec<Tile>,
) -> isize {
    let (result, _, _) = list.iter().fold((0, Tile::FREE, 0), |(result, pawn, count), item| {
            match (*item, pawn, count) {
                (Tile::FREE, _, _) => (result, Tile::FREE, 0),
                (item, Tile::FREE, _) => (result, item, 0),
                (item, pawn, 0) if item != pawn => (result, item, 1),
                (item, pawn, 0) if item == pawn => (result, Tile::FREE, 0),
                (item, pawn, count) if item == pawn => (result, pawn, count + 1),
                (Tile::BLACK, Tile::WHITE, count) => (result - count, Tile::FREE, 0),
                (Tile::WHITE, Tile::BLACK, count) => (result + count, Tile::FREE, 0),
                (Tile::BLACK, Tile::BLACK, _) => unimplemented!(),
                (Tile::WHITE, Tile::WHITE, _) => unimplemented!(),
            }
        }
    );
    match result {
        result if result > 1 => result,
        _ => 0,
    }
}

#[allow(unused_variables)]
pub fn heuristic(board: GoBoard, team: Team) -> i32 {
    let grid = board.tiles;
    let segment_0 = (0..grid.len()).map(|i| (0..(grid.len())).map(|z| grid[z][i]).collect::<Vec<_>>()); // Ok [7, 5, 3, 1, 0] -> [0, 2, 4, 6, 0] horizontal
    let segment_1 = (0..grid.len()).map(|i| (0..(grid.len())).map(|z| grid[i][z]).collect::<Vec<_>>()); // Ok [7, 5, 3, 1, 0] -> [0, 2, 4, 6, 0] vertical
    let segment_2 = (0..{grid.len()-1}).map(|i| (0..(grid.len() - i)).map(|z| grid[z][z + i]).collect::<Vec<_>>()); // Ok [7, 3, 0, 4, 0] -> [1, 2] diagonal-right middle-to-bottom
    let segment_3 = (0..{grid.len()-1}).map(|i| (0..(grid.len() - i)).map(|z| grid[i + z][z]).collect::<Vec<_>>()); // Ok [7, 3, 0, 4, 0] -> [1, 2] diagonal-right middle-to-top
    let segment_4 = (0..{grid.len()-1}).map(|i| (0..(grid.len() - i)).map(|z| grid[grid.len()-1 - z][z + i]).collect::<Vec<_>>()); // Ok [0, 0, 0, 0, 0] -> [6, 6] diagonal-left middle-to-bottom
    let segment_5 = (0..{grid.len()-1}).map(|i| (0..(grid.len() - i)).map(|z| grid[grid.len()-1 - i - z][z]).collect::<Vec<_>>()); // Ok [0, 0, 0, 0, 0] -> [5, 5] diagonal-left middle-to-top
    let lines: Vec<Vec<Tile>> = alternate (
        segment_0,
        segment_1,
        segment_2,
        segment_3,
        segment_4,
        segment_5
    ).collect();

    lines.iter().fold(0, |acc, item|
        acc + free_three(item) + captures(item) * if team.get_tile() == Tile::WHITE {-1} else {1}
    ) as i32
}
