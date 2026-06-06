use std::mem::MaybeUninit;

pub struct Slab<T, const N: usize> {
    storage: [MaybeUninit<T>; N],
    free_list: Vec<usize>,
}

impl<T, const N: usize> Slab<T, N> {
    pub fn new() -> Self {
        let storage = std::array::from_fn(|_| MaybeUninit::uninit());

        let free_list = (0..N).rev().collect();

        Self {
            storage,
            free_list,
        }
    }

    pub fn allocate(&mut self, value: T) -> Option<usize> {
        let index = self.free_list.pop()?;

        self.storage[index].write(value);

        Some(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        Some(unsafe { self.storage[index].assume_init_ref() })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        Some(unsafe { self.storage[index].assume_init_mut() })
    }

    pub fn deallocate(&mut self, index: usize) {
        unsafe {
            self.storage[index].assume_init_drop();
        }

        self.free_list.push(index);
    }
}



fn main() {
    let mut slab = Slab::<String, 4>::new();

    let a = slab.allocate("hello".into()).unwrap();
    let b = slab.allocate("world".into()).unwrap();

    println!("{}", slab.get(a).unwrap());
    println!("{}", slab.get(b).unwrap());

    slab.deallocate(a);

    let c = slab.allocate("reused with semicolon".into()).unwrap();

    println!("{}", slab.get(c).unwrap());
}
