use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_simulation(c: &mut Criterion) {
    const GRID_X: usize = 200;
    const GRID_Y: usize = 200;
    const STEPS: usize = 1000;

    let mut s = rustymold::Simulation::new(GRID_X, GRID_Y, 16);

    let mut group = c.benchmark_group("benchmark of Simulation.update()");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(60));
    group.bench_function(
        format!("{STEPS} update()s on {GRID_X}x{GRID_Y} grid"),
        |b| {
            b.iter(|| {
                // seed the RNG
                fastrand::seed(4);

                // reset simulation state
                s.clear();

                // create some molds
                for dx in (10..GRID_X).step_by(20) {
                    for dy in (10..GRID_Y).step_by(20) {
                        s.generate_mold(dx, dy);
                    }
                }

                // perform update steps
                for _ in 0..STEPS {
                    s.update()
                }
            })
        },
    );
}

criterion_group!(benches, bench_simulation);
criterion_main!(benches);
