mod raw {
    use std::{collections::HashMap, sync::mpsc};

    pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
        let (producer, consumer) = mpsc::channel::<HashMap<_, usize>>();

        // avoid the need for a mutex (and thereby the need to lock on every message)
        // by using a 1 element channel to communicate the final result
        let (result_tx, result_rx) = mpsc::sync_channel(1);

        let consumer_thread = std::thread::spawn(move || {
            let mut freq = HashMap::new();
            // `recv()` returns Err when the sender has hung up
            // we use this to indicate that the sender has finished producing data
            while let Ok(map) = consumer.recv() {
                // lock the frequency map for every received message
                for (c, count) in map.into_iter() {
                    *freq.entry(c).or_default() += count;
                }
            }

            result_tx.send(freq).unwrap();
        });

        let nchunks = input.len() / worker_count;
        input
            .chunks(if nchunks == 0 {
                // At least 1 chunk: `chunks(0)` panics, which can happen if the input is empty
                // or if `input.len()` < worker count.
                //
                // In both cases we want 1:
                //   * in the case that the input is empty an empty iterator is produced
                //   * in the case of `input.len()` <= `worker_count` we want to create chunks of size 1,
                //     which will distribute each of the elements to a different worker thread
                input.len().max(1)
            } else {
                nchunks
            })
            .map(|chunk| {
                let chunk = chunk
                    .to_vec()
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>();
                let producer_clone = producer.clone();
                std::thread::spawn(move || {
                    producer_clone
                        .send(chunk.into_iter().fold(
                            Default::default(),
                            move |mut counts, string| {
                                for c in string
                                    .chars()
                                    .filter(|c| c.is_alphabetic())
                                    .flat_map(|c| c.to_lowercase())
                                {
                                    *counts.entry(c).or_default() += 1;
                                }
                                counts
                            },
                        ))
                        .unwrap();
                })
            })
            // this collect is critical: without collecting into a vec, the computation
            // will not run in parallel.
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|thread| thread.join().unwrap());

        // Hang up the producer side of the channel, otherwise the consumer will hangforever.
        //
        // We've joined all the producer threads at this point, so we know that data
        // has made it to the consumer.
        //
        // All producer threads have either finished successfully, otherwise the unwrap
        // of each thread.join() result would have already panicked, halting the program.
        drop(producer);

        // Wait for the consumer to finish processing
        consumer_thread.join().unwrap();

        result_rx.recv().unwrap()
    }
}

mod cb {
    use crossbeam_utils::thread;
    use std::{collections::HashMap, sync::mpsc};

    // This implementation is the same as the `raw` implementation, except that it
    // uses scoped threads to avoid copying the input.
    pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
        let (producer, consumer) = mpsc::channel::<HashMap<_, usize>>();
        let (result_tx, result_rx) = mpsc::sync_channel(1);

        thread::scope(move |scope| {
            let consumer_thread = scope.spawn(move |_| {
                let mut freq = HashMap::new();
                while let Ok(map) = consumer.recv() {
                    for (c, count) in map.into_iter() {
                        *freq.entry(c).or_default() += count;
                    }
                }

                result_tx.send(freq).unwrap();
            });

            let nchunks = input.len() / worker_count;
            input
                .chunks(if nchunks == 0 {
                    input.len().max(1)
                } else {
                    nchunks
                })
                .map(|chunk| {
                    let producer_clone = producer.clone();

                    scope.spawn(move |_| {
                        producer_clone
                            .send(chunk.iter().fold(
                                Default::default(),
                                move |mut counts, string| {
                                    for c in string
                                        .chars()
                                        .filter(|c| c.is_alphabetic())
                                        .flat_map(|c| c.to_lowercase())
                                    {
                                        *counts.entry(c).or_default() += 1;
                                    }
                                    counts
                                },
                            ))
                            .unwrap();
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|thread| thread.join().unwrap());

            drop(producer);

            consumer_thread.join().unwrap();
            result_rx.recv().unwrap()
        })
        .unwrap()
    }
}

mod managed {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use std::collections::HashMap;

    pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(worker_count)
            .build()
            .unwrap()
            .scope(|_scope| {
                input
                    .into_par_iter()
                    .fold(HashMap::new, |mut counts, &c| {
                        for lower_char in c
                            .chars()
                            .filter(|c| c.is_alphabetic())
                            .flat_map(|c| c.to_lowercase())
                        {
                            *counts.entry(lower_char).or_default() += 1;
                        }
                        counts
                    })
                    .reduce(Default::default, |mut counts, map| {
                        for (c, count) in map.into_iter() {
                            *counts.entry(c).or_default() += count;
                        }
                        counts
                    })
            })
    }
}

pub use raw::frequency;
