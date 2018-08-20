use std::hash::Hash;
use std::mem::swap;

pub struct PropHashMap<K, V>
where
    K: Eq + Hash,
{
    data: Vec<(K, V)>,
}

impl<K, V> Default for PropHashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> PropHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        if let Some(idx) = self.data.iter().position(|probe| probe.0 == *k) {
            Some(&(self.data[idx].1))
        } else {
            None
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(idx) = self.data.iter().position(|probe| probe.0 == k) {
            // TODO(blt) This violates the semantics of HashMap a
            // little. Specifically, we change out the key. HashMap documents
            // that it _does not_ update the key on every insert, important for
            // types that are Eq without actually being identical.
            //
            // TODO(blt) This is an important enough constraint that there
            // should be a property around this.
            let mut pair = (k, v);
            swap(&mut pair, &mut self.data[idx]);
            Some(pair.1)
        } else {
            self.data.push((k, v));
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    enum Op<K, V> {
        Insert { k: K, v: V },
        Get { k: K },
    }

    impl<K: 'static, V: 'static> Arbitrary for Op<K, V>
    where
        K: Clone + Send + Arbitrary,
        V: Clone + Send + Arbitrary,
    {
        fn arbitrary<G>(g: &mut G) -> Op<K, V>
        where
            G: Gen,
        {
            let i: usize = g.gen_range(0, 100);
            let k: K = Arbitrary::arbitrary(g);
            let v: V = Arbitrary::arbitrary(g);
            match i {
                0..=50 => Op::Insert { k, v },
                _ => Op::Get { k },
            }
        }
    }

    #[test]
    fn oprun() {
        fn inner(capacity: usize, ops: Vec<Op<u16, u16>>) -> TestResult {
            let mut model: PropHashMap<u16, u16> = PropHashMap::new();
            let mut sut: HashMap<u16, u16> = HashMap::with_capacity(capacity);
            for op in &ops {
                match op {
                    Op::Get { k } => {
                        let model_res = model.get(k);
                        let sut_res = sut.get(&k);
                        assert_eq!(model_res, sut_res);
                    }
                    Op::Insert { k, v } => {
                        let model_res = model.insert(*k, *v);
                        let sut_res = sut.insert(*k, *v);
                        assert_eq!(model_res, sut_res);
                    }
                }
            }
            TestResult::passed()
        }
        QuickCheck::new().quickcheck(inner as fn(usize, Vec<Op<u16, u16>>) -> TestResult)
    }
}
