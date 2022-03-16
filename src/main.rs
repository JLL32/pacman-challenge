use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Grab the pellets as fast as you can!
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32); // size of the grid
    let height = parse_input!(inputs[1], i32); // top left corner is (x=0, y=0)
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string(); // one line of the grid: space " " is floor, pound "#" is wall
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_score = parse_input!(inputs[0], i32);
        let opponent_score = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pac_count = parse_input!(input_line, i32); // all your pacs and enemy pacs in sight
        let mut mine_pos = (0, 0);
        for i in 0..visible_pac_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let pac_id = parse_input!(inputs[0], i32); // pac number (unique within a team)
            let mine = parse_input!(inputs[1], i32); // true if this pac is yours
            let x = parse_input!(inputs[2], i32); // position in the grid
            let y = parse_input!(inputs[3], i32); // position in the grid
            if mine != 0 {
                mine_pos.0 = x;
                mine_pos.1 = y;
            }
            let type_id = inputs[4].trim().to_string(); // unused in wood leagues
            let speed_turns_left = parse_input!(inputs[5], i32); // unused in wood leagues
            let ability_cooldown = parse_input!(inputs[6], i32); // unused in wood leagues
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_pellet_count = parse_input!(input_line, i32); // all pellets in sight
        let (mut max_x, mut max_y, mut max_value) = (0, 0, 1);
        let mut shortest_distance = f32::MAX;
        for i in 0..visible_pellet_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            let value = parse_input!(inputs[2], i32); // amount of points this pellet is worth
            let distance =
                f32::sqrt(i32::pow(mine_pos.0 - x, 2) as f32 + i32::pow(mine_pos.1 - y, 2) as f32);
            if value > max_value || distance < shortest_distance {
                max_x = x;
                max_y = y;
                max_value = value;
            }
        }
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        println!("MOVE 0 {} {}", max_x, max_y); // MOVE <pacId> <x> <y>
    }
}

