use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn disemvowel_pattern(s: &str) -> String {
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

fn disemvowel_vec(s: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    s.chars()
        .filter(|c| !vowels.contains(&c.to_ascii_lowercase()))
        .collect()
}

fn disemvowel_replace(s: &str) -> String {
    s.replace(&['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'][..], "")
}

fn bench_disemvowels(c: &mut Criterion) {
    let test_string = "The quick brown fox jumps over the lazy dog. This string is just a sample to test disemvowel performance. Let's add some more vowels here: aeiouAEIOU".repeat(10000);

    let mut group = c.benchmark_group("Disemvowel Operations");

    group.bench_function("disemvowel - pattern match (&c)", |b| {
        b.iter(|| disemvowel_pattern(black_box(&test_string)))
    });

    group.bench_function("disemvowel - vec + contains", |b| {
        b.iter(|| disemvowel_vec(black_box(&test_string)))
    });

    group.bench_function("disemvowel - replace", |b| {
        b.iter(|| disemvowel_replace(black_box(&test_string)))
    });
}

criterion_group!(benches, bench_disemvowels);
criterion_main!(benches);
