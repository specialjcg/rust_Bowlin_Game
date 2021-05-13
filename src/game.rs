pub struct Game {
    score: i64,
    vec: Vec<i64>,
}

impl Game {
    pub fn origin() -> Game {
        Game {
            score: 0,
            vec: Vec::new(),
        }
    }
    pub fn roll(&mut self, p0: i64) {
        self.new_score(p0);
    }
    pub fn completed(&self) -> bool {
        self.vec.len() == 19
    }
    pub fn new_score(&mut self, p0: i64) {
        self.vec.push(p0);
        let len = self.vec.len();
        if (len) % 2 == 0 && len > 1 && self.noStrike(len) {
            if (self.vec[len - 1] + self.vec[len - 2]) == 10 {
                self.Spare()
            }
        }
        if self.vec[len - 1] == 10 && len > 2 {
            self.Strike()
        }

        self.score = p0 + self.get_score();
    }

    fn noStrike(&mut self, len: usize) -> bool {
        self.vec[len - 1] != 10
    }

    fn Strike(&mut self) {
        self.score = self.score + 30
    }

    fn Spare(&mut self) {
        self.score = self.score + 5
    }

    pub fn get_score(&self) -> i64 {
        self.score
    }
}
