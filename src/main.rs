use rand::Rng;
use std::io::stdin;

// Cribbed from the 10th edition (latest) core rules.
fn get_dice_roll_from_rules(strength: i32, toughness: i32) -> i32 {
    // Implement these checks in the same order as the table from the core rules
    // to avoid bugs.
    if strength >= toughness * 2 {
        2
    } else if strength > toughness {
        3
    } else if strength == toughness {
        4
    // Swap these last two checks from the book order to avoid always returning 5.
    } else if strength * 2 <= toughness {
        6
    } else if strength < toughness {
        5
    } else {
        // this should be impossible, throw an error
        println!(
            "not sure how we got here, strength is {} and toughness is {}",
            strength, toughness
        );
        0
    }
}

fn get_dice_roll_from_user() -> i32 {
    let mut our_dice_roll = String::new();
    stdin()
        .read_line(&mut our_dice_roll)
        .expect("Failed to read line");

    match our_dice_roll.trim().parse::<i32>() {
        Ok(number) => number,
        Err(e) => {
            eprintln!("Failed to parse number: {}", e);
            0
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // Run forever - the user can break out with Control-C.
    loop {
        // Generate a random strength and toughness to quiz us on.
        // I picked the min and max pretty arbitrarily. Yes, I know there are
        // models that can exceed T12, but this is good enough for my purposes.
        let strength = rng.gen_range(2..=12);
        let toughness = rng.gen_range(2..=12);
        let true_dice_roll = get_dice_roll_from_rules(strength, toughness);

        println!(
            "You have strength {} and they have toughness {}, what min dice roll is needed?",
            strength, toughness
        );
        let our_dice_roll = get_dice_roll_from_user();

        if our_dice_roll == true_dice_roll {
            println!("Correct, nice job!");
        } else {
            println!(
                "{} is not correct. The right answer is {}",
                our_dice_roll, true_dice_roll
            );
        }
    }
}
