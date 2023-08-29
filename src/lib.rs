use std::{
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct Looper<T> {
    arr: Box<[T]>,
    index: Arc<AtomicUsize>,
}

impl<T> Looper<T> {
    pub fn set(&mut self, value: T) {
        let index = self.index.fetch_add(1, Ordering::AcqRel);
        self.arr[index % self.arr.len()] = value;
    }

    pub fn get(&self) -> &T {
        let index = self.index.fetch_add(1, Ordering::AcqRel);
        &self.arr[index % self.arr.len()]
    }

    pub fn get_mut(&mut self) -> &mut T {
        let len = self.arr.len();
        let index = self.index.fetch_add(1, Ordering::AcqRel);
        &mut self.arr[index % len]
    }
}

impl<T> From<Vec<T>> for Looper<T> {
    fn from(value: Vec<T>) -> Self {
        Looper {
            arr: value.into_boxed_slice(),
            index: Arc::new(0.into()),
        }
    }
}

impl<T> From<Box<[T]>> for Looper<T> {
    fn from(value: Box<[T]>) -> Self {
        Looper {
            arr: value,
            index: Arc::new(0.into()),
        }
    }
}

impl<T> Deref for Looper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for Looper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let mut looper: Looper<i32> = vec![1, 2, 3, 4].into();

        assert_eq!(*looper, 1);
        assert_eq!(*looper, 2);
        assert_eq!(*looper, 3);
        assert_eq!(*looper, 4);
        assert_eq!(*looper, 1);

        *looper = 102;
        *looper += 100;

        assert_eq!(*looper, 4);
        assert_eq!(*looper, 1);
        assert_eq!(*looper, 102);
        assert_eq!(*looper, 103);
    }
}
