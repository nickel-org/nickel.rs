use util::*;

use std::thread;
use std::sync::Arc;

#[test]
#[should_panic(expected = "Expected '5 unique numbers': got [1, 2, 4, 5]")]
fn sanity() {
    // ensure the test panics when things are bad
    assert_unique(5, vec![1, 2, 2, 4, 5]);
}

#[test]
fn returns_incrementing_number() {
    run_example("route_data", |port| {
        let urls = [format!("http://localhost:{}/a", port),
                    format!("http://localhost:{}/b", port)];
        let arc = Arc::new(urls);

        // FIXME: Use a pool instead of spawning so many threads!
        const THREADS_TO_SPAWN: usize = 100;

        let threads = (0..THREADS_TO_SPAWN).map(|i| {
            let urls = arc.clone();

            thread::spawn(move || {
                // split the threads between /a and /b
                let url = &urls[i % 2];

                let s = read_url(url);
                s.parse::<usize>().unwrap()
            })
        }).collect::<Vec<_>>();

        let results = threads.into_iter()
                             .map(|t| t.join().unwrap())
                             .collect::<Vec<_>>();

        assert_unique(THREADS_TO_SPAWN, results)
    })
}

fn assert_unique(len: usize, mut values: Vec<usize>) {
    assert_eq!(values.len(), len);
    values.dedup();
    assert!(values.len() == len,
            "Expected '{} unique numbers': got {:?}", len, values);
}
