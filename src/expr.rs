type BE = Box<Expr>;

///For functions whose parameters have restrictions (e.g.: for SkewNormal/skewnorm, shape should be
///larger than zero and finite), they will have implicit "decorations" (i.e.: input will be changed
///if it does not fit)
///
///For Skwn, scale is handled such that it will be 1.0 if the input is -0.3
#[derive(Debug, Clone)]
pub enum Expr {
    Atom(f64),

    Clip { val: BE, min: BE, max: BE },
    Min { a: BE, b: BE },
    Max { a: BE, b: BE },
    Norm { mean: BE, std: BE },
    Skewnorm { loc: BE, scale: BE, shape: BE },
    Logn { base: BE, arg: BE },
    Log2 { arg: BE },
    Ln { arg: BE },
    Abs { val: BE },
    Sqrt { val: BE },

    Neg { val: BE },
    Add { a: BE, b: BE },
    Mul { a: BE, b: BE },
    Sub { left: BE, right: BE },
    Div { left: BE, right: BE },
    Pow { base: BE, exp: BE },
}
