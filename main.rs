use rand::Rng;
use std::io;

enum Action { //shit the player can do at the beginning of each loop, more variants to come
    Walk,
    Sneak,
    Help,
    Bind,
    Other,
}
struct Controls { //controls changed by Action::Bind which return Action::Walk and Action::Sneak respectively (please learn lifetimes and change Strings to &strs)
    walk: Vec<String>,
    sneak: Vec<String>,
}

struct World { //vectors associated with each Dimension, more fields likely to be added in time (please learn lifetimes and change Strings to &strs)
    sounds: Vec<String>,
    deaths: Vec<String>,
}
enum Dimension { //the dimension the player is in
    Overworld,
    Nether,
    End, //not yet implemented
}
struct Player { //info abt the player needed for various functions (please learn lifetimes and change Strings to &strs)
    name: String,
    world: Dimension,
    steps: u32,
    n_steps: u32,
    portal_dist: i32,
    health: i32,
    alive: bool,
    hunger: i32, //not yet implemented
}
impl Player {
    fn walk(mut self, portal_step: bool) -> Player {
        println!("\n   Footsteps");
        self.steps += 2;
        step_event(&mut self, true);
        if portal_step {
            self.portal_dist -= 2;
        }
        match self.world {
            Dimension::Nether => {
                self.health -= 1;
                println!("   Player burns on magma");
                println!(" Health: {}", self.health);
                if self.health > 0 {
                    advance_nether(self.n_steps)
                } else {
                    println!("       Game over!\n {} discovered the floor was lava", self.name);
                    self.alive = false;
                }
            },
            _ => (),
        }
        if self.health > 0 {
            advance(self.steps);
        }
        self
    }
    fn sneak(mut self, portal_step: bool) -> Player {
        println!("");
        self.steps += 1;
        step_event(&mut self, false);
        if portal_step {
            self.portal_dist -= 1;
        }
        if self.health > 0 {
            advance(self.steps);
            match self.world {
                Dimension::Nether => advance_nether(self.n_steps),
                _ => (),
            }
        }
        self
    }
    fn null(&mut self) {
        if you() == 0 {
            self.alive = false;
            now();
        }
    }
}

fn main() {
    let mut username = String::new();
    println!(" Please enter username to log in:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");

    let mut player = Player {
        name: username.trim().to_string(), //(please learn lifetimes and change Strings to &strs)
        world: Dimension::Overworld,
        steps: 0,
        n_steps: 0,
        portal_dist: 20,
        health: 20,
        alive: true,
        hunger: 20,
    };

    let splash: Vec<String> = ["Hello, world!", "Made in [REDACTED]!", "Way better than the original!", "Perfomance enhanced!", "Written in Rust!", "Totally necessary and not stupid!", "`struct Stupid` was not stupid!", "Yes!", "Poggers!", "You'll never guess me!", "Contains unsigned integers!", "Look, mom! I made a sequel to one of the best selling games of all time!", "Villagers are replaced by funko pops!", "So much lava!", "Is this enough splash texts?", "I shouldn't spend so much time on splashes!", "Maws can't spell sash text!", "Has no multiplayer!", "359 lines of code!", "$PLAYER IS YOU!", "Hello, $player!", "$player is a funny name!", "You, my dear $player, are playing a very good game!", "$player is pretty pog ngl!", "How are you today, $player?"].iter().map(|x| x.replace("$player", &player.name)).map(|x| x.replace("$PLAYER", &player.name.to_uppercase())).collect(); //(please learn lifetimes and change Strings to &strs)
    let splash_index = rand::thread_rng().gen_range(0, splash.len());

    println!(r#" ____    ____   ___   ___   ___   ________    _______    ________        ___       ________   ___________       ______
|    \  /    | |   | |   \ |   | |    ____|  /  ___  \  |   __   \      /   \     |    ____| |___     ___|    /  __   \
|     \/     | |   | |    \|   | |   |___   /  /   \__| |  |__|   |    |  _  |    |   |___       |   |       |__/  \   |
|            | |   | |         | |    ___| |  |     __  |        /    /  /_\  \   |    ___|      |   |            _/  /
|   |\  /|   | |   | |   |\    | |   |____  \  \___/  | |   |\   \   |  _____  |  |   |          |   |        ___/   /_
|___| \/ |___| |___| |___| \___| |________|  \_______/  |___| \___\ /__/     \__\ |___|          |___|       |_________| version 1.1
"#);
    println!("      {}", splash[splash_index]);

    let mut keybinds = Controls { //(please learn lifetimes and change Strings to &strs)
        walk: vec!["w", "a", "s", "d"].iter().map(|x| x.to_string()).collect(),
        sneak: vec!["W", "A", "S", "D"].iter().map(|x| x.to_string()).collect(),
    };
    let overworld = World { //(please learn lifetimes and change Strings to &strs)
        sounds: vec!["< Portal whooshes", "Portal whooshes", "Portal whooshes >", "< Zombie grunts", "Zombie grunts", "Zombie grunts >", "< Lava pops", "Lava pops", "Lava pops >", "< Orange Juice pops", "Orange Juice pops", "Orange Juice pops >", "< Tomato Sauce pops", "Tomato Sauce pops", "Tomato Sauce pops >", "< Cheese pops", "Cheese pops", "Cheese pops >", "< Funko Pop mumbles", "Funko Pop mumbles", "Funko Pop mumbles >", "< Minecart rolls", "Minecart rolls", "Minecart rolls >", "< Bat takes off", "Bat takes off", "Bat takes off >", "< Water flows", "Water flows", "Water flows >"].iter().map(|x| x.to_string()).collect(),
        deaths: vec!["fell from a high place", "tried to swim in lava", "was slain by Zombie", "walked into a cactus whilst trying to escape Husk", "hit the ground too hard", "fell off a ladder", "was squashed by a falling anvil"].iter().map(|x| x.to_string()).collect(),
    };
    let nether = World { //(please learn lifetimes and change Strings to &strs)
        sounds: vec!["< Portal whooshes", "Portal whooshes", "Portal whooshes >", "< Zombified Piglin grunts", "Zombified Piglin grunts", "Zombified Piglin grunts >", "< Lava pops", "Lava pops", "Lava pops >", "< Orange Juice pops", "Orange Juice pops", "Orange Juice pops >", "< Tomato Sauce pops", "Tomato Sauce pops", "Tomato Sauce pops >", "< Cheese pops", "Cheese pops", "Cheese pops >", "< Strider chirps ", "Strider chirps ", "Strider chirps >", "< Piglin snorts", "Piglin snorts", "Piglin snorts >", "< Magma Cube squishes", "Magma Cube squishes", "Magma Cube squishes >", "< Hoglin growls", "Hoglin growls", "Hoglin growls >"].iter().map(|x| x.to_string()).collect(),
        deaths: vec!["fell from a high place", "tried to swim in lava", "was slain by Zombified Piglin", "walked into a cactus whilst trying to escape Hoglin", "hit the ground too hard", "fell off some twisting vines", "was squashed by a falling anvil"].iter().map(|x| x.to_string()).collect(),
    };

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let loop_index: usize = rand::thread_rng().gen_range(0, match player.world {
            Dimension::Overworld => overworld.sounds.len(),
            Dimension::Nether => nether.sounds.len(),
            Dimension::End => {
                println!(" The End doesn't exist yet, the fuck are you doing there?");
                30
            }
        });
        let portal_step: bool = loop_index <= 2 && player.portal_dist > 0;

        match parse_action(&input, &keybinds) {
            Action::Walk => player = player.walk(portal_step),
            Action::Sneak => player = player.sneak(portal_step),
            Action::Help => help(&keybinds),
            Action::Bind => {
                let x = bind_keys();
                if let Ok(new) = x {
                    keybinds = new;
                } else if let Err(error) = x {
                    println!("\n Error: {}", error);
                    action_error();
                }
            },
            Action::Other => action_error(),
        }
        if player.health <= 0 && player.alive {
            let why = String::from(" died in the End. The End doesn't exist yet, the fuck are you doing dying there?"); //(please learn lifetimes and change Strings to &strs)
            println!("       Game over!");
            let index = rand::thread_rng().gen_range(0, overworld.deaths.len());
            println!(" {} {}", player.name, match player.world {
                Dimension::Overworld => &overworld.deaths[index],
                Dimension::Nether => &nether.deaths[index],
                Dimension::End => &why,
            });
            player.alive = false;
        }
        if player.alive == false {
            break;
        }
        if player.portal_dist <= 0 {
            match player.world {
                Dimension::Overworld => {
                    player.world = Dimension::Nether;
                    player.portal_dist = 20;
                    println!("\n   Portal noise intensifies\n   Ghast cries");
                },
                Dimension::Nether => {
                    credits(&player.name);
                    break;
                },
                Dimension::End => {
                    println!(" The End doesn't exist yet, the fuck are you doing there?");
                    break;
                },
            }
        } else {
            let why = String::from("Dragon roars\nThe End doesn't exist yet, the fuck are you doing there?"); //(please learn lifetimes and change Strings to &strs)
            println!("   {}\n", match player.world {
                Dimension::Overworld => &overworld.sounds[loop_index],
                Dimension::Nether => &nether.sounds[loop_index],
                Dimension::End => &why,
            });
        }
    }
    let mut final_words = String::new();
    io::stdin().read_line(&mut final_words).expect("Failed to read line.");
}

fn parse_action(input: &str, controls: &Controls) -> Action {
    match input.trim() {
        "h" | "H" => return Action::Help,
        "c" | "C" => return Action::Bind,
        key => { //(please learn lifetimes and change Strings to &strs)
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

//functions called by Actions
fn bind_keys() -> Result<Controls, &'static str> { //I don't actually know lifetimes I just stole it ok
    println!("\n Please input the 4 letters you wish to assign (note: the letters must be upper or lower case)\n");
    let mut rebind = String::new();
    io::stdin().read_line(&mut rebind).expect("Failed to read line.");
    rebind = rebind.trim().to_string(); //(please learn lifetimes and change Strings to &strs)
    let walk = rebind.to_lowercase();
    let sneak = rebind.to_uppercase();
    if rebind.len() != 4 {
        return Err("Invalid length!");
    } else if walk.chars().all(|x| x.is_lowercase()) == false {
        return Err("One or more invalid characters!");
    } else if walk.contains("c") || walk.contains("h") {
        return Err("Cannot overwrite default controls, `C` and `H`!");
    }
    println!("\n Your controls have been changed to {}!\n", walk);
    Ok(Controls { //(please learn lifetimes and change Strings to &strs)
        walk: walk.chars().map(|x| x.to_string()).collect(),
        sneak: sneak.chars().map(|x| x.to_string()).collect()
    })
}
fn help(controls: &Controls) {
    println!(r#"
 Help:
    To walk, enter "{}", "{}", "{}", or "{}"
    To sneak, enter "{}", "{}", "{}", or "{}"
    To see this help text, enter `H`
    To change the movement controls, enter `C` followed by 4 letters you wish to assign (note: the letters must be upper or lower case)
    Hint: sneaking increases your chances of survival, but is slower!
"#, controls.walk[0], controls.walk[1], controls.walk[2], controls.walk[3], controls.sneak[0], controls.sneak[1], controls.sneak[2], controls.sneak[3])
}
fn now() {let sun: &[u8] = &[89, 111, 117, 32, 119, 101, 114, 101, 32, 115, 108, 97, 105, 110, 32, 98, 121, 32, 72, 101, 114, 111, 98, 114, 105, 110, 101,
32, 97, 103, 97, 105, 110, 46]; println!("       Game over!\n {}", fall(sun));}
fn action_error() {
    println!(" Could not parse action. Enter `H` for list of possible actions.\n");
}
fn you() -> usize {rand::thread_rng().gen_range(0, 420)}
fn fall(sun: &[u8]) -> String {use std::str; str::from_utf8(&sun).unwrap().to_string()} //(please learn lifetimes and change Strings to &strs)
fn step_event(player: &mut Player, walk: bool) {
    let mut safety: usize = match player.world {
        Dimension::Overworld => 12,
        Dimension::Nether => 6,
        Dimension::End => 4,
    };
    if walk {
        safety /= 2;
    }
    let outcome = rand::thread_rng().gen_range(0, 50 * safety);
    match outcome {
        1..=100 => {
            player.health -= 1;
            println!("   Player hurts");
            println!(" Health: {}", player.health);
        },
        0 => player.null(),
        _ => (),
    }
}

//functions related to progression
fn advance(steps: u32) {
    const REQ: u32 = 128;
    if steps == REQ {
        println!("\n Advancement unlocked!\n      Hardcoded Hardcore:\n      Survive walking {} blocks!\n", REQ / 2);
    }
}
fn advance_nether(steps: u32) {
    const REQ: u32 = 40;   //┌──in case of odd number of Sneak uses followed by Walk passing the threshold
    if steps == REQ || steps == REQ + 1 {
        println!("\n Advancement unlocked!\n      Lucky as Hell:\n      Survive walking {} blocks in the Nether!\n", REQ / 2);
    }
}
fn credits(dreamer: &str) {
    println!(r#"
   Portal noise intensifies
   Chicken clucks

 Honestly, how did you get here?

 Advancement unlocked!
      To Hell and Back:
      Survive the Nether!

 I'm genuinely impressed.
 {}, the End doesn't exist yet, but you've earned this...
 Send a screenshot of this output to `Maws Enderteeth#7475` on Discord to collect your reward.

 One reward per player and no cheating, please. This and all previously versions of this offer expire 00:00 AM (CET), January 1st, 2021.

 Well played, {}. Well played indeed.
 Begin a new dream, a better dream. With graphics maybe.

                            Credits:
                     Created by  Maws Enderteeth
                   Developed by  Maws Enderteeth
                    Designed by  Maws Enderteeth
                 Play-tested by  Maws Enderteeth
                     Written by  Maws Enderteeth
                    Directed by  Maws Enderteeth
                         Art by  Maws Enderteeth
              Tooth modeling by  Maws Enderteeth
                      Funded by  Kim & Jack Crimes Inc.
           Moral support led by  an unspecified Alex
        Moral support server by  Maws Enderteeth#7475
     Backwards name spelling by  Hteetredne Swam
 Moth Appreciation Committee by  Maws Enderteeth
        Bugs and their fixes by  Maws Enderteeth
          Annoying fan noise by  Maws Enderteeth's laptop
             `struct Stupid` by  Maws Enderteeth
             Lead dev's name by  Maws Enderteeth
                    Lead dev by  Maws Enderteeth's mom
          Credits creativity by  Maws Enderteeth
                                 ReveredOxygen
                                 Rith
                                 Xe
            Credits alignment by  Maws Enderteeth
               Credits rated by  Havilah
            Impeccable humor by  Maws Enderteeth
        Pats on his own back by  Maws Enderteeth

                       Special thanks to:

                        AccessViolation,
          for his support and inspiration to learn the
 Rust Programming Language and making this silly game possible.

                         Aether Killer,
     for making fun of me sometimes, telling me to get some
       sleep, and other important forms of moral support.

                            Havilah,
 for their additional moral support and telling me to get food.

                         ReveredOxygen,
      for their help, support, ideas, and most importantly
              prompt to make this game in one day.

      "Minecraft" intellectual property owned by Microsoft
       This application is a free and open-source parody,
          developed as a learning project and for fun.
 Visit https://www.minecraft.net/en-us/get-minecraft to play a real game!
"#, dreamer, dreamer);
}
// 0123456789|123456789|123456789||987654321|987654321|9876543210
