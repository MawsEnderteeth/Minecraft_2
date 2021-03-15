use rand::Rng;
use std::io;
enum Action {
    Walk,
    Sneak,
    Help,
    Controls(String),
    Other,
}
fn main() {
    let mut username = String::new();
    println!("Please enter username to log in:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");
    username = username.trim().to_string();

    let splash = vec!["Hello, world!", "Made in [REDACTED]!", "Way better than the original!", "Perfomance enhanced!", "Written in Rust!", "Totally necessary and not stupid!", "`struct Stupid` is not stupid!", "Yes!", "Poggers!", "You'll never guess me!", "Contains unsigned integers!", "Look, mom! I made a sequel to one of the best selling games of all time!", "Villagers are replaced by funko pops!", "So much lava!", "Is this enough splash texts?", "I shouldn't spend so much time on splashes!", "Maws can't spell sash text!", "Has no multiplayer!", "$PLAYER IS YOU!", "Hello, $player!", "$player is a funny name!", "You, my dear $player, are playing a very good game!", "$player is pretty pog ngl!", "How are you today, $player?"].replace("$PLAYER", username.to_uppercase()).replace("$player", username);
    let splash_index = rand::thread_rng().gen_range(0, splash.len());

    println!(r#" ____    ____   ___   ___   ___   ________    _______    ________        ___       ________   ___________       ______
|    \  /    | |   | |   \ |   | |    ____|  /  ___  \  |   __   \      /   \     |    ____| |___     ___|    /  __   \
|     \/     | |   | |    \|   | |   |___   /  /   \__| |  |__|   |    |  _  |    |   |___       |   |       |__/  \   |
|            | |   | |         | |    ___| |  |     __  |        /    /  /_\  \   |    ___|      |   |            _/  /
|   |\  /|   | |   | |   |\    | |   |____  \  \___/  | |   |\   \   |  _____  |  |   |          |   |        ___/   /_
|___| \/ |___| |___| |___| \___| |________|  \_______/  |___| \___\ /__/     \__\ |___|          |___|       |_________| (beta)
"#);
    println!("    {}", splash[splash_index]);

    let mut walk = vec!["w", "a", "s", "d"];

    let sounds = vec!["< Zombie grunts", "Zombie grunts", "Zombie grunts >", "< Lava pops", "Lava pops", "Lava pops >", "< Orange Juice pops", "Orange Juice pops", "Orange Juice pops >", "< Tomato Sauce pops", "Tomato Sauce pops", "Tomato Sauce pops >", "< Cheese pops", "Cheese pops", "Cheese pops >", "< Funko Pop mumbles", "Funko Pop mumbles", "Funko Pop mumbles >", "< Minecart rolls", "Minecart rolls", "Minecart rolls >", "< Bat takes off", "Bat takes off", "Bat takes off >", "< Water flows", "Water flows", "Water flows >", "< Portal whooshes", "Portal whooshes", "Portal whooshes >"];
    let mut steps = 0;
    let mut nether_prox = 10;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let sounds_index = rand::thread_rng().gen_range(0, sounds.len());

        match parse_action(&input, &walk) {
            Action::Walk => {
                println!("   Footsteps");
                steps += 2;
                if is_deadly(&username, 4) {
                break;
                }
                if sounds_index <= 29 && sounds_index >= 27 && nether_prox > 0 {
                    nether_prox -= 2;
                }
                advance(steps);
            }
            Action::Sneak => {
                steps += 1;
                if is_deadly(&username, 8) {
                    break;
                }
                if sounds_index <= 29 && sounds_index >= 27 && nether_prox > 0 {
                    nether_prox -= 1;
                }
                advance(steps);
            },
            Action::Help => {
                println!(r#"Help:
    To walk, enter {}
    To sneak, enter {}
    To see this help text, enter `H`
    To change the movement controls, enter `C` followed by 4 letters you wish to assign (note: the letters must be upper or lower case)
    Hint: sneaking increases your chances of survival!"#, walk, walk.to_uppercase());
            },
            Action::Controls(new) => {
                walk = vec![new[0], new[1], new[2], new[3]];
                println!("Your controls have been changed to {}", walk);
            }
            Action::Other => {println!("Could not parse action. Enter `H` for list of possible actions.");},
        }

        if nether_prox == 0 {
            println!("   Portal noise intensifies");
            println!("   Ghast cries");
            break;
        }
        println!("   {}", sounds[sounds_index]);
    }

    let nether_sounds = vec!["< Zombified Piglin grunts", "Zombified Piglin grunts", "Zombified Piglin grunts >", "< Lava pops", "Lava pops", "Lava pops >", "< Orange Juice pops", "Orange Juice pops", "Orange Juice pops >", "< Tomato Sauce pops", "Tomato Sauce pops", "Tomato Sauce pops >", "< Cheese pops", "Cheese pops", "Cheese pops >", "< Strider chirps ", "Strider chirps ", "Strider chirps >", "< Piglin snorts", "Piglin snorts", "Piglin snorts >", "< Magma Cube squishes", "Magma Cube squishes", "Magma Cube squishes >", "< Hoglin growls", "Hoglin growls", "Hoglin growls >", "< Portal whooshes", "Portal whooshes", "Portal whooshes >"];
    let mut nether_steps = 0;
    let mut overworld_prox = 6;
    let mut health = 10;
    while nether_prox == 0 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
            let sounds_index = rand::thread_rng().gen_range(0, nether_sounds.len());

        match parse_action(&input, &walk) {
            Action::Walk => {
                println!("   Footsteps");
                nether_steps += 2;
            health -= 1;
            if health > 0 {
                println!("   Player burns on magma");
            } else {
                println!("     Game over!");
                println!("{} discovered the floor was lava", username);
                break;
            }
                if is_deadly(&username, 4) {
                break;
                }
                if sounds_index <= 29 && sounds_index >= 27 && nether_prox > 0 {
                    overworld_prox -= 2;
                }
                advance_nether(nether_steps);
            }
            Action::Sneak => {
                nether_steps += 1;
                if is_deadly(&username, 4) {
                    break;
                }
                if sounds_index <= 29 && sounds_index >= 27 && nether_prox > 0 {
                    overworld_prox -= 1;
                }
                advance_nether(nether_steps);
            },
            Action::Help => {
                println!(r#"Help:
    To walk, enter {}
    To sneak, enter {}
    To see this help text, enter `H`
    To change the movement controls, enter `C` followed by 4 letters you wish to assign (note: the letters must be upper or lower case)
    Hint: sneaking increases your chances of survival!"#, walk, walk.to_uppercase());
            },
            Action::Controls(new) => {
                walk = vec![new[0], new[1], new[2], new[3]];
                println!("Your controls have been changed to {}", walk);
            }
            Action::Other => {println!("Could not parse action. Enter `H` for list of possible actions.");},
        }
        if overworld_prox == 0 {
            println!(r#"   Portal noise intensifies
   Chicken clucks
Honestly, how did you get here?
Advancement unlocked!
   To Hell and Back:
   Survive the Nether!
I'm genuinely impressed.
{}, the End doesn't exist yet, but you've earned this...
Send a screenshot of this output to `Maws Enderteeth#7475` on Discord to collect your reward.
Well played, {}. Well played indeed.
Begin a new dream, a better dream. With graphics maybe.
                    Credits:
 Created, developed, and playtested by Maws Enderteeth
 Special thanks to: ReveredOxygen, for their help, support, ideas, and most importantly prompt to make this game in one day."#, username, username);
            break;
        }
        println!("   {}", nether_sounds[sounds_index]);
    }
    let mut final_words = String::new();
    io::stdin()
        .read_line(&mut final_words)
        .expect("Failed to read line.");
}

fn parse_action(input: &str, controls: &Vec<String>) -> Action {
    let bytes = input[1..].trim().to_lowercase().as_bytes();
    match input.chars().next() {
        'h' | 'H' => return Action::Help,
        'c' | 'C' => {
            if input[1..].trim().len() < 4 {
                return Action::Other;
            }
            for byte in bytes[0..4].iter() {
                if input.to_lowercase().chars().next().is_lowercase() && byte != b'h' && byte != b'c' {
                    continue;
                } else {
                    return Action::Other;
                }
            }
            let new = input[1..].trim().to_lowercase();
            return Action::Controls(new[0..3])
        },
        key => {
            if controls.contains(key) {
                return Action::Walk;
            } else if controls.to_uppercase().contains(key) {
                return Action::Sneak;
            } else {
                return Action::Other;
            }
        }
    }
}

fn is_deadly(player: &str, safety: usize) -> bool {
    let deaths = vec!["fell from a high place", "tried to swim in lava", "was slain by Zombie", "walked into a cactus whilst trying to escape Husk", "hit the ground too hard", "fell off a ladder", "was squashed by a falling anvil", "discovered the floor was lava"];
    let proceed = rand::thread_rng().gen_range(0, deaths.len() * safety);
    if proceed == deaths.len() {
        println!("     Game over!");
        println!("{} was struck by lightning whilst trying to escape Herobrine", player);
        return true;
    } else if proceed < deaths.len() {
        println!("     Game over!");
        println!("{} {}", player, deaths[proceed]);
        return true;
    }
    false
}

fn advance(steps: u32) {
    const REQ: u32 = 60;
    if steps == REQ {
        println!("Advancement unlocked!");
        println!("   Hardcoded Hardcore:");
        println!("   Survive walking {} blocks!", REQ / 2);
    }
}
fn advance_nether(steps: u32) {
    const REQ: u32 = 20;
    if steps == REQ {
        println!("Advancement unlocked!");
        println!("   Lucky as Hell:");
        println!("   Survive walking {} blocks in the Nether!", REQ / 2);
    }
} 