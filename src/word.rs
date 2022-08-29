pub struct Word<'a> {
    pub str_repr: &'a str,
    pub int_repr: i32
}

impl<'a> Word<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut i = 0;
        if !s.is_ascii() { panic!("{s} is not valid ASCII.") }
        let sc = s.chars();
        for c in sc {
            let ascii = c as i32 - 97;
            i |= 1 << (31 - ascii);
        }
        Self {
            str_repr: s,
            int_repr: i

        }
    }
}

impl std::fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:#034b}", self.str_repr, self.int_repr)
    }
}