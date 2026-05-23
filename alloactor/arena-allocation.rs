use std::cell::{Cell, UnsafeCell};
use std::mem;

struct Chunk {
    data: UnsafeCell<Vec<u8>>,
    used: Cell<usize>,
}

impl Chunk {
    fn new(capacity: usize) -> Self {
        Chunk {
            data: UnsafeCell::new(vec![0u8; capacity]),
            used: Cell::new(0),
        }
    }

    fn capacity(&self) -> usize {
        unsafe { (*self.data.get()).len() }
    }

    fn try_alloc(&self, size: usize, align: usize) -> Option<*mut u8> {
        let base = unsafe { (*self.data.get()).as_mut_ptr() };
        let cur = self.used.get();
        let aligned = (cur + align - 1) & !(align - 1);
        if aligned + size > self.capacity() {
            return None;
        }
        self.used.set(aligned + size);
        Some(unsafe { base.add(aligned) })
    }
}

pub struct Arena {
    chunks: UnsafeCell<Vec<Chunk>>,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            chunks: UnsafeCell::new(vec![Chunk::new(4096)]),
        }
    }

    pub fn alloc<T>(&self, value: T) -> &T {
        let size  = mem::size_of::<T>();
        let align = mem::align_of::<T>();

        let ptr = self.alloc_raw(size, align) as *mut T;
        unsafe {
            ptr.write(value);
            &*ptr
        }
    }

    fn alloc_raw(&self, size: usize, align: usize) -> *mut u8 {
        let chunks = unsafe { &mut *self.chunks.get() };
        if let Some(ptr) = chunks.last().unwrap().try_alloc(size, align) {
            return ptr;
        }
        let new_cap = (chunks.last().unwrap().capacity() * 2).max(size + align);
        chunks.push(Chunk::new(new_cap));
        chunks.last().unwrap().try_alloc(size, align).unwrap()
    }

    pub fn reset(&mut self) {
        for chunk in unsafe { &mut *self.chunks.get() } {
            chunk.used.set(0);
        }
    }
}

#[derive(Debug)]
struct ArenaAstNode<'a> {
    value: u64,
    left:  Option<&'a ArenaAstNode<'a>>,
    right: Option<&'a ArenaAstNode<'a>>,
}

fn build_tree_arena<'a>(arena: &'a Arena, depth: u32) -> &'a ArenaAstNode<'a> {
    if depth == 0 {
        return arena.alloc(ArenaAstNode { value: 0, left: None, right: None });
    }
    let left  = arena.alloc(ArenaAstNode { value: depth as u64 - 1, left: None, right: None });
    let right = arena.alloc(ArenaAstNode { value: depth as u64 - 1, left: None, right: None });
    arena.alloc(ArenaAstNode {
        value: depth as u64,
        left:  Some(left),
        right: Some(right),
    })
}

fn main() {
    let mut arena = Arena::new();
    {
        let root = build_tree_arena(&arena, 4);
        println!("Arena tree root value: {}", root.value);
    }
    arena.reset();
    println!("Arena reset — backing memory reused, zero calls to the OS allocator.");
}
