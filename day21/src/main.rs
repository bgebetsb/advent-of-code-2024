use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::io;
use utils::*;

fn find_shortest_paths2(
    keypad: &Vec<[char; 3]>,
    y: usize,
    x: usize,
    character: char,
) -> Vec<(usize, usize, String)> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut results = Vec::new();

    queue.push_back((y, x, 0, String::new()));
    visited.insert((y, x), 0);

    while let Some((y, x, steps, path)) = queue.pop_front() {
        if keypad[y][x] == character {
            results.push((y, x, format!("{}A", path)));
            continue;
        }

        for &(cur_y, cur_x) in &directions {
            if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                if new_y < keypad.len() && new_x < keypad[new_y].len() {
                    let steps = steps + 1;
                    let path = match (cur_y, cur_x) {
                        (-1, 0) => format!("{}^", path),
                        (1, 0) => format!("{}v", path),
                        (0, -1) => format!("{}<", path),
                        (0, 1) => format!("{}>", path),
                        _ => panic!("Invalid direction"),
                    };

                    if keypad[new_y][new_x] != '\0'
                        && (!visited.contains_key(&(new_y, new_x))
                            || visited[&(new_y, new_x)] == steps)
                    {
                        visited.insert((new_y, new_x), steps);
                        queue.push_back((new_y, new_x, steps, path));
                    }
                }
            }
        }
    }

    results
}

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['\0', '0', 'A'],
];

const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [['\0', '^', 'A'], ['<', 'v', '>']];

struct Robot {
    pos_y: usize,
    pos_x: usize,
    keypad: Vec<[char; 3]>,
    next: Option<Box<Robot>>,
}

impl Robot {
    fn new(numeric: bool, next: Option<Box<Robot>>) -> Self {
        let keypad = {
            if numeric {
                NUMERIC_KEYPAD.to_vec()
            } else {
                DIRECTIONAL_KEYPAD.to_vec()
            }
        };

        let (pos_y, pos_x) = if numeric { (3, 2) } else { (0, 2) };

        Self {
            pos_y,
            pos_x,
            keypad,
            next,
        }
    }

    fn calculate_steps(&mut self, input: &str, update_position: bool) -> String {
        let mut output = String::new();

        let (old_y, old_x) = (self.pos_y, self.pos_x);

        for c in input.chars() {
            let result = find_shortest_paths2(&self.keypad, self.pos_y, self.pos_x, c);
            let mut lowest = result[0].2.clone();
            if let Some(ref mut next) = self.next {
                let mut lowest_length = next.calculate_steps(&lowest, false).len();
                for item in result.iter().skip(1) {
                    let result = next.calculate_steps(&item.2, false);
                    if result.len() < lowest_length {
                        lowest = item.2.clone();
                        lowest_length = result.len();
                    }
                }
            }
            output.push_str(&lowest);
            self.update_position(c);
        }

        if let Some(ref mut next) = self.next {
            return next.calculate_steps(&output, true);
        }

        if update_position {
            (self.pos_y, self.pos_x) = (old_y, old_x);
        }
        output
    }

    fn update_position(&mut self, target_char: char) {
        for (y, row) in self.keypad.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == target_char {
                    self.pos_y = y;
                    self.pos_x = x;
                    return;
                }
            }
        }
    }
}

fn find_shortest_paths(
    keypad: &[Vec<char>],
    y: usize,
    x: usize,
    character: char,
) -> Vec<(usize, usize, String)> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut results = Vec::new();

    queue.push_back((y, x, 0, String::new()));
    visited.insert((y, x), 0);

    while let Some((y, x, steps, path)) = queue.pop_front() {
        if keypad[y][x] == character {
            results.push((y, x, format!("{}A", path)));
            continue;
        }

        for &(cur_y, cur_x) in &directions {
            if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                if new_y < keypad.len() && new_x < keypad[new_y].len() {
                    let steps = steps + 1;
                    let path = match (cur_y, cur_x) {
                        (-1, 0) => format!("{}^", path),
                        (1, 0) => format!("{}v", path),
                        (0, -1) => format!("{}<", path),
                        (0, 1) => format!("{}>", path),
                        _ => panic!("Invalid direction"),
                    };

                    if keypad[new_y][new_x] != '\0'
                        && (!visited.contains_key(&(new_y, new_x))
                            || visited[&(new_y, new_x)] == steps)
                    {
                        visited.insert((new_y, new_x), steps);
                        queue.push_back((new_y, new_x, steps, path));
                    }
                }
            }
        }
    }

    results
}

/*
fn find_shortest_recursive(
    keypad: &[Vec<char>],
    y: usize,
    x: usize,
    input: &str,
    current: &str,
    depth: usize,
) -> String {
    if depth == input.len() {
        // Base case: All characters in the input have been processed
        return current.to_string();
    }

    let target_char = input.chars().nth(depth).unwrap();
    let mut shortest: Option<String> = None;

    for path in find_shortest_paths(keypad, y, x, target_char) {
        let (end_y, end_x) = apply_path(y, x, &path);
        let candidate = find_shortest_recursive(
            keypad,
            end_y,
            end_x,
            input,
            &format!("{}{}", current, path),
            depth + 1,
        );

        if shortest.is_none() || candidate.len() < shortest.as_ref().unwrap().len() {
            shortest = Some(candidate);
        }
    }

    shortest.unwrap() // Unwrap the shortest path, which is guaranteed to exist
}
*/
fn find_shortest(
    keypad: &[Vec<char>],
    directional: &[Vec<char>],
    y: usize,
    x: usize,
    input: &str,
    depth: usize,
) -> (usize, usize, String) {
    if depth == 3 {
        return (y, x, input.to_string());
    }

    let mut total = String::new();
    let (mut y, mut x) = (y, x);
    for c in input.chars() {
        let path = {
            if depth == 0 {
                find_shortest_paths(keypad, y, x, c)
                    .iter()
                    .map(|(y, x, path)| find_shortest(keypad, directional, *y, *x, path, depth + 1))
                    .min_by_key(|(_, _, output)| output.len())
            } else if depth == 1 {
                find_shortest_paths(directional, 0, 2, c)
                    .iter()
                    .map(|(y, x, path)| find_shortest(keypad, directional, *y, *x, path, depth + 1))
                    .min_by_key(|(_, _, output)| output.len())
            } else {
                find_shortest_paths(directional, y, x, c)
                    .iter()
                    .map(|(y, x, path)| find_shortest(keypad, directional, *y, *x, path, depth + 1))
                    .min_by_key(|(_, _, output)| output.len())
            }
        };
        let path = path.unwrap();
        (y, x) = (path.0, path.1);
        total.push_str(path.2.as_str());
    }
    (y, x, total)
}

/*
fn find_shortest(keypad: &[Vec<char>], y: usize, x: usize, input: &str) -> String {
    let shortest = String::new();
    for char in input.chars() {
        for (y, x, path) in find_shortest_paths(keypad, y, x, char) {
            for char in path.chars() {
                for (y, x, path) in find_shortest_paths(keypad, y, x, char) {
                    for char in path.chars() {
                        for (y, x, path) in find_shortest_paths(keypad, y, x, char) {
                            return path;
                        }
                    }
                }
            }
        }
    }
    String::new()
}
*/

fn type_on_keypad(
    keypad: &[Vec<char>],
    character: char,
    nextchar: Option<char>,
    y: usize,
    x: usize,
    directions: &Vec<(isize, isize)>,
) -> Vec<(usize, usize, Vec<char>)> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut totalresults = Vec::new();

    queue.push_back((y, x, 0));
    visited.insert((y, x), vec![]);
    while let Some((y, x, steps)) = queue.pop_front() {
        if keypad[y][x] == character {
            let mut current = visited.get(&(y, x)).unwrap().to_vec();
            current.push('A');
            totalresults.push((y, x, current));
        }
        for &(cur_y, cur_x) in &directions {
            let mut current = visited.get(&(y, x)).unwrap().to_vec();
            let c = match (cur_y, cur_x) {
                (-1, 0) => '^',
                (1, 0) => 'v',
                (0, -1) => '<',
                (0, 1) => '>',
                _ => panic!("Invalid direction"),
            };
            if let (Ok(new_y), Ok(new_x)) = (y.offset(cur_y), x.offset(cur_x)) {
                if new_y >= keypad.len() || new_x >= keypad[new_y].len() {
                    continue;
                }
                current.push(c);
                if !visited.contains_key(&(new_y, new_x)) {
                    visited.insert((new_y, new_x), current);
                    queue.push_back((new_y, new_x, steps + 1));
                }
            }
        }
    }
    totalresults
}

fn steer_robot(keypad: &[Vec<char>], input: &[char], y: usize, x: usize) -> Vec<char> {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut y = y;
    let mut x = x;
    let mut total_directons = Vec::new();
    for (i, c) in input.iter().enumerate() {
        let mut nextchar = None;
        if i != input.len() - 1 {
            nextchar = Some(input[i + 1]);
        }
        let (new_y, new_x, directions) =
            &type_on_keypad(&keypad, *c, nextchar, y, x, &directions)[0];
        for direction in directions {
            total_directons.push(*direction);
        }
        y = *new_y;
        x = *new_x;
    }
    total_directons
}

fn main() -> Result<(), io::Error> {
    let mut input = read_to_string("example.txt")?.get_lines().lines_as_chars();
    let keypad = [
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['Z', '0', 'A'],
    ];

    let directional = [vec!['Z', '^', 'A'], vec!['<', 'v', '>']];

    let keycodes = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    // let keycodes = ["029A", "980A", "179A", "456A", "379A"];
    for keycode in keycodes {
        let (y, x) = (3, 2);
        let keycode = keycode.chars().collect::<Vec<char>>();
        let mut output = steer_robot(&keypad, &keycode, y, x);
        for i in 0..2 {
            output = steer_robot(&directional, &output, 0, 2);
        }
        let length = output.len();
        println!("{}", length);
        println!("{}", output.into_iter().collect::<String>());
    }

    let search = "980A";
    println!(
        "{:?}",
        find_shortest(&keypad, &directional, 3, 2, &search, 0)
            .2
            .len()
    );

    let mut robo3 = Robot::new(false, None);
    let mut robo2 = Robot::new(false, Some(Box::new(robo3)));
    let mut robo = Robot::new(true, Some(Box::new(robo2)));

    println!("EXAMPLES");
    println!("{}", robo.calculate_steps("029A", true).len());
    println!("{}", robo.calculate_steps("980A", true).len());
    println!("{}", robo.calculate_steps("179A", true).len());
    println!("{}", robo.calculate_steps("456A", true).len());
    println!("{}", robo.calculate_steps("379A", true).len());

    println!("REAL");

    println!("{}", robo.calculate_steps("540A", true).len());
    println!("{}", robo.calculate_steps("582A", true).len());
    println!("{}", robo.calculate_steps("169A", true).len());
    println!("{}", robo.calculate_steps("593A", true).len());
    println!("{}", robo.calculate_steps("579A", true).len());

    /*
        let mut robo26 = Robot::new(false, None);
        let mut robo25 = Robot::new(false, Some(Box::new(robo26)));
        let mut robo24 = Robot::new(false, Some(Box::new(robo25)));
        let mut robo23 = Robot::new(false, Some(Box::new(robo24)));
        let mut robo22 = Robot::new(false, Some(Box::new(robo23)));
        let mut robo21 = Robot::new(false, Some(Box::new(robo22)));
        let mut robo20 = Robot::new(false, Some(Box::new(robo21)));
        let mut robo19 = Robot::new(false, Some(Box::new(robo20)));
        let mut robo18 = Robot::new(false, Some(Box::new(robo19)));
        let mut robo17 = Robot::new(false, Some(Box::new(robo18)));
        let mut robo16 = Robot::new(false, Some(Box::new(robo17)));
        let mut robo15 = Robot::new(false, Some(Box::new(robo16)));
        let mut robo14 = Robot::new(false, Some(Box::new(robo15)));
        let mut robo13 = Robot::new(false, Some(Box::new(robo14)));
        let mut robo12 = Robot::new(false, Some(Box::new(robo13)));
        let mut robo11 = Robot::new(false, Some(Box::new(robo12)));
        let mut robo10 = Robot::new(false, Some(Box::new(robo11)));
        let mut robo9 = Robot::new(false, Some(Box::new(robo10)));
        let mut robo8 = Robot::new(false, Some(Box::new(robo9)));
        let mut robo7 = Robot::new(false, Some(Box::new(robo8)));
        let mut robo6 = Robot::new(false, Some(Box::new(robo7)));
        let mut robo5 = Robot::new(false, Some(Box::new(robo6)));
        let mut robo4 = Robot::new(false, Some(Box::new(robo5)));
        let mut robo3 = Robot::new(false, Some(Box::new(robo4)));
        let mut robo2 = Robot::new(false, Some(Box::new(robo3)));
        let mut robo = Robot::new(true, Some(Box::new(robo2)));

        println!("{}", robo.calculate_steps("540A", true).len() * 540);
        println!("{}", robo.calculate_steps("582A", true).len() * 582);
        println!("{}", robo.calculate_steps("169A", true).len() * 169);
        println!("{}", robo.calculate_steps("593A", true).len() * 593);
        println!("{}", robo.calculate_steps("579A", true).len() * 579);
    */

    // println!();
    Ok(())
}
