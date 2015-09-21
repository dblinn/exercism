extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::HashMap;

type WorkerMap = Arc<Mutex<HashMap<char,usize>>>;

pub fn frequency(input: &[&str], workers: usize) -> HashMap<char, usize> {
    let mut worker_output = Vec::with_capacity(workers);
    for _ in (0..workers) { worker_output.push(Arc::new(Mutex::new(HashMap::new()))); }

    parallel_count_frequencies(input, &worker_output)
}

fn parallel_count_frequencies(input: &[&str], results: &Vec<WorkerMap>) -> HashMap<char, usize>
{
    let (tx, rx) = mpsc::channel();
    let worker_count = results.len();
    let pool = ThreadPool::new(worker_count);
    let worker_indices = (0..worker_count).chain(rx.iter());

    for (s, worker_index) in input.iter().zip(worker_indices) {
        let tx = tx.clone();
        let worker_hash_map = results[worker_index].clone();
        let sequence = s.to_string();
        let current_index = worker_index.clone();

        pool.execute(move || {
            count_frequency_for_word(&sequence, &mut worker_hash_map.lock().unwrap());
            match tx.send(current_index) {
                Err(mpsc::SendError(index)) => { println!("Error on worker {}: {}", index, sequence) },
                _ => {}
            }
        });
    }

    wait_for_remaining_workers(worker_count, input.len(), &rx);
    accumulate_results(results)
}

fn count_frequency_for_word(input: &str, frequencies: &mut HashMap<char, usize>) {
    for c in input.chars().filter(|c| c.is_alphabetic()) {
        for lower in c.to_lowercase() {
            *frequencies.entry(lower).or_insert(0) += 1;
        }
    }
}

fn accumulate_results(results: &Vec<WorkerMap>) -> HashMap<char,usize> {
    let mut cumulative: HashMap<char,usize> = HashMap::new();
    for output in results {
        for (c, count) in output.lock().unwrap().iter() {
            *cumulative.entry(*c).or_insert(0) += *count;
        }
    }

    cumulative
}

fn wait_for_remaining_workers(worker_count: usize, input_size: usize,
    rx: &mpsc::Receiver<usize>)
{
    for _ in rx.iter().take(std::cmp::min(worker_count, input_size)) {}
}
