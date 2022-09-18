use std::cmp::PartialEq;

pub struct Student {
    pub number: StudentId,
    name: StudentName,
    colors: StudentColors,
}

impl Student {
    pub fn new (number: StudentId, name: StudentName, colors: StudentColors) -> Self {
        Self {
            number,
            name,
            colors
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct StudentId(u16);

impl TryFrom<u16> for StudentId {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 1000 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<StudentId> for u16 {
    fn from(n: StudentId) -> u16 {
        n.0
    }
}


pub struct StudentName(String);

impl TryFrom<String> for StudentName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}


pub struct StudentColors(Vec<StudentColor>);

impl TryFrom<Vec<String>> for StudentColors {
    type Error = ();

    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            Err(())
        } else {
            let mut pts = vec![];
            for t in ts.iter() {
                match StudentColor::try_from(String::from(t)) {
                    Ok(pt) => pts.push(pt),
                    _ => return Err(()),
                }
            }
            Ok(Self(pts))
        }
    }
}

enum StudentColor {
    Blue,
    Red,
}

impl TryFrom<String> for StudentColor {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Blue" => Ok(Self::Blue),
            "Red" => Ok(Self::Red),
            _ => Err(()),
        }
    }
}