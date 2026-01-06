// TODO: implement PartialOrd and PartialEq for all types

#[derive(Clone, Debug)]
pub enum RelativeLength {
    Cap(f64),
    Em(f64),
    Ex(f64),
    Ic(f64),
    Lh(f64),
}

#[derive(Clone, Debug)]
pub enum RelativeLengthBasedOnRoot {
    Rcap(f64),
    Rch(f64),
    Rem(f64),
    Rex(f64),
    Ric(f64),
    Rlh(f64),
}

#[derive(Clone, Debug)]
pub enum AbsoluteLength {
    Px(f64), // for print 1px = 1/96th of an inch
    Cm(f64), // 1cm = 96px / 2.54
    Q(f64),  // 1q = 1cm / 40
    In(f64), // 1in = 96px
    Mm(f64), // 1mm = 1cm / 10
    Pt(f64), // 1pt = 1/72th of an inch
    Pc(f64), // 1pc = 1/6th of an inch
}

#[derive(Clone, Debug)]
pub enum RelativeLengths {
    RelativeLength(RelativeLength),
    RelativeLengthBasedOnRoot(RelativeLengthBasedOnRoot),
}

#[derive(Clone, Debug)]
pub enum Length {
    Absoltue(AbsoluteLength),
    Relative(RelativeLengths),
}

#[derive(Clone, Debug)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(f64),
}
