use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use text_to_png::*;

const TEST_STR_LEN: usize = 30;

#[allow(dead_code)]
fn random_string() -> String {
    let mut rng = thread_rng();

    String::from_utf8(
        std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(TEST_STR_LEN)
            .collect(),
    )
    .expect("This should be valid characters")
}

fn default_render_benchmark(c: &mut Criterion) {
    let tr = TextRenderer::default();

    let repeat = 10;

    let to_render = (0..repeat).map(|_| random_string()).collect::<Vec<_>>();

    c.bench_function("Default Font 100px ", |b| {
        b.iter(|| {
            for s in to_render.iter() {
                if let Err(e) = black_box(tr.render_text_to_png_data(s, 100, 0))
                {
                    panic!("Failed to render: {} - {}", s, e);
                }
            }
        })
    });
}

criterion_group!(benches, default_render_benchmark,);
criterion_main!(benches);
