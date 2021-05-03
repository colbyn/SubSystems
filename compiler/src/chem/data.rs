use std::iter::FromIterator;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::{HashMap, BTreeSet, BTreeMap, LinkedList};

use crate::numbers::Number;
use crate::matrix::{Matrix, Row, Column};

///////////////////////////////////////////////////////////////////////////////
// BASICS
///////////////////////////////////////////////////////////////////////////////

type Coefficient = Number;
type Subscript = Number;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Element(pub String);

impl Element {
    pub fn from_str(value: &str) -> Self {
        Element(String::from(value))
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Aq,
    S,
    L,
    G
}


///////////////////////////////////////////////////////////////////////////////
// SUBSTANCE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum Node {
    Chunk(Coefficient, Vec<Node>, Option<State>),
    Parens(Vec<Node>, Subscript),
    Unit(Element, Subscript),
}

impl Node {
    pub fn to_string(&self) -> String {
        fn fancy_unicode_subscript(x: &Number) -> String {
            let sign = x.sign();
            let num = x.numerator().unwrap().abs();
            let den = x.denominator().unwrap().abs();
            if den == 1 {
                format!("{}", num)
                    .chars()
                    .map(|x: char| {
                        match x {
                            '0' => '₀',
                            '1' => '₁',
                            '2' => '₂',
                            '3' => '₃',
                            '4' => '₄',
                            '5' => '₅',
                            '6' => '₆',
                            '7' => '₇',
                            '8' => '₈',
                            '9' => '₉',
                            x => x,
                        }
                    })
                    .collect::<String>()
            } else {
                unimplemented!()
            }
        }
        fn render_list(xs: &Vec<Node>) -> String {
            xs.into_iter()
              .map(|x| x.to_string())
              .collect::<Vec<_>>()
              .join("")
        }
        fn render_state(state: &State) -> String {
            match state {
                State::Aq => String::from("(aq)"),
                State::G => String::from("(g)"),
                State::L => String::from("(l)"),
                State::S => String::from("(s)"),
            }
        }
        match self {
            Node::Chunk(coefficient, xs, state_opt) => {
                let xs = render_list(xs);
                let state = state_opt.as_ref().map(render_state).unwrap_or(String::new());
                format!("{}{} {}", coefficient, xs, state)
            }
            Node::Parens(xs, subscritp) => {
                let xs = render_list(xs);
                format!("({}){}", xs, fancy_unicode_subscript(&subscritp))
            }
            Node::Unit(element, subscritp) => {
                format!("{}{}", element, fancy_unicode_subscript(&subscritp))
            }
        }
    }
    pub fn from_str(source: &str) -> Option<Self> {
        crate::chem::parser::parse_group(source).ok().map(|(_, x)| x)
    }
    pub fn root_coefficient(&self) -> Number {
        match self {
            Node::Chunk(x, _, _) => x.clone(),
            Node::Parens(_, _) => Number::int(1),
            Node::Unit(_, _) => Number::int(1),
        }
    }
    pub fn trans(self, func: Rc<RefCell<dyn Fn(Node) -> Option<Node>>>) -> Option<Node> {
        let new_node = match self {
            Node::Chunk(co, xs, st) => {
                let xs = xs
                    .into_iter()
                    .filter_map(|x| x.trans(func.clone()))
                    .collect::<Vec<_>>();
                Node::Chunk(co, xs, st)
            }
            Node::Parens(xs, sub) => {
                let xs = xs
                    .into_iter()
                    .filter_map(|x| x.trans(func.clone()))
                    .collect::<Vec<_>>();
                Node::Parens(xs, sub)
            }
            Node::Unit(x, sub) => {
                Node::Unit(x, sub)
            }
        };
        (func.borrow_mut())(new_node)
    }
    pub fn atoms(&self) -> Vec<Element> {
        #[inline]
        fn go(input: &[Node], mult: Number) -> Vec<Element> {
            let atms = input
                .iter()
                .flat_map(|x| x.atoms())
                .collect::<Vec<_>>();
            (1 ..= mult.unpack_integer().unwrap())
                .into_iter()
                .flat_map(|_| atms.clone())
                .collect::<Vec<_>>()
        }
        match self {
            Node::Chunk(co, xs, _) => {
                go(xs, co.clone())
            }
            Node::Parens(xs, sub) => {
                go(xs, sub.clone())
            }
            Node::Unit(x, sub) => {
                (1 ..= sub.unpack_integer().unwrap())
                    .into_iter()
                    .map(|_| x.clone())
                    .collect::<Vec<_>>()
            }
        }
    }
    pub fn count(&self, reference: &Element) -> usize {
        let atoms = self.atoms();
        let mut counter = 0;
        for element in atoms {
            if element == *reference {
                counter = counter + 1;
            }
        }
        counter
    }
    pub fn coefficient_map(
        &self,
        total_elements: &BTreeSet<Element>
    ) -> BTreeMap<Element, usize> {
        assert_eq!(true, {
            self.atoms()
                .into_iter()
                .all(|atom| total_elements.get(&atom).is_some())
        });
        total_elements
            .iter()
            .map(|element| {
                (element.clone(), self.count(element))
            })
            .collect()
    }
    pub fn coefficient_row(
        &self,
        total_elements: &BTreeSet<Element>
    ) -> Row {
        let map = self.coefficient_map(total_elements);
        let xs = map
            .values()
            .map(|x| Number::from_usize(*x))
            .collect::<Vec<_>>();
        Row(xs)
    }
    pub fn coefficient_column(
        &self,
        total_elements: &BTreeSet<Element>
    ) -> Column {
        self.coefficient_row(total_elements).transpose()
    }
}


///////////////////////////////////////////////////////////////////////////////
// SUM OF NODES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Sequence(pub Vec<Node>);

impl Sequence {
    pub fn atoms(&self) -> Vec<Element> {
        self.0
            .iter()
            .flat_map(|x| x.atoms())
            .collect::<Vec<_>>()
    }
    pub fn coefficient_column(
        &self,
        total_elements: &BTreeSet<Element>,
        sign: isize,
    ) -> Vec<Column> {
        let sign = Number::from_isize(sign);
        self.0
            .iter()
            .map(|x| x.coefficient_column(total_elements))
            .map(|x| x.mul_each(&sign))
            .collect()
    }
}


///////////////////////////////////////////////////////////////////////////////
// REACTION
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Reaction {
    pub reactants: Sequence,
    pub products: Sequence,
}

impl Reaction {
    pub fn from_str(source: &str) -> Option<Self> {
        crate::chem::parser::parse_reaction(source).ok().map(|(_, x)| x)
    }
    pub fn to_string(&self) -> String {
        let left = self.reactants.0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" + ");
        let right = self.products.0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" + ");
        format!("{} ⟶   {}", left, right)
    }
    pub fn is_balanced(&self) -> bool {
        let left = self.reactants.atoms();
        let right = self.products.atoms();
        left.len() == right.len()
    }
    pub fn is_valid(&self) -> bool {
        let left = BTreeSet::<Element>::from_iter(self.reactants.atoms());
        let right = BTreeSet::<Element>::from_iter(self.products.atoms());
        left.len() == right.len()
    }
    pub fn merge(&self) -> Sequence {
        Sequence(vec![
            self.reactants.0.clone(),
            self.products.0.clone(),
        ].concat())
    }
    pub fn balance(&self) {
        // Must be a BTreeSet data type; we explicitly want an ordered data type.
        let total_elements: BTreeSet<Element> = BTreeSet::from_iter(
            vec![self.reactants.atoms(), self.products.atoms()].concat()
        );
        let mut matrix = Matrix::new();
        for col in self.reactants.coefficient_column(&total_elements, 1) {
            matrix.push_column(col);
        }
        for col in self.products.coefficient_column(&total_elements, -1) {
            matrix.push_column(col);
        }
        matrix.push_column(Column(vec![
            Number::int(0),
            Number::int(0),
            Number::int(0),
        ]));
        let solved = matrix
            .non_zero_diagnal()
            .solve();
    }
}

///////////////////////////////////////////////////////////////////////////////
// UTILS
///////////////////////////////////////////////////////////////////////////////

// fn add_row_to_columns(columns: Vec<na::DVector<f64>>, entry: f64) -> Vec<na::DVector<f64>> {
//     columns
//         .into_iter()
//         .map(|column| {
//             let row_count = column.nrows();
//             column.insert_row(row_count, entry)
//         })
//         .collect()
// }

///////////////////////////////////////////////////////////////////////////////
// ENV
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Env {
    subs: Rc<RefCell<HashMap<String, Node>>>,
    counter: Counter,
}

impl Env {
    pub fn new_name(&self) -> String {
        format!("𝜶{}", self.counter.inc())
    }
    pub fn insert(&self, node: Node) -> String {
        let new_name = self.new_name();
        let entry = self.subs.borrow_mut().insert(
            new_name.clone(),
            node,
        );
        assert!(entry.is_none());
        new_name
    }
}

#[derive(Clone, Default)]
pub struct Counter(Rc<RefCell<usize>>);

impl Counter {
    pub fn new() -> Self {Counter::default()}
    pub fn inc(&self) -> usize {
        let old_value = self.0.replace_with(|old| *old + 1);
        old_value
    }
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn dev() {
    // let source = "Ca(O3H2)2(aq) + HCl(aq) -> CaCl2(aq) + H2O(l)";
    // let source = "PCl5 + H2O -> H3PO4 + HCl";
    // let source = "Al + O2 -> Al2O3";
    // let source_balanced = "1PCl5 + 4H2O -> 1H3PO4 + 5HCl";
    // let source = "C3H8 + O2 -> CO2 + H2O";
    // let source = "Na3PO4 + Ba(NO3)2 -> Ba3(PO4)2 + NaNO3";
    let source = "2XY2 + 4Y -> 8X2 + 8YX";
    let mut reaction = Reaction::from_str(source).unwrap();
    println!("{}", reaction.to_string());
    reaction.balance();

    // reaction.balance();
    // let elements = BTreeSet::<Element>::from_iter(reaction.merge().atoms());
    // for term in reaction.merge().0 {
    //     println!("{:#?}", term.coefficient_map(&elements));
    //     // println!("{} : {:?}", term.to_string(), term.atoms());
    // }
}

pub fn main() {
    dev();
    // use nalgebra::*;
    // let columns = vec![
    //     na::RowDVector::from_vec(vec![1, 2, 6]),
    //     na::RowDVector::from_vec(vec![2, 1, 0]),
    //     na::RowDVector::from_vec(vec![2, 3, 1]),
    // ];
    // let constants = na::DVector::from_vec(vec![7, 6, 9]);
    // let matrix = na::DMatrix::from_rows(&columns.into_iter().collect::<Vec<_>>()).map(|x| x as f64);
    // let constants = constants.map(|x| x as f64);
    // let x = matrix.clone().svd(true, true).solve(&constants, 0.0).unwrap();
    // println!("{}", matrix);
    // println!("{}", constants);
    // println!("{}", x);
}