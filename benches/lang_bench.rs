use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ytml_lang::{html::ast_to_html, tokens::Tag};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("AST to html", |b| {
        b.iter(|| {
            let doc = vec![Tag {
                name: String::from("html"),
                inner: Vec::new(),
                attributes: HashMap::new(),
            }];
            ast_to_html(black_box(doc), black_box(0));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
