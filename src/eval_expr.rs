use crate::expr::Expr;
use rand_distr::*;

impl Expr {
    pub fn eval_once<R>(&self, rng: &mut R) -> f64
    where
        R: rand::Rng + ?Sized,
    {
        match self {
            Expr::Atom(val) => *val,
            Expr::Clip { val, min, max } => {
                let val = val.eval_once(rng);
                let min = min.eval_once(rng);
                let max = max.eval_once(rng);
                val.min(min).max(max)
            }
            Expr::Min { a, b } => {
                let a = a.eval_once(rng);
                let b = b.eval_once(rng);
                a.min(b)
            }
            Expr::Max { a, b } => {
                let a = a.eval_once(rng);
                let b = b.eval_once(rng);
                a.max(b)
            }
            Expr::Norm { mean, std } => {
                let mean = mean.eval_once(rng);
                let mut std = std.eval_once(rng);
                if !std.is_finite(){
                    std = 1.;
                }
                let x = Normal::new(mean, std).unwrap();
                x.sample(rng)
            }
            Expr::Skewnorm { loc, scale, shape } => {
                let loc = loc.eval_once(rng);
                let mut scale = scale.eval_once(rng);
                let mut shape = shape.eval_once(rng);
                if scale <= 0.{
                    scale = 1.;
                }
                if !scale.is_finite(){
                    scale = 1.;
                }
                if !shape.is_finite(){
                    shape = 1.;
                }
                let x = SkewNormal::new(loc, scale, shape).unwrap();
                x.sample(rng)
            }
            Expr::Logn { base, arg } => {
                let base = base.eval_once(rng);
                let arg = arg.eval_once(rng);
                arg.log(base)
            }
            Expr::Log2 { arg } => {
                let arg = arg.eval_once(rng);
                arg.log2()
            }
            Expr::Ln { arg } => {
                let arg = arg.eval_once(rng);
                arg.ln()
            }
            Expr::Abs { val } => {
                let val = val.eval_once(rng);
                val.abs()
            }
            Expr::Sqrt { val } => {
                let val = val.eval_once(rng);
                val.sqrt()
            }
            Expr::Neg { val } => {
                let val = val.eval_once(rng);
                -val
            }
            Expr::Add { a, b } => {
                let a = a.eval_once(rng);
                let b = b.eval_once(rng);
                a + b
            }
            Expr::Mul { a, b } => {
                let a = a.eval_once(rng);
                let b = b.eval_once(rng);
                a * b
            }
            Expr::Sub { left, right } => {
                let left = left.eval_once(rng);
                let right = right.eval_once(rng);
                left - right
            }
            Expr::Div { left, right } => {
                let left = left.eval_once(rng);
                let right = right.eval_once(rng);
                left / right
            }
            Expr::Pow { base, exp } => {
                let base = base.eval_once(rng);
                let exp = exp.eval_once(rng);
                base.powf(exp)
            }
        }
    }
}
