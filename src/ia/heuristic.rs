extern crate std;

use board::{GoBoard, Team};
use board::Tile;
use board::Tile::BLACK;
use board::Tile::WHITE;
use board::Tile::FREE;

pub type HeuristicFn = fn(board: &GoBoard, team: Team) -> i32;

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
) -> i32 {
    let (result, pawn, count) = list.iter().fold((0, FREE, 0), |(result, pawn, count), item| {
            if result == std::i32::MAX || result == !std::i32::MAX {
                (result, FREE, 0)
            }
            else {
                match (*item, pawn, count) {
                    (WHITE, WHITE, 3)  => (std::i32::MAX, FREE, 0),
                    (BLACK, BLACK, 3)  => (!std::i32::MAX, FREE, 0),
                    (item, FREE, _) => (result, item, 1),
                    (item, pawn, count) if item == pawn => (result, item, count + 1),
                    (WHITE, _, count) => (result - {count * {count+1}}/2, WHITE, 1),
                    (BLACK, _, count) => (result + {count * {count+1}}/2, BLACK, 1),
                    (_, WHITE, count) => (result + {count * {count+1}}/2, FREE, 0),
                    (_, BLACK, count) => (result - {count * {count+1}}/2, FREE, 0),
                }
            }
        }
    );
    result + match pawn {
        FREE => 0,
        WHITE => (count * (count+1)) / 2,
        BLACK => (count * (count+1)) / -2,
    }
}

#[test]
fn test_free_three() {
    assert!(0 == free_three(&vec!()));
    assert!(std::i32::MAX == free_three(&vec!(WHITE, WHITE, WHITE, WHITE)));
    assert!(!std::i32::MAX == free_three(&vec!(BLACK, BLACK, BLACK, BLACK)));
    assert!(0 < free_three(&vec!(WHITE, FREE, WHITE, FREE)));
    assert!(0 > free_three(&vec!(BLACK, FREE, BLACK, FREE)));
    assert!(0 < free_three(&vec!(FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, WHITE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE)));
    assert!(0 > free_three(&vec!(FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, BLACK, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE)));
    assert!(0 < free_three(&vec!(FREE, FREE, FREE, FREE, FREE, WHITE, WHITE, WHITE, BLACK, BLACK, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE)));
    assert!(0 < free_three(&vec!(FREE, FREE, FREE, FREE, FREE, WHITE, WHITE, WHITE, BLACK, BLACK, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE, FREE)));
}

#[allow(unused_variables)]
pub fn heuristic(board: &GoBoard, team: Team) -> i32 {
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

    let result = lines.iter().fold(0, |acc, item|
        match free_three(item) {
             score if score == std::i32::MAX || score == !std::i32::MAX => return score,
             score => if let Some(res) = acc.checked_add(score) {
                 res
             } else { return acc },
        }
    );
    match team.get_tile() {
        WHITE => result,
        BLACK => !result,
        FREE => unimplemented!(),
    }
}


#[test]
fn test_win_free_three() {
    assert! (
         std::i32::MAX == heuristic (
            &GoBoard::parse_with_size (&r#"19
            W W W W . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(WHITE)
        )
    );
    assert! (
         !std::i32::MAX == heuristic (
            &GoBoard::parse_with_size (&r#"19
            W W W W . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(BLACK)
        )
    );
    assert! (
         !std::i32::MAX == heuristic (
            &GoBoard::parse_with_size (&r#"19
            B B B B . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(WHITE)
        )
    );
    assert! (
         std::i32::MAX == heuristic (
            &GoBoard::parse_with_size (&r#"19
            B B B B . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(BLACK)
        )
    );
    assert! (
         !std::i32::MAX != heuristic (
            &GoBoard::parse_with_size (&r#"19
            W W W . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(BLACK)
        )
    );
    assert! (
         std::i32::MAX != heuristic (
            &GoBoard::parse_with_size (&r#"19
            B B B . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(BLACK)
        )
    );
}

#[test]
fn test_null() {
    assert! (
         0 == heuristic (
            &GoBoard::parse_with_size (&r#"19
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(WHITE)
        )
    );
    assert! (
         0 == heuristic (
            &GoBoard::parse_with_size (&r#"19
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . W . . . . . . . . . . . . . . . B .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(WHITE)
        )
    );
    assert! (
         0 == heuristic (
            &GoBoard::parse_with_size (&r#"19
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . W W W . . . . . . . . . . . B B B .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . . . . . .
            "#.to_string()),
            Team::new(WHITE)
        )
    );
}
