pub type Domino = (usize, usize);

pub fn chain(dominoes: &Vec<Domino>) -> Option<Vec<Domino>> {
    let mut input = dominoes.iter().cloned().collect();
    if dominoes.is_empty() { return Some(input) }

    let mut output = Vec::<Domino>::with_capacity(dominoes.len());
    for i in 0 .. input.len() {
        let domino = input.remove(i);
        output.push(domino);
        if scan_for_chain(domino, &mut input, &mut output) { return Some(output); }
        output.pop();
        input.insert(i, domino);
    }

    None
}

pub fn scan_for_chain(domino: Domino, input: &mut Vec<Domino>, output: &mut Vec<Domino>) -> bool
{
    if input.is_empty() { return true }

    let matching_dominoes = input.iter()
        .enumerate()
        .filter(|&(_, other_domino)| domino.1== other_domino.0)
        .map(|(i, domino_ref)| (i, *domino_ref))
        .collect::<Vec<(usize, Domino)>>();
    if matching_dominoes.is_empty() { return false }
    
    for (i, matching_domino) in matching_dominoes
    {
        input.remove(i);
        output.push(matching_domino);
        if scan_for_chain(matching_domino, input, output) { return true; }
        output.pop();
        input.insert(i, matching_domino);
    }

    false
}
