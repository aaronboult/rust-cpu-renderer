use criterion::{black_box, criterion_group, criterion_main, Criterion};

use simulation_engine::simulator;
use simulator::Vector3D;
use simulator::renderer::{Renderer, RenderMode, Transform};

fn criterion_benchmark(c: &mut Criterion) {
    let r = Renderer::new(RenderMode::R3D);
    let t = Transform::new();
    c.bench_function(
        "calculate_3d_projection 1920x1080",
        |b| b.iter(
            || r.calculate_3d_projection(black_box(&t), black_box(&Vector3D::ONE), black_box((1920, 1080)))
        )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);