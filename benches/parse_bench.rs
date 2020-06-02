use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gtm::parse::*;

fn criterion_benchmark(c: &mut Criterion) {
    let message = "[ver:1,total:4037]";
    assert!(parse_commit_note(message).is_ok());
    c.bench_function("parse empty commit note", |b| {
        b.iter(|| parse_commit_note(black_box(message)))
    });

    let message = "[ver:1,total:4037]
 comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m
 closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
 text/src/char.ts:90,1585918800:90,r
 demo/demo.ts:60,1585918800:60,r
 state/src/selection.ts:40,1585918800:40,r
 highlight/src/highlight.ts:30,1585918800:30,r
 lang-javascript/src/javascript.ts:30,1585918800:30,r
 node_modules/w3c-keyname/index.d.ts:20,1585922400:20,r
 CHANGELOG.md:20,1585918800:20,r";
    assert!(parse_commit_note(message).is_ok());
    c.bench_function("parse commit note", |b| {
        b.iter(|| parse_commit_note(black_box(message)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
