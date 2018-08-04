use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Default)]
pub struct IndexTree<N, I>
where
    N: Eq + Hash,
{
    named: HashMap<N, usize>, // Needs to be renamed to something accurate
    roots: HashSet<usize>,
    items: Vec<I>,
    parents: Vec<Option<usize>>,
}

impl<N, I> IndexTree<N, I>
where
    N: Eq + Hash,
{
    pub fn new() -> Self {
        let named = HashMap::new();
        let roots = HashSet::new();
        let items = vec![];
        let parents = vec![];
        IndexTree {
            named,
            roots,
            items,
            parents,
        }
    }

    pub fn push(&mut self, name: Option<N>, item: I) {
        self.items.push(item);
        self.parents.push(None);
        let idx = self.items.len() - 1;
        if let Some(name) = name {
            self.named.insert(name, idx); // Need to handle name collisions
        };
        for i in self.roots.drain() {
            self.parents[i] = Some(idx);
        }
        self.roots.insert(idx);
    }

    pub fn append(&mut self, other: &mut Self) {
        let offset = self.items.len();
        self.items.extend(other.items.drain(..));
        self.roots.extend(other.roots.drain().map(|r| r + offset));
        self.named
            .extend(other.named.drain().map(|(k, v)| (k, v + offset))); // Need to handle name collisions
        self.parents
            .extend(other.parents.drain(..).map(|p| p.map(|i| i + offset)));
    }

    pub fn map<X>(self, f: impl FnMut(I) -> X) -> IndexTree<N, X> {
        let named = self.named;
        let roots = self.roots;
        let items = self.items;
        let parents = self.parents;
        let items = items.into_iter().map(f).collect();
        IndexTree {
            named,
            roots,
            items,
            parents,
        }
    }

    pub fn get(&self, name: &N) -> Option<(Option<usize>, &I)> {
        let idx = *(self.named.get(name)?);
        Some((self.parents[idx], &self.items[idx]))
    }

    pub fn get_iter(&self, name: &N) -> TreeIter<I> {
        TreeIter {
            items: &self.items,
            parents: &self.parents,
            idx: self.named.get(name).cloned(),
        }
    }
}

pub struct TreeIter<'a, I: 'a> {
    items: &'a Vec<I>,
    parents: &'a Vec<Option<usize>>,
    idx: Option<usize>,
}

impl<'a, I> Iterator for TreeIter<'a, I> {
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = match self.idx {
            Some(i) => i,
            None => return None,
        };
        let item = self.items.get(idx);
        self.idx = self.parents[idx];
        item
    }
}
