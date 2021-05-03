use std::ops::{Range, Bound, RangeBounds};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::convert::AsRef;
use std::collections::{HashMap, LinkedList, HashSet, BTreeMap};
use num::{FromPrimitive, ToPrimitive, BigRational, BigInt, Signed};

use crate::numbers::Number;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

///////////////////////////////////////////////////////////////////////////////
// TYPE HELPERS
///////////////////////////////////////////////////////////////////////////////

pub type Index = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexPosition {
    row_index: usize,
    col_index: usize,
}

///////////////////////////////////////////////////////////////////////////////
// FUNCTION HELPERS
///////////////////////////////////////////////////////////////////////////////


fn order_solver(xs: Vec<(HashSet<usize>, char)>) -> Vec<char> {
    let mut done = false;
    fn init(size: usize) -> Vec<Option<char>> {
        (0..size)
            .into_iter()
            .map(|_| None)
            .collect()
    }
    fn cycle(
        options: Vec<(HashSet<usize>, char)>,
        last: Vec<Option<char>>,
        flip: bool,
    ) -> Vec<Option<char>> {
        let mut used: HashSet<char> = Default::default();
        let mut current = init(options.len());
        let mut for_entry = |ix: usize, entry: Option<char>| {
            match entry {
                None => {
                    for (valids, id) in options.clone() {
                        if valids.contains(&ix) && !used.contains(&id) {
                            used.insert(id);
                            current[ix] = Some(id);
                        }
                    }
                }
                Some(x) => {
                    used.insert(x);
                    current[ix] = Some(x);
                }
            }
        };
        match flip {
            true => for (ix, entry) in last.clone().into_iter().enumerate().rev() {
                for_entry(ix, entry);
            },
            false => for (ix, entry) in last.clone().into_iter().enumerate() {
                for_entry(ix, entry);
            },
        }
        current
    }
    let mut flip = false;
    let mut layout = init(xs.len());
    let mut cycle_counter = 0;
    while !done {
        layout = cycle(
            xs.clone(),
            layout,
            flip,
        );
        done = layout.iter().all(|x| x.is_some());
        let flip = !flip;
        cycle_counter = cycle_counter + 1;
        assert!(cycle_counter <= 100);
    }
    layout
        .into_iter()
        .map(|x| x.unwrap())
        .collect()
}

///////////////////////////////////////////////////////////////////////////////
// BASIC TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct Column(pub Vec<Number>);

impl Column {
    pub fn transpose(self) -> Row {
        Row(self.0)
    }
    pub fn map(self, f: impl Fn(Number) -> Number) -> Self {
        let xs = self.0
            .into_iter()
            .map(f)
            .collect();
        Column(xs)
    }
    pub fn add_each(self, value: &Number) -> Self {
        self.map(|x| &x + value)
    }
    pub fn div_each(self, value: &Number) -> Self {
        self.map(|x| &x / value)
    }
    pub fn mul_each(self, value: &Number) -> Self {
        self.map(|x| &x * value)
    }
    pub fn zeros(length: usize) -> Self {
        let mut xs = (0..length)
            .into_iter()
            .map(|_| Number::int(0))
            .collect::<Vec<_>>();
        Column(xs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row(pub Vec<Number>);

impl Row {
    pub fn max(&self) -> Number {
        let mut max_val = None::<Number>;
        for x in self.0.iter() {
            match max_val.clone() {
                Some(max) if x.clone() > max.clone() => {
                    max_val = Some(x.clone());
                }
                Some(_) => {}
                None => {
                    max_val = Some(x.clone());
                }
            }
        }
        max_val.unwrap()
    }
    pub fn map(&self, f: impl Fn(&Number) -> Number) -> Self {
        let xs = self.0
            .iter()
            .map(f)
            .collect();
        Row(xs)
    }
    pub fn map_mut(&mut self, f: impl Fn(&Number) -> Number) {
        for x in self.0.iter_mut() {
            *x = f(x);
        }
    }
    pub fn find_indexes(&self, f: impl Fn(usize, &Number) -> bool) -> Vec<usize> {
        let mut matches = Vec::new();
        for (ix, x) in self.0.iter().enumerate() {
            if f(ix, x) {
                matches.push(ix);
            }
        }
        matches
    }
    pub fn add_each(self, value: &Number) -> Self {
        self.map(|x| x + value)
    }
    pub fn div_each(self, value: &Number) -> Self {
        self.map(|x| x / value)
    }
    pub fn mul_each(self, value: &Number) -> Self {
        self.map(|x| x * value)
    }
    pub fn add(&self, other: &Row) -> Self {
        assert_eq!(self.0.len(), other.0.len());
        let xs = self.0
            .iter()
            .zip(other.0.iter())
            .map(|(l, r)| {
                l + r
            })
            .collect();
        Row(xs)
    }
    pub fn transpose(self) -> Column {
        Column(self.0)
    }
    pub fn get(&self, ix: usize) -> Option<&Number> {
        self.0.get(ix)
    }
}

impl From<Column> for Vec<Number> {
    fn from(x: Column) -> Self {x.0}
}
impl From<Row> for Vec<Number> {
    fn from(x: Row) -> Self {x.0}
}


///////////////////////////////////////////////////////////////////////////////
// MATRIX TYPES
///////////////////////////////////////////////////////////////////////////////

/// A dynamic matrix in **row major order**.
#[derive(Clone, PartialEq)]
pub struct Matrix(Vec<Row>);

impl Matrix {
    pub fn new() -> Self {Matrix(Default::default())}
    pub fn len(&self) -> (usize, usize) {
        if self.0.is_empty() {
            return (0, 0)
        }
        let mut row_size = self.0.len();
        let mut col_size = None;
        for row in self.0.iter() {
            match col_size {
                None => {
                    col_size = Some(row.0.len());
                }
                Some(ix) => {
                    assert_eq!(ix, row.0.len());
                }
            }
        }
        assert!(col_size.is_some());
        (row_size, col_size.unwrap())
    }
    pub fn row_len(&self) -> usize {
        self.len().0
    }
    pub fn col_len(&self) -> usize {
        self.len().1
    }
    pub fn unpack_singleton(&self) -> Option<Number> {
        match &self.0[..] {
            [r] => {
                match &r.0[..] {
                    [x] => Some(x.clone()),
                    _ => None
                }
            }
            _ => None
        }
    }
    pub fn unpack_column_vector(&self) -> Option<Column> {
        let mut column = Vec::new();
        for r in self.0.iter() {
            match &r.0[..] {
                [c] => {
                    column.push(c.clone());
                }
                _ => {
                    return None
                }
            }
        }
        Some(Column(column))
    }
    fn validate(&self) -> bool {
        let mut column_size = None;
        let mut all_valid = true;
        for row in self.0.iter() {
            match column_size {
                None => {
                    column_size = Some(row.0.len());
                }
                Some(ix) => {
                    all_valid = row.0.len() == ix;
                }
            }
        }
        all_valid
    }
    pub fn from_rows(rows: Vec<Vec<Number>>) -> Option<Self> {
        let matrix = Matrix(
            rows.into_iter()
                .map(|xs| Row(xs))
                .collect::<Vec<_>>()
        );
        if matrix.validate() {
            Some(matrix)
        } else {
            None
        }
    }
    pub fn column_vector(cols: impl Into<Vec<Number>>) -> Option<Self> {
        let cols = cols.into();
        if cols.is_empty() {
            return None
        }
        let matrix = Matrix::from_rows(vec![cols])
            .unwrap()
            .transpose();
        assert!(matrix.validate());
        Some(matrix)
    }
    pub fn transpose(self) -> Matrix {
        let row_len = self.row_len();
        let col_len = self.col_len();
        let mut new_matrix = Matrix::new();
        for c in 0..col_len {
            let column = self.get_column(c).unwrap().0;
            new_matrix.push_row(column);
        }
        new_matrix
    }
    pub fn push_row<T: Into<Vec<Number>>>(&mut self, row: T) {
        self.0.push(Row(row.into()));
    }
    pub fn push_column(
        &mut self,
        column: impl Into<Vec<Number>>,
    ) {
        let mut column = column.into();
        assert!(!column.is_empty());
        for (col_ix, col) in column.into_iter().enumerate() {
            if let Some(row) = self.0.get_mut(col_ix) {
                row.0.push(col);
            } else {
                self.0.push(Row(vec![col]));
            }
        }
    }
    pub fn to_string(&self) -> String {
        let mut rows = Vec::<String>::new();
        let row_len = self.row_len();
        let mut max_column_len = 0;
        for row in self.0.iter() {
            let mut column = Vec::<String>::new();
            for expr in row.0.iter() {
                column.push(format!(" {}", expr.to_string()));
            }
            let column = column.join(" ");
            if column.len() > max_column_len {
                max_column_len = column.len();
            }
            rows.push(column);
        }
        for (ix, row) in rows.iter_mut().enumerate() {
            let added_len = max_column_len - row.len();
            let spaces = (0..=added_len).map(|_| " ").collect::<String>();
            let row_as_spaces = (0..row.len())
                .into_iter()
                .map(|_| ' ')
                .collect::<String>();
            // ┐┘
            match ix {
                0 => {
                    *row = format!("  ┌{rowp}{ws}┐\n  │{row}{ws}│", row=row, ws=spaces, rowp=row_as_spaces);
                }
                _ if ix == row_len - 1 => {
                    *row = format!("  │{row}{ws}│\n  └{rowp}{ws}┘", row=row, ws=spaces, rowp=row_as_spaces);
                }
                _ => {
                    *row = format!("  │{}{}│", row, spaces);
                }
            }
        }
        rows.join("\n")
    }
    pub fn set(&mut self, ix: (usize, usize), value: Number) {
        let mut is_set = false;
        for (mut i, row) in self.0.iter_mut().enumerate() {
            if i == ix.0 {
                for (j, column) in row.0.iter_mut().enumerate() {
                    if j == ix.1 {
                        *column = value.clone();
                        is_set = true;
                    }
                }
            }
        }
        assert_eq!(is_set, true);
    }
    pub fn map_range(
        &mut self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
        f: impl Fn(IndexPosition, Number) -> Number,
    )
    {
        let mut did_set = false;
        for (r_ix, r) in self.0.iter_mut().enumerate() {
            for (c_ix, c) in r.0.iter_mut().enumerate() {
                if row_range.contains(&r_ix) && col_range.contains(&c_ix) {
                    let pos = IndexPosition {
                        row_index: r_ix,
                        col_index: c_ix,
                    };
                    *c = f(pos, c.clone());
                    did_set = true;
                }
            }
        }
        assert_eq!(did_set, true);
    }
    pub fn new_from(
        &mut self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> Matrix {
        let mut did_set = false;
        let mut rows = Vec::<Row>::new();
        for (r_ix, r) in self.0.iter_mut().enumerate() {
            let mut row = Vec::<Number>::new();
            for (c_ix, c) in r.0.iter_mut().enumerate() {
                if row_range.contains(&r_ix) && col_range.contains(&c_ix) {
                    did_set = true;
                    row.push(c.clone());
                }
            }
            if !row.is_empty() {
                rows.push(Row(row));
            }
        }
        let new_matrix = Matrix(rows);
        assert!(new_matrix.validate());
        assert!(did_set);
        new_matrix
    }
    pub fn map_row(&mut self, row_ix: usize, f: impl Fn(IndexPosition, Number) -> Number) {
        self.map_range(
            row_ix..=row_ix,
            ..,
            f,
        );
    }
    pub fn map_col(&mut self, col_ix: usize, f: impl Fn(IndexPosition, Number) -> Number) {
        self.map_range(
            ..,
            col_ix..=col_ix,
            f,
        );
    }
    pub fn replace_row(&mut self, row_ix: usize, new_row: Row) {
        let mut did_set = false;
        for (ix, row) in self.0.iter_mut().enumerate() {
            if ix == row_ix {
                *row = new_row.clone();
                did_set = true;
            }
        }
        assert_eq!(did_set, true);
    }
    pub fn mul_row(&mut self, row_ix: usize, mult: &Number) {
        self.map_row(row_ix, move |_, x| x * mult.clone())
    }
    pub fn get(&self, ix: (usize, usize)) -> Option<&Number> {
        let mut is_set = false;
        for (i, row) in self.0.iter().enumerate() {
            if i == ix.0 {
                for (j, column) in row.0.iter().enumerate() {
                    if j == ix.1 {
                        return Some(column)
                    }
                }
            }
        }
        None
    }
    pub fn get_row(&self, ix: usize) -> Option<&Row> {
        let mut is_set = false;
        for (i, row) in self.0.iter().enumerate() {
            if i == ix {
                return Some(row)
            }
        }
        None
    }
    pub fn get_column(&self, ix: usize) -> Option<Column> {
        let mut return_column = Vec::new();
        for (r_ix, r) in self.0.iter().enumerate() {
            for (c_ix, c) in r.0.iter().enumerate() {
                if c_ix == ix {
                    return_column.push(c.clone());
                }
            }
        }
        if !return_column.is_empty() {
            Some(Column(return_column))
        } else {
            None
        }
    }
    pub fn unsafe_get(&self, ix: (usize, usize)) -> &Number {
        self.get(ix).unwrap()
    }
    pub fn unsafe_get_row(&self, ix: usize) -> &Row {
        self.get_row(ix).unwrap()
    }
    pub fn dot(&self, rhs: &Matrix) -> Option<Matrix> {
        let mut new_rows = Vec::new();
        for l in 0..self.row_len() {
            let mut new_row = Vec::new();
            for r in 0..rhs.col_len() {
                let row = self.get_row(l).unwrap();
                let column = rhs.get_column(r).unwrap();
                if row.0.len() != column.0.len() {
                    return None
                }
                let result = row.0
                    .iter()
                    .zip(column.0.iter())
                    .map(|(l, r)| l * r)
                    .fold(Number::int(0), |l, r| l + r);
                new_row.push(result);
            }
            new_rows.push(new_row);
        }
        Some(Matrix::from_rows(new_rows).unwrap())
    }
    pub fn non_zero_diagnal(self) -> Matrix {
        let mut total_rows = self.row_len();
        let mut new_rows: Vec<(HashSet<usize>, char, Row)> = Default::default();
        // TODO: This is just for initial dev (in case I need to further debug it).
        let mut ids = vec![
            'A', 'B', 'C', 'D', 'E',
            'F', 'G', 'H', 'I', 'J',
            'K', 'L', 'M', 'N', 'O',
            'P', 'Q', 'R', 'S', 'T',
            'U', 'V', 'W', 'X', 'Y',
            'Z',
        ];
        for row in self.0.into_iter() {
            let valid_ixs = row.find_indexes(|_, x| x != &Number::int(0));
            let valid_ixs = HashSet::<usize>::from_iter(valid_ixs);
            let row_id = ids.remove(0);
            new_rows.push((
                valid_ixs,
                row_id,
                row,
            ));
        }
        let result = order_solver(
            new_rows
                .clone()
                .into_iter()
                .map(|(vs, id, _)| (vs, id))
                .collect::<Vec<_>>()
        );
        let mut new_matrix = Matrix::new();
        for id in result {
            for (_, rid, row) in new_rows.iter() {
                if &id == rid {
                    new_matrix.push_row(row.clone());
                }
            }
        }
        assert!(new_matrix.validate());
        new_matrix
    }
    pub fn forward_elimination(&mut self) {
        let row_len = self.row_len();
        let col_len = self.col_len();
        for i in 0..(col_len) {
            for j in (i + 1)..row_len {
                let new_row = (|| -> Option<Row> {
                    let i_mult = {
                        let l = self.unsafe_get((j, i));
                        let r = self.unsafe_get((i, i));
                        l.save_div(r).unwrap_or_else(|| panic!(
                            "cannot div by zero!!!"
                        ))
                    };
                    let i_mult = -i_mult;
                    let j_row = self
                        .unsafe_get_row(j);
                    let i_row = self
                        .unsafe_get_row(i)
                        .clone()
                        .mul_each(&i_mult)
                        .add(j_row);
                    Some(i_row)
                })();
                let new_row = new_row.unwrap();
                self.replace_row(j, new_row);
            }
        }
    }
    pub fn solve(&self) -> Column {
        let mut this = self.clone();
        let row_len = this.row_len();
        let col_len = this.col_len();
        this.forward_elimination();
        // Solution Vector
        let mut solution = (0..(col_len-1))
            .into_iter()
            .map(|_| Number::int(0))
            .collect::<Vec<_>>();
        let mut solution = Matrix::column_vector(solution).unwrap();
        for i in (0..(row_len)).rev() {
            let l = this.get((i, col_len - 1)).unwrap().clone();
            let r = this.new_from(
                i..=i,
                0..(col_len - 1),
            );
            let r = r
                .dot(&solution)
                .unwrap()
                .unpack_singleton()
                .unwrap();
            let current = {
                let num = (l - r);
                let den = this.get((i, i)).unwrap();
                num.save_div(den).unwrap_or_else(|| panic!(
                    "cannot div by zero!!!"
                ))
            };
            solution.set((i, 0), current);
        }
        solution.unpack_column_vector().unwrap()
    }
    pub fn nullspace(&self) -> Column {
        let col_len = self.col_len();
        let mut solution = self.solve();
        for ix in 0..=col_len {
            if ix > solution.0.len() {
                solution.0.push(Number::int(1));
            }
        }
        solution
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.0
            .iter()
            .map(|row| -> String {
                let row = row.0.iter()
                    .map(|x| format!("{:?}", x))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("  [{}]", row)
            })
            .collect::<Vec<_>>()
            .join(",\n");
        write!(f, "[\n{}\n]", rows)
    }
}


impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = Matrix::column_vector(self.clone().0).unwrap();
        write!(f, "{}", this)
    }
}
impl std::fmt::Debug for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = Matrix::column_vector(self.clone().0).unwrap();
        this.fmt(f)
    }
}


///////////////////////////////////////////////////////////////////////////////
// MACRO HELPERS
///////////////////////////////////////////////////////////////////////////////


// $(,)?
/// Internal helper.
#[macro_export]
macro_rules! matrix_row {
    ($row:expr;; $($entry:expr)*) => {
        $(
            $row.push($entry);
        )*
    };
}

// $(,)?
/// Internal helper.
#[macro_export]
macro_rules! matrix_rows {
    ($rows:expr;; $(
        $($entry:expr),*
    );*) => {
        $({
            let mut row: Vec<Number> = Vec::new();
            matrix_row!(row;; $($entry)*);
            if !row.is_empty() {
                $rows.push(row);
            }
        })*
    };
}

#[macro_export]
macro_rules! matrix {
    ($($x:tt)*) => {{
        let mut rows: Vec<Vec<Number>> = Vec::new();
        matrix_rows!(rows;; $($x)*);
        Matrix::from_rows(rows).unwrap()
    }};
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn main() {
    // let mut matrix: Matrix = matrix!{
    //     Number::int(1), Number::int(2), Number::int(-6), Number::int(6);
    //     Number::int(5), Number::int(0), Number::int(-5), Number::int(6);
    //     Number::int(1), Number::int(4), Number::int(0), Number::int(8);
    // };
    // println!("\n");
    // let solution = matrix.solve();
    // println!("{}", matrix);
    let mut matrix: Matrix = matrix!{
        Number::int(8), Number::int(0), Number::int(0), Number::int(-2);
        Number::int(0), Number::int(2), Number::int(-2), Number::int(-1);
        Number::int(3), Number::int(0), Number::int(-1), Number::int(0);
    };
    let solution = matrix
        .clone()
        // .non_zero_diagnal()
        .nullspace();
    println!("-----------");
    // println!("-----------");
    // println!("=");
    println!("{}", solution)
}