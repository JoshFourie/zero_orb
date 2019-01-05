use zksnark::field::{Polynomial, Field};
use zksnark::groth16::{
    QAP, 
    coefficient_poly::{CoefficientPoly, root_poly},
    circuit::{RootRepresentation, dummy_rep::DummyRep}
};
use std::vec::IntoIter;
use serde_derive::{Serialize, Deserialize};

// zksnark-rs does not permit QAP generation generically from a custom field.
// the WrappedDumyRep and WrappedQAP newtypes permit generic conversion.
// note that z655 is currently deprecated in favour of FrLocal.

#[derive(Serialize, Deserialize)]
pub struct WrappedDummyRep<F>(pub DummyRep<F>);

impl<'a, F> From<&'a str> for WrappedDummyRep<F> 
    where F: Field + From<usize>
{
    fn from(code: &'a str) -> Self {
        let mut line_count = 0;
        let mut lines = code.lines();
        let inputs = lines.next().unwrap().split(' ').collect::<Vec<_>>();
        let witness = lines.next().unwrap().split(' ').collect::<Vec<_>>();
        let temp_vars = lines.next().unwrap().split(' ').collect::<Vec<_>>();
        lines.next();

        let num_vars = inputs.len() + witness.len() + temp_vars.len() + 1;
        let mut u: Vec<Vec<(F, F)>> = vec![Vec::new(); num_vars];
        let mut v: Vec<Vec<(F, F)>> = vec![Vec::new(); num_vars];
        let mut w: Vec<Vec<(F, F)>> = vec![Vec::new(); num_vars];

        for (n, line) in lines.enumerate() {
            line_count += 1;

            let mut symbols = line.split(' ');
            let first = symbols.next().unwrap();
            let pos = inputs
                .clone()
                .into_iter()
                .chain(
                    witness
                        .clone()
                        .into_iter()
                        .chain(temp_vars.clone().into_iter()),
                )
                .position(|s| s == first)
                .unwrap() + 1;

            w[pos].push(((n + 1).into(), 1.into()));
            symbols.next();

            let left = symbols
                .by_ref()
                .take_while(|&c| c != ")")
                .collect::<Vec<_>>();

            for l in left {
                if l == "1" {
                    u[0].push(((n + 1).into(), 1.into()));
                } else {
                    let pos = inputs
                        .clone()
                        .into_iter()
                        .chain(
                            witness
                                .clone()
                                .into_iter()
                                .chain(temp_vars.clone().into_iter()),
                        )
                        .position(|s| s == l)
                        .unwrap() + 1;

                    u[pos].push(((n + 1).into(), 1.into()));
                }
            }
            symbols.next();

            let right = symbols.take_while(|&c| c != ")").collect::<Vec<_>>();

            for r in right {
                let pos = inputs
                    .clone()
                    .into_iter()
                    .chain(
                        witness
                            .clone()
                            .into_iter()
                            .chain(temp_vars.clone().into_iter()),
                    )
                    .position(|s| s == r)
                    .unwrap() + 1;

                v[pos].push(((n + 1).into(), 1.into()));
            }
        }
        Self(
            DummyRep {
                u,
                v,
                w,
                roots: (1..line_count + 1).map(|n| n.into()).collect::<Vec<_>>(),
                input: inputs.len(),
            }
        )
    }
}

impl<F: Field> RootRepresentation<F> for WrappedDummyRep<F> {
    type Row = IntoIter<Self::Column>;
    type Column = IntoIter<(F, F)>;
    type Roots = IntoIter<F>;

    fn u(&self) -> Self::Row {
        self.0.u
            .clone()
            .into_iter()
            .map(|x| x.into_iter())
            .collect::<Vec<_>>()
            .into_iter()
    }
    fn v(&self) -> Self::Row {
        self.0.v
            .clone()
            .into_iter()
            .map(|x| x.into_iter())
            .collect::<Vec<_>>()
            .into_iter()
    }
    fn w(&self) -> Self::Row {
        self.0.w
            .clone()
            .into_iter()
            .map(|x| x.into_iter())
            .collect::<Vec<_>>()
            .into_iter()
    }
    fn roots(&self) -> Self::Roots {
        self.0.roots.clone().into_iter()
    }
    fn input(&self) -> usize {
        self.0.input
    }
}

#[derive(Serialize, Deserialize)]
pub struct WrappedQAP<F> (pub QAP<CoefficientPoly<F>>);

impl<F> From<WrappedDummyRep<F>> for WrappedQAP<F>
where
    F: Field
    + From<usize>
{
    fn from(root_rep: WrappedDummyRep<F>) -> Self {
        let (mut u, mut v, mut w) = (Vec::new(), Vec::new(), Vec::new());

        for points in root_rep.u() {
            u.push(CoefficientPoly::from((root_rep.roots(), points)));
        }
        for points in root_rep.v() {
            v.push(CoefficientPoly::from((root_rep.roots(), points)));
        }
        for points in root_rep.w() {
            w.push(CoefficientPoly::from((root_rep.roots(), points)));
        }

        assert_eq!(u.len(), v.len());
        assert_eq!(u.len(), w.len());

        let t = root_poly(root_rep.roots());
        let input = root_rep.input();
        let degree = t.degree();

        Self(
            QAP {
                u,
                v,
                w,
                t,
                input,
                degree,
            }
        )
    }
}
