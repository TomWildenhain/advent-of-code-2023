use std::fs;

struct Stone {
    position: Vec<i64>,
    velocity: Vec<i64>
}

fn parse_tuple(tuple_str: &str) -> Vec<i64> {
    let parts: Result<Vec<i64>, _> = tuple_str.split(", ").map(|s| s.parse()).collect();
    return parts.unwrap()
}

fn parse_stone(line: &str) -> Stone {
    let (positions_str, velocity_str) = line.split_once(" @ ").unwrap();
    return Stone {
        position: parse_tuple(positions_str),
        velocity: parse_tuple(velocity_str),
    }
}

// Yes I know there are other crates that implement rationals, but implementing them from scratch can be fun!
mod rationals {
    use std::ops;
    use num::integer::gcd;
    use num::BigInt;
    use num_traits::{Zero,One, ToPrimitive};

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct Rational {
        // Switched to BigInt at the end to prevent overflow. Please excuse all the .clone()/.into().
        numerator: BigInt,
        denominator: BigInt
    }

    impl Rational {
        pub fn new<T:Into<BigInt>>(numerator: T, denominator: T) -> Rational {
            let numerator: BigInt = numerator.into();
            let denominator: BigInt = denominator.into();

            if denominator == Zero::zero() {
                panic!();
            }

            let mut d = gcd(numerator.clone(), denominator.clone());
            if denominator < Zero::zero() && d > Zero::zero() {
                d = -d;
            }

            return Rational { numerator: numerator / d.clone(), denominator: denominator / d };
        }

        pub fn int(n: i64) -> Rational {
            return Rational::new(n, 1);
        }

        pub fn zero() -> Rational {
            return Rational::int(0);
        }

        pub fn reciprocal(&self) -> Rational {
            return Rational::new(self.denominator.clone(), self.numerator.clone());
        }

        pub fn to_int(&self) -> Option<i64> {
            if self.denominator != One::one() {
                return None;
            }

            return self.numerator.to_i64();
        }
    }

    impl ops::Add<Rational> for Rational {
        type Output = Self;

        fn add(self, rhs: Rational) -> Rational {
            return Rational::new(
                self.numerator * rhs.denominator.clone() + rhs.numerator * self.denominator.clone(), 
                self.denominator * rhs.denominator)
        }
    }

    impl ops::Neg for Rational {
        type Output = Self;

        fn neg(self) -> Self::Output {
            return Rational::new(-self.numerator, self.denominator);
        }
    }

    impl ops::Sub<Rational> for Rational {
        type Output = Self;

        fn sub(self, rhs: Rational) -> Rational {
            return self + (-rhs);
        }
    }

    impl ops::Mul<Rational> for Rational {
        type Output = Self;

        fn mul(self, rhs: Rational) -> Rational {
            return Rational::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator);
        }
    }

    impl ops::Div<Rational> for Rational {
        type Output = Self;

        fn div(self, rhs: Rational) -> Rational {
            return self * rhs.reciprocal();
        }
    }
}

mod linear_equations {
    use super::rationals::Rational;

    pub struct Variable {
        pub idx: usize,
        pub name: String,
    }

    pub struct Term<'a> {
        pub constant: Rational,
        pub variable: &'a Variable
    }

    impl<'a> Term<'a> {
        pub fn new(constant: Rational, variable: &Variable) -> Term {
            return Term {constant: constant, variable: variable};
        }
    }

    pub struct Equation<'a> {
        pub lhs: Vec<Term<'a>>,
        pub rhs: Rational
    }
}

mod rles {
    // RLES = Rational Linear Equation Solver (tm)

    use super::rationals::Rational;
    use super::linear_equations::*;

    type Matrix = Vec<Vec<Rational>>;

    fn eqn_to_row(eqn: &Equation, num_variables: usize) -> Vec<Rational> {
        let mut row = vec![Rational::zero(); num_variables + 1];

        row[num_variables] = eqn.rhs.clone();
        for term in eqn.lhs.iter() {
            row[term.variable.idx] = term.constant.clone();
        }

        return row;
    }

    fn find_row_with_nonzero_x(matrix: &Matrix, start_y: usize, start_x: usize) -> Option<(usize, usize)> {
        let height = matrix.len();
        let width = matrix[0].len();

        for x in start_x..width {
            for y in start_y..height {
                if matrix[y][x] != Rational::zero() {
                    return Some((y, x));
                }
            }
        }

        return None;
    }

    fn scale_row(matrix: &mut Matrix, y: usize, scale: Rational) {
        let row = &mut matrix[y];
        for x in 0..row.len() {
            row[x] = row[x].clone() * scale.clone();
        }
    }

    fn add_to_row(matrix: &mut Matrix, y_dst: usize, y_src: usize, scale_src: Rational) {
        let width = matrix[y_dst].len();

        for x in 0..width {
            matrix[y_dst][x] = matrix[y_dst][x].clone() + matrix[y_src][x].clone() * scale_src.clone();
        }
    }

    fn zero_out_column_using_row(matrix: &mut Matrix, y: usize, x: usize) {
        scale_row(matrix, y, matrix[y][x].reciprocal());

        for y2 in 0..matrix.len() {
            if y != y2 {
                let v = matrix[y2][x].clone();
                if v != Rational::zero() {
                    add_to_row(matrix, y2, y, -v);
                }
            }
        }
    }

    pub fn reduced_row_echelon_form(matrix: &mut Matrix) {
        let height = matrix.len();

        let mut x = 0;
        for y in 0..height {
            if let Some((ty, tx)) = find_row_with_nonzero_x(&matrix, y, x) {
                x = tx;
                if ty != y {
                    matrix.swap(y, ty);
                }
                zero_out_column_using_row(matrix, y, x);
            } else {
                // Zeros all the way down
                break;
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum Outcome {
        Solution(Vec<Rational>),
        Underconstrained(Vec<Option<Rational>>),
        Unsolvable,
    }

    enum RowType {
        AllZero,
        OneOne(usize),
        Other
    }

    fn classify_row(row: &[Rational]) -> RowType {
        let mut row_with_one: Option<usize> = None;
        for x in 0..row.len() {
            let v = row[x].clone();
            if v == Rational::zero() {
                continue;
            }
            if v == Rational::int(1) {
                if row_with_one.is_some() {
                    return RowType::Other;
                } else {
                    row_with_one = Some(x);
                }
            }
            else {
                return RowType::Other;
            }
        }

        match row_with_one {
            Some(x) => RowType::OneOne(x),
            None => RowType::AllZero
        }
    }

    pub fn solve<'a>(system: &'a Vec<Equation<'a>>) -> Outcome {
        let num_variables = system.iter().flat_map(
            |eqn| eqn.lhs.iter().map(
                |term| term.variable.idx)).max().unwrap() + 1;

        let mut variables: Vec<Option<&Variable>> = vec![None; num_variables];
        for eqn in system {
            for t in eqn.lhs.iter() {
                variables[t.variable.idx] = Some(t.variable);
            }
        }
        
        let mut matrix: Matrix = system.iter().map(|eqn| eqn_to_row(eqn, num_variables)).collect();
        reduced_row_echelon_form(&mut matrix);

        let mut solution: Vec<Option<Rational>> = vec![None; num_variables];
        for y in 0..matrix.len() {
            match classify_row(&matrix[y][0..num_variables]) {
                RowType::AllZero => {
                    if matrix[y][num_variables] != Rational::zero() {
                        return Outcome::Unsolvable;
                    }
                }
                RowType::OneOne(x) => {
                    solution[x] = Some(matrix[y][num_variables].clone());
                }
                _ => {
                    // Pass
                }
            }
        }

        if solution.iter().all(|s| s.is_some()) {
            return Outcome::Solution(solution.into_iter().map(|s|s.unwrap()).collect());
        }

        return Outcome::Underconstrained(solution);
    }
}

#[cfg(test)]
mod tests {
    use super::rationals::Rational;
    use super::rles::*;
    use crate::linear_equations::*;

    #[test]
    fn test_add() {
        assert_eq!(Rational::new(1, 3) + Rational::new(1, 4), Rational::new(7, 12));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Rational::new(1, 3) - Rational::new(1, 4), Rational::new(1, 12));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Rational::new(2, 3) * Rational::new(5, 4), Rational::new(10, 12));
    }

    #[test]
    fn test_div() {
        assert_eq!(Rational::new(2, 3) / Rational::new(5, 4), Rational::new(8, 15));
    }

    #[test]
    fn test_rre() {
        let mut matrix = vec![
            vec![Rational::int(2), Rational::int(6), Rational::int(2), Rational::int(4)],
            vec![Rational::int(1), Rational::int(3), Rational::int(1), Rational::int(2)],
            vec![Rational::int(1), Rational::int(3), Rational::int(2), Rational::int(4)],
        ];
        reduced_row_echelon_form(&mut matrix);

        let expected = vec![
            vec![Rational::int(1), Rational::int(3), Rational::int(0), Rational::int(0)],
            vec![Rational::int(0), Rational::int(0), Rational::int(1), Rational::int(2)],
            vec![Rational::int(0), Rational::int(0), Rational::int(0), Rational::int(0)],
        ];

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_solve() {
        let variables = vec![Variable{idx: 0, name: "x".to_owned()}, Variable{idx: 1, name: "y".to_owned()}];

        let system = vec![
            Equation {lhs: vec![Term {constant: Rational::int(2), variable: &variables[0]}], rhs: Rational::int(6)},
            Equation {lhs: vec![
                Term {constant: Rational::int(1), variable: &variables[0]},
                Term {constant: Rational::int(1), variable: &variables[1]},
                ], rhs: Rational::int(7)}
        ];

        let result = solve(&system);
        let expected = Outcome::Solution(vec![Rational::int(3), Rational::int(4)]);

        assert_eq!(result, expected);
    }
}

use linear_equations::*;
use rationals::Rational;
use rles::*;

fn make_equation<'a>(
    pa: &'a Variable, 
    pb: &'a Variable, 
    va: &'a Variable, 
    vb: &'a Variable, 
    pavb: &'a Variable, 
    pbva: &'a Variable, 
    pai: &Rational, 
    pbi: &Rational, 
    vai: &Rational, 
    vbi: &Rational) -> Equation<'a>
    {
    let one = Rational::int(1);
    let neg_one = Rational::int(-1);

    return Equation { 
        lhs: vec![
            Term::new(one, pavb),
            Term::new(-pai.clone(), vb),
            Term::new(-vbi.clone(), pa),

            Term::new(neg_one, pbva),
            Term::new(pbi.clone(), va),
            Term::new(vai.clone(), pb),
        ], 
        rhs: pbi.clone() * vai.clone() - pai.clone() * vbi.clone()
    };
}

fn main() {
    let content = fs::read_to_string("./src/input24.txt").unwrap();
    let stones: Vec<_> = content.lines().map(parse_stone).collect();

    let px = Variable { idx: 0, name: "px".to_owned() };
    let py = Variable { idx: 1, name: "py".to_owned() };
    let pz = Variable { idx: 2, name: "pz".to_owned() };
    let vx = Variable { idx: 3, name: "vx".to_owned() };
    let vy = Variable { idx: 4, name: "vy".to_owned() };
    let vz = Variable { idx: 5, name: "vz".to_owned() };

    // We can't represent these quantities using linear expressions of the variables, so we declare new variables.
    // If we are lucky, we can still solve. (Spoiler: we can)
    let pxvy = Variable { idx: 6, name: "pxvy".to_owned() };
    let pxvz = Variable { idx: 7, name: "pxvz".to_owned() };
    let pyvx = Variable { idx: 8, name: "pyvx".to_owned() };
    let pyvz = Variable { idx: 9, name: "pyvz".to_owned() };
    let pzvx = Variable { idx: 10, name: "pzvx".to_owned() };
    let pzvy = Variable { idx: 11, name: "pzvy".to_owned() };

    let mut system: Vec<Equation> = vec![];

    for stone in stones {
        let pxi = Rational::int(stone.position[0]);
        let pyi = Rational::int(stone.position[1]);
        let pzi = Rational::int(stone.position[2]);
        let vxi = Rational::int(stone.velocity[0]);
        let vyi = Rational::int(stone.velocity[1]);
        let vzi = Rational::int(stone.velocity[2]);
        
        system.push(make_equation(&px, &py, &vx,& vy, &pxvy, &pyvx, &pxi, &pyi, &vxi, &vyi));
        system.push(make_equation(&py, &pz, &vy,& vz, &pyvz, &pzvy, &pyi, &pzi, &vyi, &vzi));
        system.push(make_equation(&px, &pz, &vx,& vz, &pxvz, &pzvx, &pxi, &pzi, &vxi, &vzi));
    }

    let outcome = solve(&system);

    match outcome {
        // It is underconstrained due to the new variables, but the ones we need are defined.
        Outcome::Underconstrained(solution) => {
            let px_value = solution[px.idx].as_ref().unwrap().to_int().unwrap();
            let py_value = solution[py.idx].as_ref().unwrap().to_int().unwrap();
            let pz_value = solution[pz.idx].as_ref().unwrap().to_int().unwrap();
            let sum = px_value + py_value + pz_value;

            println!("px = {}, py = {}, pz = {}, sum = {}", px_value, py_value, pz_value, sum);
        }
        _ => panic!()
    }
}