use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

fn update(sums: &mut Arc<[Mutex<usize>; 10]>, input: File) {
    let reader = BufReader::new(input);
    for row in reader.lines().map(Result::unwrap) {
        if let Some((key_str, val_str)) = row.split_once(',') {
            let key: usize = key_str.parse().unwrap();
            let val: usize = val_str.parse().unwrap();
            *sums[key].lock().unwrap() += val;
        } else {
            panic!("Bad line: {}", row);
        }
    }
}

fn main() {
    let sums: Arc<[Mutex<usize>; 10]> = Default::default();
    let files_names = args().into_iter().skip(1);
    let handles = files_names.map(|name| {
        let mut sums = Arc::clone(&sums);
        std::thread::spawn(move || {
            let input = File::open(name).unwrap();
            update(&mut sums, input);
        })
    });
    handles.for_each(|h| {
        h.join().unwrap();
    });
    println!("{:?}", sums);
}
