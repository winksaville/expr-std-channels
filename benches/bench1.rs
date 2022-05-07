use criterion::{criterion_group, criterion_main, Criterion};

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn volatile_write_benchmark(c: &mut Criterion) {
    let mut v = 123i32;
    let pv = &mut v as *mut i32;

    c.bench_function("volatile_write", |b| {
        b.iter(|| unsafe {
            std::ptr::write_volatile(pv, v);
        });
    });
}

pub fn fn_to_fn_benchmark(c: &mut Criterion) {
    {
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        c.bench_function("fn_to_fn", |b| {
            b.iter(|| {
                tx.send(1).unwrap();
                let v = rx.recv().unwrap();
                assert!(v == 1);
            });
        });
    }
}

criterion_group!(benches, volatile_write_benchmark, fn_to_fn_benchmark);
criterion_main!(benches);
