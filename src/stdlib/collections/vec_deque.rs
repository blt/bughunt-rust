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
    pub fn push_back(&mut self, value: T) -> () {
        self.data.push(value)
    }

    /// Pop a value from the back of `PropVecDeque<T>`, if one exists
    ///
    /// This is like to [`std::collections::VecDeque::pop_back`]
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop()
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
#[derive(Clone, Debug)]
pub enum Op<T> {
    /// This operation triggers `std::collections::VecDeque::push_back`
    PushBack(T),
    /// This operation triggers `std::collections::VecDeque::pop_back`
    PopBack,
}

impl<T> Arbitrary for Op<T>
where
    T: Clone + Send + Arbitrary,
{
    fn arbitrary<U>(u: &mut U) -> Result<Self, U::Error>
    where
        U: Unstructured + ?Sized,
    {
        // ================ WARNING ================
        //
        // `total_enum_fields` is a goofy annoyance but it should match
        // _exactly_ the number of fields available in `Op<T>`. If it
        // does not then we'll fail to generate `Op` variants for use in our
        // QC tests.
        let total_enum_fields = 2;
        let variant: u8 = Arbitrary::arbitrary(u)?;
        let op = match variant % total_enum_fields {
            0 => {
                let t: T = Arbitrary::arbitrary(u)?;
                Op::PushBack(t)
            }
            1 => Op::PopBack,
            _ => unreachable!(),
        };
        Ok(op)
    }
}
