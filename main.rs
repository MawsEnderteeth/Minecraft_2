use rand::Rng;
use std::io;
enum Action {
    Walk,
    Sneak,
    Help,
    Bind,
    Other,
}
struct Controls {
    walk: Vec<String>,
    sneak: Vec<String>,
}
fn main() {
    let mut username = String::new();
    println!("Please enter username to log in:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");
    username = username.trim().to_string();

    let splash: Vec<String> = ["Hello, world!", "Made in [REDACTED]!", "Way better than the original!", "Perfomance enhanced!", "Written in Rust!", "Totally necessary and not stupid!", "`struct Stupid` was not stupid!", "Yes!", "Poggers!", "You'll never guess me!", "Contains unsigned integers!", "Look, mom! I made a sequel to one of the best selling games of all time!", "Villagers are replaced by funko pops!", "So much lava!", "Is this enough splash texts?", "I shouldn't spend so much time on splashes!", "Maws can't spell sash text!", "Has no multiplayer!", "$PLAYER IS YOU!", "Hello, $player!", "$player is a funny name!", "You, my dear $player, are playing a very good game!", "$player is pretty pog ngl!", "How are you today, $player?"].iter().map(|x| x.replace("$player", &username)).map(|x| x.replace("$PLAYER", &username.to_uppercase())).collect();
    let splash_index = rand::thread_rng().gen_range(0, splash.len());

    println!(r#" ____    ____   ___   ___   ___   ________    _______    ________        ___       ________   ___________       ______
|    \  /    | |   | |   \ |   | |    ____|  /  ___  \  |   __   \      /   \     |    ____| |___     ___|    /  __   \
|     \/     | |   | |    \|   | |   |___   /  /   \__| |  |__|   |    |  _  |    |   |___       |   |       |__/  \   |
|            | |   | |         | |    ___| |  |     __  |        /    /  /_\  \   |    ___|      |   |            _/  /
|   |\  /|   | |   | |   |\    | |   |____  \  \___/  | |   |\   \   |  _____  |  |   |          |   |        ___/   /_
|___| \/ |___| |___| |___| \___| |________|  \_______/  |___| \___\ /__/     \__\ |___|          |___|       |_________| version 1.0
"#);
    println!("    {}", splash[splash_index]);

    let mut keybinds = Controls {
        walk: vec!["w", "a", "s", "d"].iter().map(|x| x.to_string()).collect(),
        sneak: vec!["W", "A", "S", "D"].iter().map(|x| x.to_string()).collect(),
    };

    let ow_sounds = vec!["< Zombie grunts", "Zombie grunts", "Zombie grunts >", "< Lava pops", "Lava pops", "Lava pops >", "< Orange Juice pops", "Orange Juice pops", "Orange Juice pops >", "< Tomato Sauce pops", "Tomato Sauce pops", "Tomato Sauce pops >", "< Cheese pops", "Cheese pops", "Cheese pops >", "< Funko Pop mumbles", "Funko Pop mumbles", "Funko Pop mumbles >", "< Minecart rolls", "Minecart rolls", "Minecart rolls >", "< Bat takes off", "Bat takes off", "Bat takes off >", "< Water flows", "Water flows", "Water flows >", "< Portal whooshes", "Portal whooshes", "Portal whooshes >"];
    let mut steps = 0;
    let mut nether_prox = 20;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let ow_sounds_index = rand::thread_rng().gen_range(0, ow_sounds.len());

        match parse_action(&input, &keybinds) {
            Action::Walk => {
                println!("   Footsteps");
                steps += 2;
                if is_deadly(&username, 4) {
                break;
                }
                if ow_sounds_index <= 29 && ow_sounds_index >= 27 && nether_prox > 0 {
                    nether_prox -= 2;
                }
                advance(steps);
            }
            Action::Sneak => {
                steps += 1;
                if is_deadly(&username, 8) {
                    break;
                }
                if ow_sounds_index <= 29 && ow_sounds_index >= 27 && nether_prox > 0 {
                    nether_prox -= 1;
                }
                advance(steps);
            },
            Action::Help => help(&keybinds),
            Action::Bind => {
                let x = bind_keys();
                if let Ok(new) = x {
                    keybinds = new;
                } else if let Err(error) = x {
                    println!("Error: {}", error);
                    action_error();
                }
            },
            Action::Other => action_error(),
        }

        if nether_prox == 0 {
            println!("   Portal noise intensifies");
            println!("   Ghast cries");
            break;
        }
        println!("   {}", ow_sounds[ow_sounds_index]);
    }

    let nether_sounds = vec!["< Zombified Piglin grunts", "Zombified Piglin grunts", "Zombified Piglin grunts >", "< Lava pops", "Lava pops", "Lava pops >", "< Orange Juice pops", "Orange Juice pops", "Orange Juice pops >", "< Tomato Sauce pops", "Tomato Sauce pops", "Tomato Sauce pops >", "< Cheese pops", "Cheese pops", "Cheese pops >", "< Strider chirps ", "Strider chirps ", "Strider chirps >", "< Piglin snorts", "Piglin snorts", "Piglin snorts >", "< Magma Cube squishes", "Magma Cube squishes", "Magma Cube squishes >", "< Hoglin growls", "Hoglin growls", "Hoglin growls >", "< Portal whooshes", "Portal whooshes", "Portal whooshes >"];
    let mut nether_steps = 0;
    let mut overworld_prox = 20;
    let mut health = 10;
    while nether_prox == 0 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let nether_sounds_index = rand::thread_rng().gen_range(0, nether_sounds.len());

        match parse_action(&input, &keybinds) {
            Action::Walk => {
                walking();
                
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
                if nether_sounds_index <= 29 && nether_sounds_index >= 27 && nether_prox > 0 {
                    overworld_prox -= 2;
                }
                advance_nether(nether_steps);
            }
            Action::Sneak => {
                sneaking();

                nether_steps += 1;
                if is_deadly(&username, 4) {
                    break;
                }
                if nether_sounds_index <= 29 && nether_sounds_index >= 27 && nether_prox > 0 {
                    overworld_prox -= 1;
                }
                advance_nether(nether_steps);
            },
            Action::Help => help(&keybinds),
            Action::Bind => {
                let x = bind_keys();
                if let Ok(new) = x {
                    keybinds = new;
                } else if let Err(error) = x {
                    println!("Error: {}", error);
                    action_error();
                }
            },
            Action::Other => action_error(),
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
        println!("   {}", nether_sounds[nether_sounds_index]);
    }
    let mut final_words = String::new();
    io::stdin()
        .read_line(&mut final_words)
        .expect("Failed to read line.");
}

fn parse_action(input: &str, controls: &Controls) -> Action {
    match input.trim() {
        "h" | "H" => return Action::Help,
        "c" | "C" => return Action::Bind,
        key => {
            if controls.walk.contains(&key.to_string()) {
                return Action::Walk;
            } else if controls.sneak.contains(&key.to_string()) {
                return Action::Sneak;
            } else {
                return Action::Other;
            }
        }
    }
}

//functions directly related to Actions
fn walking() {
    println!("   Footsteps");
}

fn sneaking() {

}

fn bind_keys() -> Result<Controls, &'static str> {
    println!("Please input the 4 letters you wish to assign (note: the letters must be upper or lower case)");
    let mut rebind = String::new();
    io::stdin().read_line(&mut rebind).expect("Failed to read line.");
    rebind = rebind.trim().to_string();
    let walk = rebind.to_lowercase();
    let sneak = rebind.to_uppercase();
    if rebind.len() != 4 {
        return Err("Invalid length!");
    } else if walk.chars().all(|x| x.is_lowercase()) == false {
        return Err("One or more invalid characters!");
    } else if walk.contains("c") || walk.contains("h") {
        return Err("Cannot overwrite default controls, `C` and `H`!");
    }
    println!("Your controls have been changed to {}", walk);
    Ok(Controls {
        walk: walk.chars().map(|x| x.to_string()).collect(),
        sneak: sneak.chars().map(|x| x.to_string()).collect()
    })
}

fn help(controls: &Controls) {
    println!(r#"Help:
    To walk, enter "{}", "{}", "{}", or "{}"
    To sneak, enter "{}", "{}", "{}", or "{}"
    To see this help text, enter `H`
    To change the movement controls, enter `C` followed by 4 letters you wish to assign (note: the letters must be upper or lower case)
    Hint: sneaking increases your chances of survival!"#, controls.walk[0], controls.walk[1], controls.walk[2], controls.walk[3], controls.sneak[0], controls.sneak[1], controls.sneak[2], controls.sneak[3])
}

fn action_error() {
    println!("Could not parse action. Enter `H` for list of possible actions.");
}

//functions called by several Actions
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
