use crate::letters::EARIOTNSLCUDPMHGBFYWKVXZJQ;

pub struct Word<'a> {
    pub str_repr: &'a str,
    pub int_repr: u32
}

impl<'a> Word<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut i = 0;
        if !s.is_ascii() { panic!("{s} is not valid ASCII.") }
        let sc = s.chars();
        for c in sc {
            let ascii = c as u32 - 97;
            i |= EARIOTNSLCUDPMHGBFYWKVXZJQ[ascii as usize];
        }
        Self {
            str_repr: s,
            int_repr: i

        }
    }
}

impl std::fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str_repr)
    }
}

impl std::fmt::Debug for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:#034b}", self.str_repr, self.int_repr)
    }
}

impl std::ops::BitAnd for Word<'_> {
    type Output = u32;
    fn bitand(self, rhs: Self) -> Self::Output{
        self.int_repr & rhs.int_repr
    }
}

impl std::ops::BitAnd for &Word<'_> {
    type Output = u32;
    fn bitand(self, rhs: Self) -> Self::Output{
        self.int_repr & rhs.int_repr
    }
}