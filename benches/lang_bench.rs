use std::{collections::HashMap, sync::RwLock};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ytml_lang::{html::ast_to_html, tokens::Tag};

fn criterion_benchmark(c: &mut Criterion) {
    let doc = RwLock::new(vec![Tag {
        name: String::from("html"),
        inner: Vec::new(),
        attributes: HashMap::new(),
    }]);
    c.bench_function("AST to html", |b| {
        b.iter(|| {
            let a = doc.read().unwrap();
            ast_to_html(black_box(a.to_vec()), black_box(0));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
