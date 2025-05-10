use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn generate_mock_records(count: usize) -> Vec<(String, String, i64)> {
    (0..count)
        .map(|i| {
            (
                format!("user{}", i),
                format!("2023-0{}-{}", (i % 12) + 1, (i % 28) + 1),
                i as i64,
            )
        })
        .collect()
}

// Rust-style implementation: efficient memory usage, leveraging stack when possible
fn process_records_rust_style(records: &[(String, String, i64)]) -> Vec<String> {
    records
        .iter()
        .filter(|(_, date, _)| date.starts_with("2023-05"))
        .map(|(name, _, value)| {
            let mut result = String::with_capacity(name.len() + 20);
            result.push_str(name);
            result.push_str(": ");
            result.push_str(&value.to_string());
            result
        })
        .collect()
}

// Python-style implementation: more dynamic, heap-heavy approach
fn process_records_python_style(records: &[(String, String, i64)]) -> Vec<String> {
    let mut results = Vec::new();
    
    for (name, date, value) in records {
        if date.starts_with("2023-05") {
            let result = name.clone() + ": " + &value.to_string();
            results.push(result);
        }
    }
    
    results
}

// TypeScript-style implementation: object-oriented approach with more allocations
fn process_records_typescript_style(records: &[(String, String, i64)]) -> Vec<String> {
    let filtered = records
        .iter()
        .filter(|(_, date, _)| date.starts_with("2023-05"))
        .collect::<Vec<_>>();
    
    let mut results = Vec::new();
    for &(name, _, value) in filtered.iter() {
        let result = format!("{}: {}", name, value);
        results.push(result);
    }
    
    results
}

fn bench_db_operations(c: &mut Criterion) {
    let records = generate_mock_records(1000000);
    
    let mut group = c.benchmark_group("Database Operations");
    
    group.bench_function("rust_implementation", |b| {
        b.iter(|| process_records_rust_style(black_box(&records)))
    });
    
    group.bench_function("python_style_implementation", |b| {
        b.iter(|| process_records_python_style(black_box(&records)))
    });
    
    group.bench_function("typescript_style_implementation", |b| {
        b.iter(|| process_records_typescript_style(black_box(&records)))
    });
    
    group.finish();
}

criterion_group!(benches, bench_db_operations);
criterion_main!(benches);
