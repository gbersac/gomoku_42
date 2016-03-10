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

/// dx and dy must equal 1 or -1
fn nb_in_line(board: &GoBoard,
              x: i32, y: i32,
              dx: i32, dy: i32,
              team: Tile,
              player_turn: bool)
              -> i32 {
    let mut ttl = 1;
    let mut free_extrems = 0;
    let mut blank = 0;

    let mut nx = x + dx;
    let mut ny = y + dy;
    while check_index(board, nx, ny) &&
          board.get((nx as usize, ny as usize)) == team {
        nx += dx;
        ny += dy;
        ttl += 1;
    }
    if check_index(board, nx, ny) &&
            board.get((nx as usize, ny as usize)) != team.ennemy() {
        free_extrems += 1;
        while check_index(board, nx, ny) &&
              board.get((nx as usize, ny as usize)) == Tile::FREE &&
              blank + ttl < 6 {
            nx += dx;
            ny += dy;
            blank += 1;
        }
    }

    nx = x - dx;
    ny = y - dy;
    while check_index(board, nx, ny) &&
          board.get((nx as usize, ny as usize)) == team {
        nx -= dx;
        ny -= dy;
        ttl += 1;
    }
    if check_index(board, nx, ny) &&
            board.get((nx as usize, ny as usize)) != team.ennemy() {
        free_extrems += 1;
        while check_index(board, nx, ny) &&
              board.get((nx as usize, ny as usize)) == Tile::FREE &&
              blank + ttl < 6 {
            nx -= dx;
            ny -= dy;
            blank += 1;
        }
    }

    if blank + ttl < 5 { // if can't expand to victory line, this line is useless
        0
    } else if ttl >= 5 {
        WIN + (ttl - 5) * free_extrems
    } else if free_extrems == 2 && ttl >= 4 {
        WIN - 1
    } else if free_extrems == 2 && ttl >= 3 && player_turn {
        WIN - 2
    } else if ttl > 0 {
        ttl * free_extrems
    } else {
        0
    }
}

fn tile_value(board: &GoBoard,
              x: i32,
              y: i32,
              team: Tile,
              player_turn: bool)
              -> i32 {
    let mut ttl_tile = 0;
    let score_tile = nb_in_line(board, x, y, 1, 1, team, player_turn);
    if score_tile >= WIN - 10 {
        return WIN;
    }
    ttl_tile += score_tile;
    let score_tile = nb_in_line(board, x, y, 0, 1, team, player_turn);
    if score_tile >= WIN - 10 {
        return WIN;
    }
    ttl_tile += score_tile;
    let score_tile = nb_in_line(board, x, y, 1, 0, team, player_turn);
    if score_tile >= WIN - 10 {
        return WIN;
    }
    ttl_tile += score_tile;
    let score_tile = nb_in_line(board, x, y, 1, -1, team, player_turn);
    if score_tile >= WIN - 10 {
        return WIN;
    }
    ttl_tile += score_tile;
    ttl_tile
}

pub fn heuristic(board: &GoBoard, team: Team) -> i32 {
    let mut player_score = team.captured() as i32 * team.captured() as i32;
    let mut enemy_score = 0;
    for x in 0..board.get_size() {
        for y in 0..board.get_size() {
            match board.get((x, y)) {
                Tile::FREE => {},
                t if team.get_tile() == t => {
                    let tile_score = tile_value(board, x as i32, y as i32, team.get_tile(), true);
                    if tile_score >= WIN - 10 {
                        return tile_score;
                    } else {
                        player_score += tile_score;
                    }
                },
                t if team.get_ennemy_tile() == t => {
                    let tile_score = tile_value(board, x as i32, y as i32, team.get_ennemy_tile(), false);
                    if tile_score >= WIN - 10 {
                        return tile_score;
                    }
                    let tile_score = tile_value(board, x as i32, y as i32, team.get_ennemy_tile(), false);
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

    let s = r#"19
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . B . . . . . . . . .
. . . . . . . . . . . W . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . W . B . . . . . . .
. . . . . . . . . B . W . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
        "#;
    test_one(s);
}
