extern crate std;

use board::{GoBoard, Team};
use board::Tile;
use ia;

pub type HeuristicFn = fn(board: &GoBoard, team: Team) -> i32;

const WIN: i32 = ia::INFINITE - 100;

fn check_index(board: &GoBoard, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if x >= board.get_size() as i32 || y >= board.get_size() as i32 {
        return false;
    }
    true
}

fn nb_in_line(board: &GoBoard,
              x: i32, y: i32,
              dx: i32, dy: i32,
              team: Tile)
              -> i32 {
    let mut ttl = 0;
    let mut nx = x + dx;
    let mut ny = y + dy;
    while check_index(board, nx, ny) &&
          board.get((nx as usize, ny as usize)) == team {
        nx += dx;
        ny += dy;
        ttl += 1;
    }
    nx = x - dx;
    ny = y - dy;
    while check_index(board, nx, ny) &&
          board.get((nx as usize, ny as usize)) == team {
        nx -= dx;
        ny -= dy;
        ttl += 1;
    }

    if ttl == 4 {
        WIN
    } else if ttl > 0 {
        ttl
    } else {
        0
    }
}

fn tile_value(board: &GoBoard, x: i32, y: i32, team: Tile) -> i32 {
    let mut ttl_tile = 1;
    let score_tile = nb_in_line(board, x, y, 1, 1, team);
    if score_tile == WIN {
        return WIN;
    }
    ttl_tile += score_tile;
    let score_tile = nb_in_line(board, x, y, 0, 1, team);
    if score_tile == WIN {
        return WIN;
    }
    ttl_tile += score_tile;
    let score_tile = nb_in_line(board, x, y, 1, 0, team);
    if score_tile == WIN {
        return WIN;
    }
    ttl_tile += score_tile;
    let score_tile = nb_in_line(board, x, y, 1, -1, team);
    if score_tile == WIN {
        return WIN;
    }
    ttl_tile += score_tile;
    ttl_tile
}

pub fn heuristic(board: &GoBoard, team: Team) -> i32 {
    let mut player_score = team.captured() as i32;
    let mut enemy_score = 0;
    for x in 0..board.get_size() {
        for y in 0..board.get_size() {
            match board.get((x, y)) {
                Tile::FREE => {},
                t if team.get_tile() == t => {
                    let tile_score = tile_value(board, x as i32, y as i32, team.get_tile());
                    if tile_score == WIN {
                        return WIN;
                    } else {
                        player_score += tile_score;
                    }
                },
                t if team.get_ennemy_tile() == t => {
                    let tile_score = tile_value(board, x as i32, y as i32, team.get_ennemy_tile());
                    if tile_score == WIN {
                        return ia::NINFINITE;
                    } else {
                        enemy_score += tile_score;
                    }
                },
                _ => {}
            }
        }
    }
    player_score - enemy_score
}

/// Heuristic for white is greater than black.
#[cfg(test)]
fn test_one(s: &str) {
    let board = GoBoard::parse_with_size(&s.to_string());
    let (team_b, team_w) = Team::new_teams();
    println!("Test\n{}", board);
    let white = heuristic(&board, team_w);
    let black = heuristic(&board, team_b);
    println!("Is white {} > black {}", white, black);
    assert!(white >black);
}

#[test]
fn test_heuristic() {
    let s = r#"19
W . . . . . . . . . . . . . . . . . .
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
. . . . . . . . . . . . . . . . . . W
        "#;
    test_one(s);

    let s = r#"19
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . W . . . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . W . . . . . . . . .
. . . . . . . . . . . B . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . B . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
        "#;
    test_one(s);

    let s = r#"19
. . . . . . . . . B . . . . . . . . .
. . . . . . . . . B . . . . . . . . .
. . . . . . . . . B . . . . . . . . .
. . . . B . . . . B . . . . . . . . .
. . . . B . . . . . . . . . . . . . .
. . . . B . . . . . . . . . . . . . .
. . . . B . . . . . . B . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . W . . . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . . . B . . . . . . .
. . . . . . . . . . . B . . . . . . .
. . B B B B . . . . . B . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
        "#;
    test_one(s);
}
