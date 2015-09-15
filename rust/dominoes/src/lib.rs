pub type Domino = (usize, usize);

pub fn chain(dominoes: &Vec<Domino>) -> Option<Vec<Domino>> {
    for p in [1,2,3,4].permutations() {
        println!("{:?}", p);
    }
    Some(dominoes.iter().cloned().collect())
}
