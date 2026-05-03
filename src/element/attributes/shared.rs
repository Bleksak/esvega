use std::fmt;

pub fn write_semicolon_separated<W, I>(f: &mut W, iter: I) -> fmt::Result
where
    W: fmt::Write,
    I: Iterator,
    I::Item: fmt::Display,
{
    write!(f, "=\"")?;
    for (i, item) in iter.enumerate() {
        if i > 0 {
            write!(f, ";")?;
        }
        write!(f, "{}", item)?;
    }
    write!(f, "\"")
}

pub fn write_space_separated<W, I>(f: &mut W, iter: I) -> fmt::Result
where
    W: fmt::Write,
    I: Iterator,
    I::Item: fmt::Display,
{
    write!(f, "=\"")?;
    for (i, item) in iter.enumerate() {
        if i > 0 {
            write!(f, " ")?;
        }
        write!(f, "{}", item)?;
    }
    write!(f, "\"")
}

pub fn write_comma_separated<W, I>(f: &mut W, iter: I) -> fmt::Result
where
    W: fmt::Write,
    I: Iterator,
    I::Item: fmt::Display,
{
    write!(f, "=\"")?;
    for (i, item) in iter.enumerate() {
        if i > 0 {
            write!(f, ",")?;
        }
        write!(f, "{}", item)?;
    }
    write!(f, "\"")
}
