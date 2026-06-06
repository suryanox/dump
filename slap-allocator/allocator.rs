
use criterion::{criterion_group, criterion_main, Criterion};
use std::{hint::black_box, mem::MaybeUninit};

const N: usize = 50_000;

#[derive(Clone, Copy)]
struct Connection {
    id: u64,
    state: [u8; 128],
}


fn bench_box(c: &mut Criterion) {
    c.bench_function("box_alloc_free", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(N);

            for i in 0..N {
                v.push(Box::new(Connection {
                    id: black_box(i as u64),
                    state: [0; 128],
                }));
            }

            drop(v);
        })
    });
}


struct Slab<T> {
    storage: Box<[MaybeUninit<T>]>,
    free: Vec<usize>,
}

impl<T> Slab<T> {
    fn new(cap: usize) -> Self {
        let storage = std::iter::repeat_with(MaybeUninit::uninit)
            .take(cap)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            storage,
            free: (0..cap).rev().collect(),
        }
    }

    fn allocate(&mut self, value: T) -> usize {
        let idx = self.free.pop().unwrap();
        self.storage[idx].write(value);
        idx
    }

    fn deallocate(&mut self, idx: usize) {
        unsafe {
            self.storage[idx].assume_init_drop();
        }
        self.free.push(idx);
    }
}

fn bench_slab(c: &mut Criterion) {
    c.bench_function("slab_alloc_free", |b| {
        b.iter(|| {
            let mut slab = Slab::<Connection>::new(N);
            let mut handles = Vec::with_capacity(N);

            for i in 0..N {
                handles.push(slab.allocate(Connection {
                    id: black_box(i as u64),
                    state: [0; 128],
                }));
            }

            for h in handles {
                slab.deallocate(h);
            }
        })
    });
}


criterion_group!(benches, bench_box, bench_slab);
criterion_main!(benches);
