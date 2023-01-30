use mytrie::Trie;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use std::ops::Range;
use std::time::Instant;

fn generate_string(length: usize, charset: &[char]) -> String {
    let mut rng = &mut rand::thread_rng();

    charset.choose_multiple(&mut rng, length).collect()
}

fn generate_samples(count: usize, length: Range<usize>, charset: &[char]) -> Vec<String> {
    let between = Uniform::from(length);
    let mut rng = rand::thread_rng();

    (0..count)
        .map(|_| generate_string(between.sample(&mut rng), charset))
        .collect()
}

const COUNT: usize = 10_000;

#[test]
fn bench() {
    let charset: Vec<char> = "abcdefghijklnmopqrstuvwxyz".chars().collect();
    let mut samples = generate_samples(COUNT, 30..60, &charset);
    samples.sort();

    let begin = Instant::now();
    let mut trie = Trie::from(&samples);
    eprintln!("inserting durated {} ms", begin.elapsed().as_millis());

    let begin = Instant::now();
    let mut retrieved_samples: Vec<String> = trie.iter_content("").collect();
    eprintln!("retrieving durated {} ms", begin.elapsed().as_millis());

    retrieved_samples.sort();
    assert_eq!(COUNT, retrieved_samples.len());
    assert_eq!(&samples, &retrieved_samples);

    let begin = Instant::now();
    for item in samples.iter() {
        trie.remove(item).unwrap();
    }
    eprintln!("removing durated {} ms", begin.elapsed().as_millis());

    assert!(trie.is_empty())
}
