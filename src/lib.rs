
mod bowling;

#[cfg(test)]
mod bowling_test {
    use crate::bowling::build_game;

    #[test]
    fn should_score_gutter_game() {
        let mut bowling =build_game(Vec::new());
        bowling.roll(0);
        assert_eq!(0, bowling.get_score());
    }
    #[test]
    fn should_score_game_of_ones() {
        let mut bowling =build_game(Vec::new());
        bowling.roll(1);
        assert_eq!(1, bowling.get_score());
    }
    #[test]
    fn should_score_spare() {
        let mut bowling =build_game(Vec::new());
        bowling.roll(7);
        bowling.roll(3);
        bowling.roll(4);
        assert_eq!((7+3)+4+4, bowling.get_score());
    }
}
