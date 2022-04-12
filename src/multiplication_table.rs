pub mod calc {
    pub fn print() {
        for i in 1..=9 {
            for j in 1..=i {
                print!("{} x {} = {}  ", j, i, i * j);
            }
            print!("\n");
        }
    }
}
