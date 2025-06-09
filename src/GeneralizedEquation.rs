use std::collections::HashMap;
use crate::Word::*;

// pg 361, 362:
// While boundary claims to be an abstract total order,
// we also make use of the fact that on pg. 362, we can say 'boundary+1',
// which implies that the most sensible way to encode it is probably as a natural number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Boundary(usize);

// pg 361. Defintion 2: Generalized Equation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Base<C, X> {
    label : Label<C, X>,
    boundaries : Vec<Boundary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BaseId(usize); // a pointer into the vector of bases.

// pg 361. Defintion 2: Generalized Equation
#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneralizedEquation<C, X : std::cmp::Eq + std::hash::Hash> {
    bases : Vec<Base<C, X>>,
    var2base : HashMap<(X, bool), BaseId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum C1Violation<X> {
    NotEnoughDuals(X),
    TooManyDuals(X, BaseId, BaseId, BaseId),
    NotEqualBoundaryLength(X, BaseId, BaseId),
}

impl<C, X : std::cmp::Eq + std::hash::Hash > GeneralizedEquation<C, X > {
    // pg. 361
    fn checkC1(&self) -> Vec<C1Violation<X>> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum C2Violation<C> {
    BoundaryLengthNotTwo(C, BaseId),
}

impl<C, X : std::cmp::Eq + std::hash::Hash> GeneralizedEquation<C, X> {
    fn checkC2(&self) -> Vec<C2Violation<C>> {
        Vec::new()
    }
}


impl<C, X> Base<C, X> {
    // pg. 361
    fn is_constant(&self) -> bool {
        matches!(self.label, Label::Constant(_))
    }
    // pg. 361
    fn is_variable(&self) -> bool {
        matches!(self.label, Label::Variable(_))
    }
}

// pg. 362
fn is_boundary_pair_column(i : &Boundary, j : &Boundary) -> bool {
    let Boundary(i) = i;
    let Boundary(j) = j;
    i <= j
}

// pg. 362
fn is_boundary_pair_indecomposable(i : &Boundary, j : &Boundary) -> bool {
    let Boundary(i) = i;
    let Boundary(j) = j;
    *i + 1 == *j
}

// pg. 362
fn is_boundary_pair_empty(i : &Boundary, j : &Boundary) -> bool {
    let Boundary(i) = i;
    let Boundary(j) = j;
    i == j
}

// pg 362. A generalized equation is solved if all its variable bases are empty.
impl<C, X> Base <C, X> {
    fn is_solved(&self) -> bool {
        if self.is_constant() {
            return true;
        }
        for slice in self.boundaries.windows(2) {
            match slice {
                [b1, b2] if is_boundary_pair_column(b1, b2) => {
                    if !is_boundary_pair_indecomposable(b1, b2) {
                        return false;
                    }
                },
                _ => continue,
            }
        }
        return true;
    }
}
impl<C, X : std::hash::Hash + std::cmp::Eq > GeneralizedEquation<C, X> {
    fn is_solved(&self) -> bool {
        for base in &self.bases {
            if base.is_variable() && !base.boundaries.is_empty() {
                return false;
            }
        }
        true
    }
}



// Definition 4. Linear diophantine equation L(GE, c)
// corresponding to generalized equation.

// Lemma 5. If GE has a unifier, then L(GE, c) is solvable for each 'c in C'.
