use std::iter::Chain;
use std::slice;

use crate::problem::*;

pub struct WrappedProblem {
    pub problem: Problem,
    pub learnt_disjunctions: Vec<Disjunction>,
}

pub type AllDisjunctionsIter<'a> =
    Chain<slice::Iter<'a, Disjunction>, slice::Iter<'a, Disjunction>>;

impl WrappedProblem {
    pub fn disjunctions(&self) -> AllDisjunctionsIter {
        self.problem
            .disjunctions
            .iter()
            .chain(self.learnt_disjunctions.iter())
    }
}