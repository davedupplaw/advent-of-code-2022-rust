pub mod day02 {
    use std::str::Split;

    struct Play {
        theirs: String,
        yours: &'static str,
    }

    fn to_value(ref mut play: Split<&str>) -> Play {
        let p = Play {
            theirs: String::from(play.next().unwrap()),
            yours: map_yours(play.next().unwrap()),
        };
        return p;
    }

    fn map_yours(yours: &str) -> &'static str {
        return match yours {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => panic!("Should not be possible")
        };
    }

    fn play_score(ref play: &Play) -> u64 {
        let mut score: u64 = 0;
        let s = play.yours;
        let choice_score: u64 = match s {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0
        };
        score += choice_score;
        return score;
    }

    fn win_score(ref play: &Play) -> u64 {
        if play.theirs.eq(play.yours) {
            return 3;
        }

        if (play.theirs.eq("A") && play.yours.eq("C")) ||
            (play.theirs.eq("B") && play.yours.eq("A")) ||
            (play.theirs.eq("C") && play.yours.eq("B")) {
            return 0;
        }

        return 6;
    }

    pub fn part1() {
        println!("----- Day 02 Part 1 ------");

        let lines: u64 = include_str!("inputs/day02.txt")
            .lines()
            .map(|play| play.split(" "))
            .map(|play| to_value(play))
            .map(|play| play_score(&play) + win_score(&play))
            .sum();

        println!("part1 = {}", lines);
    }
}
