use std::collections::{hash_map, HashMap, HashSet};
use std::ops::{Index, IndexMut, Not};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Variable(pub usize);

#[derive(PartialEq, Clone, Copy)]
pub enum Sign {
    Pos,
    Neg,
}

#[derive(Clone, Copy)]
pub struct Literal(pub Variable, pub Sign);

pub struct Disjunction(HashMap<Variable, Sign>);

pub type DisjunctionIter<'a> = hash_map::Iter<'a, Variable, Sign>;

impl Disjunction {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> DisjunctionIter {
        self.0.iter()
    }
}

impl Index<Variable> for Disjunction {
    type Output = Sign;

    fn index(&self, k: Variable) -> &Sign {
        self.0.index(&k)
    }
}

pub struct Problem {
    pub disjunctions: Vec<Disjunction>,
}

impl Problem {
    pub fn vars(&self) -> HashSet<Variable> {
        self.disjunctions
            .iter()
            .map(|&Disjunction(ref hm)| hm.keys().map(|&k| k))
            .flatten()
            .collect()
    }
}

impl Not for Sign {
    type Output = Sign;

    fn not(self) -> Sign {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        }
    }
}

pub struct SignMap<V> {
    pos_val: V,
    neg_val: V,
}

impl<V> SignMap<V> {
    pub fn new<F: Fn(Sign) -> V>(init: F) -> SignMap<V> {
        SignMap{
            pos_val: init(Sign::Pos),
            neg_val: init(Sign::Neg),
        }
    }
}

impl<V> Index<Sign> for SignMap<V> {
    type Output = V;

    fn index(&self, k: Sign) -> &V {
        match k {
            Sign::Pos => &self.pos_val,
            Sign::Neg => &self.neg_val,
        }
    }
}

impl<V> IndexMut<Sign> for SignMap<V> {
    fn index_mut(&mut self, k: Sign) -> &mut V {
        match k {
            Sign::Pos => &mut self.pos_val,
            Sign::Neg => &mut self.neg_val,
        }
    }
}
