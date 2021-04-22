use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
};

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let (producer, consumer) = mpsc::channel::<HashMap<char, usize>>();

    let freq = Arc::new(Mutex::new(HashMap::new()));
    let freq_clone = freq.clone();
    let consumer_thread = std::thread::spawn(move || {
        while let Ok(map) = consumer.recv() {
            let mut freq_map = freq_clone.lock().unwrap();
            for (c, count) in map.into_iter() {
                *freq_map.entry(c).or_default() += count;
            }
        }
    });

    for thread in input
        .chunks((input.len() as f64 / worker_count as f64).ceil() as usize)
        .map(|chunk| {
            let producer_clone = producer.clone();
            let chunk = chunk.to_vec().into_iter().map(ToOwned::to_owned);
            std::thread::spawn(move || {
                let mut submap = HashMap::new();
                for string in chunk {
                    for c in string.chars() {
                        *submap.entry(c).or_default() += 1;
                    }
                }
                producer_clone.send(submap).unwrap();
            })
        })
    {
        thread.join().unwrap();
    }

    // hang up the sender
    drop(producer);

    consumer_thread.join().unwrap();

    let result = Arc::try_unwrap(freq).unwrap().into_inner().unwrap();
    dbg!(result)
}
