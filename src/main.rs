use std::io;
use std::collections;
use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

enum ActionType {
    Cast,
    OpponentCast,
    Learn,
    Brew,
}

struct Action {
    id: i32,
    action_type: String,
    tiers_0_ingredient: i32,
    tiers_1_ingredient: i32,
    tiers_2_ingredient: i32,
    tiers_3_ingredient: i32,
    price: i32,
    tome_index: i32,
    tax_count: i32,
    castable: i32,
    repeatable: i32,
}

struct Player {
    tiers_0_inventory: i32,
    tiers_1_inventory: i32,
    tiers_2_inventory: i32,
    tiers_3_inventory: i32,
    score: i32,
}

fn main() {

    // game loop
    loop {
        let mut brewings: Vec<Action> = Vec::new();
        let mut players: HashMap<i32, Player> = HashMap::new();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32); // the number of spells and recipes in play
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // the unique ID of this spell or recipe
            let action_type = inputs[1].trim().to_string(); // in the first league: BREW; later: CAST, OPPONENT_CAST, LEARN, BREW
            let tiers_0_ingredient = parse_input!(inputs[2], i32); // tier-0 ingredient change
            let tiers_1_ingredient = parse_input!(inputs[3], i32); // tier-1 ingredient change
            let tiers_2_ingredient = parse_input!(inputs[4], i32); // tier-2 ingredient change
            let tiers_3_ingredient = parse_input!(inputs[5], i32); // tier-3 ingredient change
            let price = parse_input!(inputs[6], i32); // the price in rupees if this is a potion
            let tome_index = parse_input!(inputs[7], i32); // in the first two leagues: always 0; later: the index in the tome if this is a tome spell, equal to the read-ahead tax
            let tax_count = parse_input!(inputs[8], i32); // in the first two leagues: always 0; later: the amount of taxed tier-0 ingredients you gain from learning this spell
            let castable = parse_input!(inputs[9], i32); // in the first league: always 0; later: 1 if this is a castable player spell
            let repeatable = parse_input!(inputs[10], i32); // for the first two leagues: always 0; later: 1 if this is a repeatable player spell

            match Some(&*action_type) {
                Some("BREW") =>
                    brewings.push(Action {
                        id,
                        action_type,
                        tiers_0_ingredient,
                        tiers_1_ingredient,
                        tiers_2_ingredient,
                        tiers_3_ingredient,
                        price,
                        tome_index,
                        tax_count,
                        castable,
                        repeatable,
                    }),
                _ => eprintln!("oops")
            }
        }
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let tiers_0_inventory = parse_input!(inputs[0], i32); // tier-0 ingredients in inventory
            let tiers_1_inventory = parse_input!(inputs[1], i32);
            let tiers_2_inventory = parse_input!(inputs[2], i32);
            let tiers_3_inventory = parse_input!(inputs[3], i32);
            let score = parse_input!(inputs[4], i32); // amount of rupees
            players.insert(i as i32, Player {
                tiers_0_inventory,
                tiers_1_inventory,
                tiers_2_inventory,
                tiers_3_inventory,
                score,
            });
        }

        let brew = get_best_potion(&brewings);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // in the first league: BREW <id> | WAIT; later: BREW <id> | CAST <id> [<times>] | LEARN <id> | REST | WAIT
        match brew {
            Some(x) => println!("BREW {}", x.id),
            None => println!("WAIT")
        }
    }

    fn get_best_potion(brewing: & Vec<Action>) -> Option<&Action> {
        let best = brewing.iter().max_by_key(|brew| brew.price);
        best
    }
}
