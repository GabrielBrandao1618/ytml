use std::{collections::HashMap, sync::RwLock};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ytml_lang::{html::ast_to_html, tokens::Tag, ytml::parse_ytml_file};

fn criterion_benchmark(c: &mut Criterion) {
    let doc = RwLock::new(vec![Tag {
        name: String::from("html"),
        inner: Vec::new(),
        attributes: HashMap::new(),
    }]);
    c.bench_function("AST to html", |b| {
        b.iter(|| {
            let doc = doc.read().unwrap();
            ast_to_html(black_box(doc.to_vec()), black_box(0));
        })
    });
    let raw_ytml = "html {body {}}";
    c.bench_function("ytml to ast", |b| {
        b.iter(|| {
            parse_ytml_file(black_box(raw_ytml));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
