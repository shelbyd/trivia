extern crate rand;

struct Player {
    name: String,
    coins: i32,
    location: i32,
    in_penaltybox: bool,
}

impl Player {
    fn with_name(name: String) -> Player {
        Player {
            name,
            coins: 0,
            location: 0,
            in_penaltybox: false,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Game {
    players: Vec<Player>,
    current_player: usize,

    pop_questions: Vec<String>,
    science_questions: Vec<String>,
    sports_questions: Vec<String>,
    rock_questions: Vec<String>,
}

impl Default for Game {
    fn default() -> Game {
        let create_question =
            |category: &str, x: usize| category.to_string() + " Question " + &x.to_string();
        let question_list =
            |category: &str| (0..50).map(|x| create_question(category, x)).collect();

        Game {
            players: vec![],
            current_player: 0,
            pop_questions: question_list("Pop"),
            science_questions: question_list("Science"),
            sports_questions: question_list("Sports"),
            rock_questions: question_list("Rock"),
        }
    }
}

impl Game {
    fn player_count(&self) -> usize {
        self.players.len()
    }

    fn current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    fn add(&mut self, player_name: String) {
        println!("{} was added", &player_name);
        self.players.push(Player::with_name(player_name));
        println!("They are player number {}", self.player_count());
    }

    fn next_player(&mut self) {
        self.current_player += 1;
        self.current_player %= self.player_count();
    }

    fn current_category_enum(&self) -> Category {
        let place = self.current_player().location;
        match place {
            0 | 4 | 8 => Category::Pop,
            1 | 5 | 9 => Category::Science,
            2 | 6 | 10 => Category::Sports,
            3 | 7 | 11 => Category::Rock,
            _ => Category::Rock,
        }
    }

    fn ask_question(&mut self) {
        println!("The category is {}", self.current_category_enum());
        let top = match self.current_category_enum() {
            Category::Pop => self.pop_questions.pop(),
            Category::Science => self.science_questions.pop(),
            Category::Sports => self.sports_questions.pop(),
            Category::Rock => self.rock_questions.pop(),
        };
        println!("{:?}", top.unwrap());
    }

    fn move_player(&mut self, distance: i32) {
        self.current_player_mut().location += distance;
        self.current_player_mut().location %= 12;
        println!(
            "{} 's new location is {}",
            self.current_player(),
            self.current_player().location
        );
    }

    fn roll(&mut self, roll: i32) -> bool {
        println!("{} is current player", self.current_player());
        println!("They have rolled a {}", roll);

        if self.will_play_with_roll(roll) {
            self.move_player(roll);
            self.ask_question();
            true
        } else {
            false
        }
    }

    fn will_play_with_roll(&self, roll: i32) -> bool {
        if !self.current_player().in_penaltybox {
            return true;
        }

        if roll % 2 == 0 {
            println!(
                "{} is not getting out of the penalty box",
                self.current_player()
            );
            false
        } else {
            println!(
                "{} is getting out of the penalty box",
                self.current_player()
            );
            true
        }
    }

    fn give_coin(&mut self) {
        self.current_player_mut().coins += 1;
        println!(
            "{} now has {} Gold Coins.",
            self.current_player(),
            self.current_player().coins
        );
    }

    fn wrong_answer(&mut self) {
        println!("Question was incorrectly answered");
        println!("{} was sent to the penalty box", self.current_player());
        self.current_player_mut().in_penaltybox = true;
        self.next_player();
    }

    fn correct_answer(&mut self, leaving_penalty: bool) -> bool {
        if !self.current_player().in_penaltybox || leaving_penalty {
            println!("Answer was correct!!!!");
            self.give_coin();
        }

        let keep_playing = self.current_player().coins != 6;
        self.next_player();
        keep_playing
    }
}

#[derive(Debug)]
enum Category {
    Pop,
    Science,
    Sports,
    Rock,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Category::Pop => write!(f, "Pop"),
            Category::Science => write!(f, "Science"),
            Category::Sports => write!(f, "Sports"),
            Category::Rock => write!(f, "Rock"),
        }
    }
}

fn main() {
    use rand::*;
    let seed = std::env::var("SEED").ok().and_then(|s| s.parse().ok());
    let mut rng: Box<Rng> = seed
        .map(|s| Box::new(rand::StdRng::from_seed(&[s])) as Box<Rng>)
        .unwrap_or_else(|| Box::new(thread_rng()));

    let mut game = Game::default();
    game.add("Chet".to_string());
    game.add("Pat".to_string());
    game.add("Sue".to_string());
    loop {
        let leaving_penalty = game.roll(rng.gen::<i32>() % 5 + 1);
        let answered_correctly = rng.gen::<i32>() % 9 != 7;
        if answered_correctly {
            if !game.correct_answer(leaving_penalty) {
                break;
            }
        } else {
            game.wrong_answer();
        }
    }
}
