pub struct Bowling {
    rolls: Vec<i64>,
}

impl Bowling {
    pub(crate) fn get_score(&self) -> i64 {
        let mut score = 0;
        for frame in 0..self.rolls.len(){
            score+=self.rolls[frame];
            if frame>=2 && (self.rolls[frame-1]+self.rolls[frame-2])==10 {
                score+=self.rolls[frame]
            }

        }
        return score;
    }
}

impl Bowling {
    pub(crate) fn roll(&mut self, pins_down: i64) {
        self.rolls.push(pins_down)
    }
}

pub fn build_game(rolls: Vec<i64>) -> Bowling {
    Bowling {
        rolls
    }
}
