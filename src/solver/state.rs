use std::collections::HashMap;

use crate::problem::*;

pub struct State<'problem> {
    assigned: HashMap<Variable, AssignInfo>,
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
        let assigned = problem.vars().iter().map(init_var).collect();
        let disjunctions = Vec::with_capacity(ndisjuncts);
        let unhandled_pure_vars = problem.vars().iter().map(|&v| v).collect();
        let unhandled_propogating_disjunctions = Vec::with_capacity(ndisjuncts);
        let mut res = State {
            assigned,
            disjunctions,
            unhandled_pure_vars,
            unhandled_propogating_disjunctions,
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
        self.disjunctions.push(info);
        self.unhandled_propogating_disjunctions.push(disjunction);
    }
    fn using_rules<'a>(&'a mut self, var: Variable) -> Vec<&'a mut DisjunctionInfo> {
        unimplemented!()
    }
    pub fn assign(&mut self, var: Variable, sig: Sign) {
        {
            let v = self
                .assigned
                .get_mut(&var)
                .unwrap_or_else(|| panic!("Missing var"));
            assert!(v.sign.is_none());
            v.sign = Some(sig);
        }
        for disinfo in self.using_rules(var) {
            unimplemented!()
        }
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
