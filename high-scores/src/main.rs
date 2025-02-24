#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort();
        sorted.reverse();

        sorted.into_iter().take(3).collect()
    }
}

fn main() {
    let scores = vec![5, 4, 3, 2, 1];

    let hi_scores = HighScores::new(&scores);

    println!("{:?}", hi_scores.scores());
    println!("{:?}", hi_scores.latest());
    println!("{:?}", hi_scores.personal_best());
    println!("{:?}", hi_scores.personal_top_three());
}