use {
    altered_states_2::states::State,
    std::{
        collections::{HashMap, HashSet, VecDeque},
        fs::File,
        io::BufReader,
    },
};

fn main() {
    let states = import_states("states.json".to_string());
    let letters_set: HashSet<char> = get_all_letters_in_states(&states);
    println!(
        "All letters: (len: {}) {:?}",
        letters_set.len(),
        letters_set
    );
    let letters_frequencies: HashMap<char, u8> = get_letter_frequencies(&states);
    println!("Letter freqs: {:?}", letters_frequencies);
    let mut count_vec: Vec<_> = letters_frequencies.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("Letter freqs: {:?}", count_vec);
    // let n: i16 = 26;
    // let k: i16 = 5;
    // let min: i16 = std::cmp::min(n, k);
    // for i in 0..min {
    //     let mut sets: VecDeque<u8> = VecDeque::new();
    //     for j in 0..n {
    //         sets.push_back(j as u8);
    //     }
    //     println!("num partitions: {}", partitions(sets, i).len());
    // }
}

fn import_states(path: String) -> Vec<State> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

#[allow(dead_code)]
fn partitions(sets: VecDeque<u8>, k: i16) -> Vec<Vec<Vec<u8>>> {
    if k == 1 {
        return vec![vec![sets.into()]];
    }
    if sets.len() == 0 {
        return vec![];
    }
    let mut result: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut sets_new: VecDeque<u8> = VecDeque::from(sets.clone());
    let first: u8 = sets_new.pop_front().unwrap();

    if k - 1 <= sets_new.len() as i16 {
        let p1 = partitions(sets_new.clone(), k - 1);
        for p in p1 {
            let mut p_new = p.clone();
            p_new.push(vec![first]);
            result.push(p_new);
        }
    }
    if k <= sets_new.len() as i16 {
        let p2 = partitions(sets_new.clone(), k);

        for p in p2 {
            for idx in 0..p.len() {
                let mut p_new = p.clone();
                p_new[idx].push(first);
                result.push(p_new);
            }
        }
    }
    return result;
}

fn get_all_letters_in_states(states: &Vec<State>) -> HashSet<char> {
    let mut letters: HashSet<char> = HashSet::with_capacity(26);
    for state in states {
        for c in state.name.chars() {
            letters.insert(c);
        }
    }
    letters
}

fn get_letter_frequencies(states: &Vec<State>) -> HashMap<char, u8> {
    let mut freqs: HashMap<char, u8> = HashMap::with_capacity(26);
    for state in states {
        for c in state.name.chars() {
            match freqs.get(&c) {
                Some(count) => freqs.insert(c, count + 1),
                None => freqs.insert(c, 1),
            };
        }
    }
    freqs
}
