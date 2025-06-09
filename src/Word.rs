// pg 361. Defintion 2: Generalized Equation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Label<C, X> {
    Constant(C),
    Variable(X),
}

// A word over an alphabet A is a finite sequence of elements from A.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Word<A>(Vec<A>);


// Pg 362. A unifier maps variables to words.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Unifier {
    unifications: HashMap<X, Word<Label<C, X>>>,
}

// Pg 360.
struct WordEquation<C, X> {
    left : Word<Label<C, X>>,
    right : Word<Label<C, X>>,
}
