use std::collections::HashMap;

// pg 361. Defintion 2: Generalized Equation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Label<C, X> {
    Constant(C),
    Variable(X),
}

// A word over an alphabet A is a finite sequence of elements from A.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word<A>(Vec<A>);

// Pg 362. A unifier maps variables to words.
#[derive(Debug, Clone)]
pub struct Unifier<C, X> {
    unifications: HashMap<X, Word<Label<C, X>>>,
}

// Pg 360.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WordEquation<C, X> {
    left : Word<Label<C, X>>,
    right : Word<Label<C, X>>,
}

// TODO: how to effectively compute exponent of periodicity of a word?
