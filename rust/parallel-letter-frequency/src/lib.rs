extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::HashMap;

type WorkerMap = Arc<Mutex<HashMap<char,usize>>>;

pub fn frequency(input: &[&str], workers: usize) -> HashMap<char, usize> {
    let mut worker_output = Vec::with_capacity(workers);
    for _ in (0..workers) { worker_output.push(Arc::new(Mutex::new(HashMap::new()))); }
    let (tx, rx) = mpsc::channel();

    parallel_count_frequencies(input, &worker_output, tx);
    wait_for_jobs_to_finish(rx, input.len());
    accumulate_results(&worker_output)
}

fn parallel_count_frequencies(input: &[&str], results: &Vec<WorkerMap>, tx: mpsc::Sender<usize>)
{
    let pool = ThreadPool::new(results.len());

    for (i, s) in input.iter().enumerate() {
        let tx = tx.clone();
        let worker_hash_map = results[i % results.len()].clone();
        let sequence = s.to_string();
        pool.execute(move || {
            count_frequency_for_word(sequence, &mut worker_hash_map.lock().unwrap());
            tx.send(i).unwrap();
        });
    }
}

fn count_frequency_for_word(input: String, frequencies: &mut HashMap<char, usize>) {
    for c in input.chars().filter(|c| c.is_alphabetic()) {
        for lower in c.to_lowercase() {
            *frequencies.entry(lower).or_insert(0) += 1;
        }
    }
}

fn wait_for_jobs_to_finish(rx: mpsc::Receiver<usize>, message_count: usize) {
    for _ in rx.iter().take(message_count) { }
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
