use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cmd_vault::models::CommandItem;

fn create_test_commands(num_commands: usize) -> Vec<CommandItem> {
    (0..num_commands)
        .map(|i| CommandItem {
            name: format!("command-{}", i),
            command: format!("echo 'Command number {}'", i),
            desc: format!("Description for command {}", i),
            created_at: i as u64,
        })
        .collect()
}

fn search_commands<'a>(commands: &'a [CommandItem], query: &str) -> Vec<&'a CommandItem> {
    let query_lower = query.to_lowercase();
    commands
        .iter()
        .filter(|cmd| {
            cmd.name.to_lowercase().contains(&query_lower)
                || cmd.command.to_lowercase().contains(&query_lower)
                || cmd.desc.to_lowercase().contains(&query_lower)
        })
        .collect()
}

fn search_benchmark(c: &mut Criterion) {
    let commands_100 = create_test_commands(100);
    let commands_1000 = create_test_commands(1000);
    let commands_10000 = create_test_commands(10000);

    let mut group = c.benchmark_group("search_performance");

    group.bench_function("search_100_commands", |b| {
        b.iter(|| search_commands(&commands_100, black_box("command")))
    });

    group.bench_function("search_1000_commands", |b| {
        b.iter(|| search_commands(&commands_1000, black_box("command")))
    });

    group.bench_function("search_10000_commands", |b| {
        b.iter(|| search_commands(&commands_10000, black_box("command")))
    });

    group.finish();
}

fn serialization_benchmark(c: &mut Criterion) {
    let commands = create_test_commands(1000);

    let mut group = c.benchmark_group("serialization");

    group.bench_function("serialize_1000_commands", |b| {
        b.iter(|| serde_json::to_string(black_box(&commands)).unwrap())
    });

    let json = serde_json::to_string(&commands).unwrap();
    group.bench_function("deserialize_1000_commands", |b| {
        b.iter(|| serde_json::from_str::<Vec<CommandItem>>(black_box(&json)).unwrap())
    });

    group.finish();
}

criterion_group!(benches, search_benchmark, serialization_benchmark);
criterion_main!(benches);
