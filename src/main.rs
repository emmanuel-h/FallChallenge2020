use std::collections;
use std::collections::HashMap;
use std::io;
use std::borrow::Borrow;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

macro_rules! no_negative {
    ($x:expr) => {
    if $x < 0 { 0 } else { $x }
    }
}

struct Potion {
    id: i32,
    tiers_0_ingredient: i32,
    tiers_1_ingredient: i32,
    tiers_2_ingredient: i32,
    tiers_3_ingredient: i32,
    price: i32,
}

impl Potion {
    fn brewable(&self, player: &Player) -> bool {
        self.tiers_0_ingredient.abs() < player.tiers_0_inventory
            &&
            self.tiers_1_ingredient.abs() < player.tiers_1_inventory
            &&
            self.tiers_2_ingredient.abs() < player.tiers_2_inventory
            &&
            self.tiers_3_ingredient.abs() < player.tiers_3_inventory
    }
}

struct Spell {
    id: i32,
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

impl Spell {
    fn enough_ingredient(&self, player: &Player) -> bool {
        (self.tiers_0_ingredient <= player.tiers_0_inventory || self.tiers_0_ingredient > 0)
            && (self.tiers_1_ingredient <= player.tiers_1_inventory || self.tiers_1_ingredient > 0)
            && (self.tiers_2_ingredient <= player.tiers_2_inventory || self.tiers_2_ingredient > 0)
            && (self.tiers_3_ingredient <= player.tiers_3_inventory || self.tiers_3_ingredient > 0)
    }

    fn add_useful_ingredient(&self, ingredients: [i32; 4]) -> i32 {
        let mut ingredients_added = 0;
        if ingredients[0] > 0 && self.tiers_0_ingredient > 0 { ingredients_added += self.tiers_0_ingredient }
        if ingredients[1] > 0 && self.tiers_1_ingredient > 0 { ingredients_added += self.tiers_1_ingredient }
        if ingredients[2] > 0 && self.tiers_2_ingredient > 0 { ingredients_added += self.tiers_2_ingredient }
        if ingredients[3] > 0 && self.tiers_3_ingredient > 0 { ingredients_added += self.tiers_3_ingredient }
        ingredients_added
    }
}

#[derive(Debug)]
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
        let mut potions: Vec<Potion> = Vec::new();
        let mut spells: Vec<Spell> = Vec::new();
        let mut player: Player;
        let mut opponent: Player;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32); // the number of spells and recipes in play
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let id = parse_input!(inputs[0], i32); // the unique ID of this spell or recipe
            let action_type = inputs[1].trim(); // in the first league: BREW; later: CAST, OPPONENT_CAST, LEARN, BREW
            let tiers_0_ingredient = parse_input!(inputs[2], i32); // tier-0 ingredient change
            let tiers_1_ingredient = parse_input!(inputs[3], i32); // tier-1 ingredient change
            let tiers_2_ingredient = parse_input!(inputs[4], i32); // tier-2 ingredient change
            let tiers_3_ingredient = parse_input!(inputs[5], i32); // tier-3 ingredient change
            let price = parse_input!(inputs[6], i32); // the price in rupees if this is a potion
            let tome_index = parse_input!(inputs[7], i32); // in the first two leagues: always 0; later: the index in the tome if this is a tome spell, equal to the read-ahead tax
            let tax_count = parse_input!(inputs[8], i32); // in the first two leagues: always 0; later: the amount of taxed tier-0 ingredients you gain from learning this spell
            let castable = parse_input!(inputs[9], i32); // in the first league: always 0; later: 1 if this is a castable player spell
            let repeatable = parse_input!(inputs[10], i32); // for the first two leagues: always 0; later: 1 if this is a repeatable player spell

            match action_type {
                "BREW" =>
                    potions.push(Potion {
                        id,
                        tiers_0_ingredient,
                        tiers_1_ingredient,
                        tiers_2_ingredient,
                        tiers_3_ingredient,
                        price,
                    }),
                "CAST" =>
                    spells.push(Spell {
                        id,
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
                _ => eprintln!("{}", action_type)
            }
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let tiers_0_inventory = parse_input!(inputs[0], i32); // tier-0 ingredients in inventory
        let tiers_1_inventory = parse_input!(inputs[1], i32);
        let tiers_2_inventory = parse_input!(inputs[2], i32);
        let tiers_3_inventory = parse_input!(inputs[3], i32);
        let score = parse_input!(inputs[4], i32); // amount of rupees
        player = Player {
            tiers_0_inventory,
            tiers_1_inventory,
            tiers_2_inventory,
            tiers_3_inventory,
            score,
        };
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let tiers_0_inventory = parse_input!(inputs[0], i32); // tier-0 ingredients in inventory
        let tiers_1_inventory = parse_input!(inputs[1], i32);
        let tiers_2_inventory = parse_input!(inputs[2], i32);
        let tiers_3_inventory = parse_input!(inputs[3], i32);
        let score = parse_input!(inputs[4], i32); // amount of rupees
        opponent = Player {
            tiers_0_inventory,
            tiers_1_inventory,
            tiers_2_inventory,
            tiers_3_inventory,
            score,
        };

        let action: String;
        let potion = get_best_potion(&potions).unwrap();
        if potion.brewable(&player) {
            action = format!("BREW {}", potion.id)
        } else {
            eprintln!("{}", potion.id);
            let mut missing_ingredients = [
                potion.tiers_0_ingredient - player.tiers_0_inventory,
                potion.tiers_1_ingredient - player.tiers_1_inventory,
                potion.tiers_2_ingredient - player.tiers_2_inventory,
                potion.tiers_3_ingredient - player.tiers_3_inventory,
            ];

            let best_spell = get_best_spell(&mut spells, &mut player, missing_ingredients);

            action = match best_spell {
                Some(best_spell) => if best_spell.castable != 0 { format!("CAST {}", best_spell.id) } else { String::from("REST") }
                None => String::from("WAIT")
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // in the first league: BREW <id> | WAIT; later: BREW <id> | CAST <id> [<times>] | LEARN <id> | REST | WAIT
        println!("{}", action);
    }

    fn get_best_potion(brewing: &Vec<Potion>) -> Option<&Potion> {
        let best_potion = brewing.iter().max_by_key(|potion| potion.price);
        best_potion
    }

    fn get_best_spell<'a>(spells: &'a mut Vec<Spell>, player: &mut Player, mut missing_ingredients: [i32; 4]) -> Option<&'a Spell> {
        let mut spell;
        let best_spell = loop {
            spell = spells.iter().max_by_key(|s| s.add_useful_ingredient(missing_ingredients)).unwrap();
            if spell.enough_ingredient(&player) && spell.castable != 0 { break Some(spell) } else {
                missing_ingredients = [
                    spell.tiers_0_ingredient - player.tiers_0_inventory,
                    spell.tiers_1_ingredient - player.tiers_1_inventory,
                    spell.tiers_2_ingredient - player.tiers_2_inventory,
                    spell.tiers_3_ingredient - player.tiers_3_inventory,
                ];
                spells.retain(|s| s.id != spell.id);
            }
            if spells.is_empty() { break None }
        };
        best_spell
    }
}