use std::{collections::VecDeque, mem::MaybeUninit};

pub struct Slab<T> {
    storage: Box<[MaybeUninit<T>]>,
    free_list: VecDeque<Index>,
}

pub struct Index(usize);

impl<T> Slab<T> {
    pub fn new(len: usize) -> Self {
        let storage: Box<[MaybeUninit<T>]> = Box::new_uninit_slice(len);
        let free_list = (0..len).rev().map(Index).collect();

        Self { storage, free_list }
    }

    pub fn allocate(&mut self, value: T) -> Option<Index> {
        let index = self.free_list.pop_front()?;

        unsafe { self.storage.get_unchecked_mut(index.0).write(value) };

        Some(index)
    }

    pub fn get(&self, index: &Index) -> &T {
        unsafe { self.storage.get_unchecked(index.0).assume_init_ref() }
    }

    pub fn get_mut(&mut self, index: Index) -> &mut T {
        unsafe { self.storage.get_unchecked_mut(index.0).assume_init_mut() }
    }

    pub fn deallocate(&mut self, index: Index) {
        unsafe {
            self.storage.get_unchecked_mut(index.0).assume_init_drop();
        }

        self.free_list.push_back(index);
    }
}

impl<T> Drop for Slab<T> {
    fn drop(&mut self) {
        let mut free_slots = vec![false; self.storage.len()];

        for index in &self.free_list {
            unsafe {
                *free_slots.get_unchecked_mut(index.0) = true;
            }
        }
        
        for (i, free) in free_slots.iter().enumerate() {
            if !free {
                unsafe {
                    self.storage.get_unchecked_mut(i).assume_init_drop();
                }
            }
        }
    }
}

fn main() {
    let mut slab = Slab::<String>::new(4);

    let a = slab.allocate("hello".into()).unwrap();
    let b = slab.allocate("world".into()).unwrap();

    println!("{}", slab.get(&a));
    println!("{}", slab.get(&b));

    slab.deallocate(a);

    let c = slab.allocate("reused with semicolon".into()).unwrap();
    println!("{}", slab.get(&c));
}
