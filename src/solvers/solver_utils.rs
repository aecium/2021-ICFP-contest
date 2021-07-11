pub trait RandRound {
    fn rand_round(&self) -> Self;
}

impl RandRound for f64 {
    fn rand_round(&self) -> Self {
        if rand::random() {
            self.floor()
        } else {
            self.ceil()
        }
    }
}