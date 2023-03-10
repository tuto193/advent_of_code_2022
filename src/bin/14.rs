type Level = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Block,
    Sand,
    Air,
}
enum Fall {
    Down,
    Left,
    Right,
    Stop,
    Abyss,
}

fn init_level(height: u32, width: u32) -> Level {
    let mut to_ret = vec![];
    for _ in 0..height {
        let mut to_append = vec![];
        for _ in 0..width {
            to_append.push(Tile::Air);
        }
        to_ret.push(to_append);
    }
    to_ret
}

fn draw_wall(level: &mut Level, lines: String) -> usize {
    let mut lowest_height: usize = 0;
    let coords_list: Vec<(usize, usize)> = lines
        .split(" -> ")
        .into_iter()
        .map(|l| {
            let v: Vec<usize> = l
                .split(",")
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect();
            lowest_height = lowest_height.max(v[1]);
            (v[0], v[1])
        })
        .collect();
    for (i, coord) in coords_list.clone().into_iter().skip(1).enumerate() {
        let (x1, y1) = coord;
        let (x0, y0) = coords_list[i];
        for y in y0.min(y1)..y1.max(y0) + 1 {
            for x in x0.min(x1)..x1.max(x0) + 1 {
                level[y][x] = Tile::Block;
            }
        }
    }
    lowest_height
}

fn parse_level(input: &str, with_bottom_line: bool) -> Level {
    let mut level = init_level(1000, 1000);
    let input: Vec<&str> = input.split("\n").collect();
    let wanted_input = input.len() - 1;
    let mut lowest_height: usize = 0;
    for wall_lines in input.into_iter().take(wanted_input) {
        lowest_height = lowest_height.max(draw_wall(&mut level, wall_lines.into()));
    }
    lowest_height += 2;
    if with_bottom_line {
        // Draw the bottom line found
        draw_wall(
            &mut level,
            format!("0,{} -> 999,{}", lowest_height, lowest_height),
        );
    }
    level
}

fn where_to_fall(grain: (usize, usize), level: &Level) -> Fall {
    let lower_bownd = level.len();
    let righmost_bound = level[0].len();
    if grain.1 < lower_bownd - 2 {
        // if let Some(next_row) = level.get(grain.1 + 1) {
        let next_row = level[grain.1 + 1].clone();
        let down = next_row[grain.0];
        let down_l = next_row[grain.0 - 1];
        let down_r = next_row[grain.0 + 1];
        match down {
            Tile::Air => {
                return Fall::Down;
            }
            d => {
                // There is something down, so check to the left and then down
                let same_row = level[grain.1].clone();
                let left = same_row[grain.0 - 1];
                let right = same_row[grain.0 + 1];
                if grain.0 > 0 || grain.0 < righmost_bound - 2 {
                    // if let Some(&left) = same_row.get(grain.0 - 1) {
                    // if !same_row[grain.0 - 1] && !next_row[grain.0 - 1] {
                    if down_l == Tile::Air {
                        match d {
                            Tile::Sand => {
                                return Fall::Left;
                            }
                            _block => {
                                if left == Tile::Air {
                                    return Fall::Left;
                                } else {
                                    return Fall::Stop;
                                }
                            }
                        }
                    } else if down_r == Tile::Air {
                        match d {
                            Tile::Sand => {
                                return Fall::Right;
                            }
                            _block => {
                                if right == Tile::Air {
                                    return Fall::Right;
                                } else {
                                    return Fall::Stop;
                                }
                            }
                        }
                    } else {
                        return Fall::Stop;
                    }
                } else {
                    panic!("Got stock at the edge of the world. Cannot fall");
                }
            }
        }
    } else {
        // Cannot go farther than the abyss
        return Fall::Abyss;
    }
}

#[allow(dead_code)] // Since we only use this to debug
fn printl_80x40_range(level: &Level, center_point: (usize, usize)) {
    let (x, y) = center_point;
    let x = 40.max(x.min(level[0].len() - 41));
    let y = 20.max(y.min(level.len() - 21));
    for row in level.into_iter().skip(y - 20).take(20) {
        for &block in row.into_iter().skip(x - 40).take(80) {
            match block {
                Tile::Block => print!("#"),
                Tile::Sand => print!("o"),
                Tile::Air => print!("??"),
            }
        }
        println!();
    }
    println!();
}

fn drop_sand_grain(level: &mut Level) -> bool {
    if level[0][500] == Tile::Sand {
        // The origin is full
        return false;
    }
    let mut grain: (usize, usize) = (500, 0);
    // let mut falling = true;
    loop {
        // if falling {

        // }
        match where_to_fall(grain, level) {
            Fall::Down => grain.1 += 1,
            Fall::Left => {
                grain.0 -= 1;
                grain.1 += 1;
            }
            Fall::Right => {
                grain.0 += 1;
                grain.1 += 1;
            }
            Fall::Stop => {
                // printl_80x40_range(level, (500, 0));
                level[grain.1][grain.0] = Tile::Sand;
                return true;
            }
            Fall::Abyss => {
                // printl_80x40_range(level, (500, 0));
                return false;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut level = parse_level(input, false);
    let mut sand_corn_index = 0;
    while drop_sand_grain(&mut level) {
        // println!("Sand: {}", sand_corn_index);
        // printl_80x40_range(&level, (500, 0));
        sand_corn_index += 1;
    }
    // printl_80x40_range(&level, (500, 0));
    Some(sand_corn_index)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut level = parse_level(input, true);
    let mut sand_corn_index = 0;
    while drop_sand_grain(&mut level) {
        println!("Sand: {}", sand_corn_index);
        printl_80x40_range(&level, (500, 0));
        sand_corn_index += 1;
    }
    printl_80x40_range(&level, (500, 0));
    Some(sand_corn_index)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
