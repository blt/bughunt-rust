//! Tests for `std::collections::VecDeque`
use arbitrary::*;

/// A `VecDeque<T>` model
///
/// This type mimics the semantics of `VecDeque<T>` while being 'obviously
/// correct' enough to serve as a `QuickCheck` model. What is a VecDeque? Well,
/// it's a queue that supports efficient push/pop from both the back and front
/// of the queue. Efficiency is of no interest to us and we'll just abuse a Vec,
/// much like with [`PropHashMap`].
pub struct PropVecDeque<T> {
    data: Vec<T>,
}

impl<T> Default for PropVecDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PropVecDeque<T> {
    /// Construct a new `PropVecDeque<T>`
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Push a value onto the back of `PropVecDeque<T>`
    ///
    /// This is like to [`std::collections::VecDeque::push_back`]
    pub fn push_back(&mut self, value: T) {
        self.data.push(value)
    }

    /// Pop a value from the back of `PropVecDeque<T>`, if one exists
    ///
    /// This is like to [`std::collections::VecDeque::pop_back`]
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Push a value to the front of `PropVecDeque<T>`.
    ///
    /// This is like to [`std::collections::VecDeque::push_front`]
    pub fn push_front(&mut self, value: T) {
        self.data.insert(0, value);
    }

    /// Insert a value at the given index into `PropVecDeque<T>`.
    ///
    /// This is like to [`std::collections::VecDeque::insert`]
    pub fn insert(&mut self, index: usize, value: T) {
        self.data.insert(index, value);
    }

    /// Remove and return a value from the given index `PropVecDeque<T>`.
    ///
    /// This is like to [`std::collections::VecDeque::remove`]
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    /// Pop a value from the front of `PropVecDeque<T>`, if one exists
    ///
    /// This is like to [`std::collections::VecDeque::pop_front`]
    pub fn pop_front(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let val = self.data.remove(0);
            Some(val)
        }
    }

    /// Clear all contents of `PropVecDeque`
    ///
    /// This is like to [`std::collections::VecDeque::clear`]
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Provide a reference to the front element, if one exists
    ///
    /// This is like to [`std::collections::VecDeque::front`]
    pub fn front(&mut self) -> Option<&T> {
        if self.data.is_empty() {
            None
        } else {
            let val = &self.data[0];
            Some(val)
        }
    }

    /// Provide a reference to the back element, if one exists
    ///
    /// This is like to [`std::collections::VecDeque::back`]
    pub fn back(&mut self) -> Option<&T> {
        if self.data.is_empty() {
            None
        } else {
            let len = self.data.len();
            let val = &self.data[len - 1];
            Some(val)
        }
    }

    /// Replace an element at the given index with the back element, return the
    /// replaced element
    ///
    /// This is like to [`std::collections::VecDeque::swap_remove_back`]
    pub fn swap_remove_back(&mut self, index: usize) -> Option<T> {
        if self.data.is_empty() {
            None
        } else if self.data.len() == 1 {
            self.pop_back()
        } else if index < self.data.len() {
            let back = self.data.len() - 1;
            self.data.swap(index, back);
            self.pop_back()
        } else {
            None
        }
    }

    /// Return the number of elements in `PropVecDeque<T>`
    ///
    /// This is like to [`std::collections::VecDeque::len`]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return true if the PropVecDeque is empty, else false
    ///
    /// This is like to [`std::collections::VecDeque::is_empty`]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// The `Op<T>` defines the set of operations that are available against
/// `VecDeque<K, V>` and `PropVecDeque<T>`. Some map directly to functions
/// available on the types, others require a more elaborate interpretation step.
#[derive(Clone, EnumCount, Debug)]
pub enum Op<T> {
    /// This operation triggers `std::collections::VecDeque::shrink_to_fit`
    ShrinkToFit,
    /// This operation triggers `std::collections::VecDeque::clear`
    Clear,
    /// This operation triggers `std::collections::VecDeque::push_back`
    PushBack(T),
    /// This operation triggers `std::collections::VecDeque::pop_back`
    PopBack,
    /// This operation triggers `std::collections::VecDeque::push_front`
    PushFront(T),
    /// This operation triggers `std::collections::VecDeque::pop_front`
    PopFront,
    /// This operation triggers `std::collections::VecDeque::insert`
    Insert(usize, T),
    /// This operation triggers `std::collections::VecDeque::remove`
    Remove(usize),
    /// This operation triggers `std::collections::VecDeque::swap_remove_back`
    SwapRemoveBack(usize),
}

impl<T> Arbitrary for Op<T>
where
    T: Clone + Send + Arbitrary,
{
    fn arbitrary<U>(u: &mut U) -> Result<Self, U::Error>
    where
        U: Unstructured + ?Sized,
    {
        let total_enum_fields = OP_COUNT as u8;
        let variant: u8 = Arbitrary::arbitrary(u)?;
        let op = match variant % total_enum_fields {
            0 => {
                let t: T = Arbitrary::arbitrary(u)?;
                Op::PushBack(t)
            }
            1 => Op::PopBack,
            2 => {
                let t: T = Arbitrary::arbitrary(u)?;
                Op::PushFront(t)
            }
            3 => Op::PopFront,
            4 => Op::Clear,
            5 => Op::ShrinkToFit,
            6 => {
                let idx: usize = Arbitrary::arbitrary(u)?;
                let t: T = Arbitrary::arbitrary(u)?;
                Op::Insert(idx, t)
            }
            7 => {
                let idx: usize = Arbitrary::arbitrary(u)?;
                Op::Remove(idx)
            }
            8 => {
                let idx: usize = Arbitrary::arbitrary(u)?;
                Op::SwapRemoveBack(idx)
            }
            _ => unreachable!(),
        };
        Ok(op)
    }
}
