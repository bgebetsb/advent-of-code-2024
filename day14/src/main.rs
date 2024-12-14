use std::fs::read_to_string;
use std::io;
use utils::*;

#[derive(Debug, Clone)]
struct MapLine {
    position: (i128, i128),
    velocities: (i128, i128),
}

fn print_map(map: &[MapLine], width: i128, height: i128) {
    let mut grid = vec![vec![0; width as usize]; height as usize];

    for robot in map {
        grid[robot.position.1 as usize][robot.position.0 as usize] += 1;
    }

    for line in grid {
        for value in line {
            if value == 0 {
                print!(".");
            } else {
                print!("{value}");
            }
        }
        println!();
    }
}

fn find_diagonal(map: &[MapLine], x: i128, y: i128, count: i128, stop: i128) -> bool {
    let mut left_found = false;
    let mut right_found = false;

    for robot in map {
        if robot.position.1 == y + count {
            if robot.position.0 == x - count {
                left_found = true;
            } else if robot.position.0 == x + count {
                right_found = true;
            }
        }
    }

    if !left_found || !right_found {
        return false;
    }

    if count != stop {
        return find_diagonal(map, x, y, count + 1, stop);
    }
    true
}

fn find_tree(map: &[MapLine]) -> bool {
    for robot in map {
        if find_diagonal(map, robot.position.0, robot.position.1, 1, 5) {
            return true;
        }
    }
    false
}

fn parse_map(input: String) -> Vec<MapLine> {
    input
        .get_lines()
        .iter()
        .map(|line| {
            let splitted = line
                .split_whitespace()
                .flat_map(|part| {
                    part.split(',')
                        .map(|number| {
                            number
                                .chars()
                                .filter(|c| c.is_ascii_digit() || *c == '-')
                                .collect::<String>()
                                .parse::<i128>()
                                .unwrap()
                        })
                        .collect::<Vec<i128>>()
                })
                .collect::<Vec<i128>>();
            MapLine {
                position: (splitted[0], splitted[1]),
                velocities: (splitted[2], splitted[3]),
            }
        })
        .collect::<Vec<MapLine>>()
}

fn robo_move(robot: &mut MapLine, width: i128, height: i128) {
    robot.position.0 += robot.velocities.0;
    if robot.position.0 < 0 {
        robot.position.0 += width;
    } else if robot.position.0 >= width {
        robot.position.0 -= width;
    }

    robot.position.1 += robot.velocities.1;
    if robot.position.1 < 0 {
        robot.position.1 += height;
    } else if robot.position.1 >= height {
        robot.position.1 -= height;
    }
}

fn part1(map: &mut [MapLine], width: i128, height: i128) {
    for _ in 0..100 {
        for robot in &mut *map {
            robo_move(robot, width, height);
        }
    }

    let mut topleft = 0;
    let mut topright = 0;
    let mut bottomleft = 0;
    let mut bottomright = 0;

    for robot in map {
        if robot.position.0 < width / 2 && robot.position.1 < height / 2 {
            topleft += 1;
        } else if robot.position.0 < width / 2 && robot.position.1 > height / 2 {
            bottomleft += 1;
        } else if robot.position.0 > width / 2 && robot.position.1 < height / 2 {
            topright += 1;
        } else if robot.position.0 > width / 2 && robot.position.1 > height / 2 {
            bottomright += 1;
        }
    }

    println!("Part 1: {}", topleft * topright * bottomleft * bottomright);
}

fn part2(map: &mut [MapLine], width: i128, height: i128) {
    for i in 0.. {
        for robot in &mut *map {
            robo_move(robot, width, height);
        }
        if find_tree(map) {
            println!("Part 2: {}", i + 1);
            print_map(map, width, height);
            return;
        }
    }
}

fn main() -> Result<(), io::Error> {
    let map = parse_map(read_to_string("input.txt")?);

    let width = 101;
    let height = 103;

    part1(&mut map.clone(), width, height);
    part2(&mut map.clone(), width, height);

    Ok(())
}