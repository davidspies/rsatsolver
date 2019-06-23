use std::collections::HashMap;
use std::mem;

use crate::problem::*;

#[derive(Clone, Copy)]
struct DisjunctionIndex(usize);

pub struct State<'problem> {
    assigned: HashMap<Variable, AssignInfo>,
    used_by: HashMap<Variable, Vec<DisjunctionIndex>>,
    disjunctions: Vec<DisjunctionInfo<'problem>>,
    unhandled_pure_vars: Vec<Variable>,
    unhandled_propogating_disjunctions: Vec<&'problem Disjunction>,
}

impl<'problem> State<'problem> {
    pub fn new(problem: &'problem Problem) -> Self {
        fn init_var(&v: &Variable) -> (Variable, AssignInfo) {
            (
                v,
                AssignInfo {
                    sign: None,
                    use_counts: SignMap::new(|_sig| 0),
                },
            )
        }
        let ndisjuncts = problem.disjunctions.len();
        let mut res = State {
            assigned: problem.vars().iter().map(init_var).collect(),
            used_by: HashMap::new(),
            disjunctions: Vec::with_capacity(ndisjuncts),
            unhandled_pure_vars: problem.vars().iter().map(|&v| v).collect(),
            unhandled_propogating_disjunctions: Vec::with_capacity(ndisjuncts),
        };
        for disjunction in &problem.disjunctions {
            res.add_disjunction(disjunction);
        }
        res
    }
    pub fn is_set(&self, v: Variable) -> bool {
        self.sign_of(v).is_some()
    }
    pub fn sign_of(&self, v: Variable) -> Option<Sign> {
        self.assigned[&v].sign
    }
    pub fn add_disjunction(&mut self, disjunction: &'problem Disjunction) {
        for (k, &v) in disjunction.iter() {
            self.assigned.get_mut(k).unwrap().use_counts[v] += 1;
        }
        let info = DisjunctionInfo::new(&self, disjunction);
        let i = DisjunctionIndex(self.disjunctions.len());
        self.disjunctions.push(info);
        self.unhandled_propogating_disjunctions.push(disjunction);
        for (x, _) in disjunction.iter() {
            self.used_by.get_mut(x).unwrap().push(i);
        }
    }
    pub fn assign(&mut self, var: Variable, sig: Sign) {
        {
            let v = self.assigned.get_mut(&var).unwrap();
            assert!(v.sign.is_none());
            v.sign = Some(sig);
        }
        for &DisjunctionIndex(di) in &self.used_by[&var] {
            let d = &mut self.disjunctions[di]; // TODO: Skip bounds check?
            if d.disjunction[var] == sig {
                d.satisfied += 1;
                if d.satisfied == 1 {
                    for (&k, &vsig) in d.disjunction.iter() {
                        let uc = &mut self.assigned.get_mut(&k).unwrap().use_counts[vsig];
                        *uc -= 1;
                        if *uc == 0 {
                            self.unhandled_pure_vars.push(k);
                        }
                    }
                }
            } else {
                d.remaining -= 1;
                if d.remaining == 1 {
                    self.unhandled_propogating_disjunctions.push(d.disjunction);
                }
            }
        }
    }
    pub fn unassign(&mut self, var: Variable) {
        let sig = {
            let v = self.assigned.get_mut(&var).unwrap();
            mem::replace(&mut v.sign, None).unwrap()
        };
        for &DisjunctionIndex(di) in &self.used_by[&var] {
            let d = &mut self.disjunctions[di]; // TODO Skip bounds-check?
            if d.disjunction[var] == sig {
                d.satisfied -= 1;
                if d.satisfied == 0 {
                    for (k, &vsig) in d.disjunction.iter() {
                        self.assigned.get_mut(k).unwrap().use_counts[vsig] += 1;
                    }
                }
            } else {
                d.remaining += 1;
            }
        }
    }
    pub fn get_and_clear_propagators(&mut self) -> Vec<&'problem Disjunction> {
        mem::replace(&mut self.unhandled_propogating_disjunctions, Vec::new())
    }
    pub fn get_and_clear_pures(&mut self) -> Vec<Variable> {
        mem::replace(&mut self.unhandled_pure_vars, Vec::new())
    }
}

struct AssignInfo {
    sign: Option<Sign>,
    use_counts: SignMap<usize>,
}

struct DisjunctionInfo<'problem> {
    disjunction: &'problem Disjunction,
    remaining: usize,
    satisfied: usize,
}

impl<'problem> DisjunctionInfo<'problem> {
    fn new(state: &State, disjunction: &'problem Disjunction) -> Self {
        let remaining = disjunction
            .iter()
            .filter(|&(&k, _)| !state.is_set(k))
            .count();
        let satisfied = disjunction
            .iter()
            .filter(|&(&k, &sig)| state.sign_of(k) == Some(sig))
            .count();
        DisjunctionInfo {
            disjunction,
            remaining,
            satisfied,
        }
    }
}
