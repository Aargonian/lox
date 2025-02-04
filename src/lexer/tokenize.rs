use super::scanner::LoxScanner;

pub trait IntoScanner<'a> {
    fn scan(self) -> LoxScanner<'a>;
}

impl<'a> IntoScanner<'a> for &'a str {
    fn scan(self) -> LoxScanner<'a> {
        LoxScanner::from_str(self)
    }
}

// impl<'a> IntoIterator for &'a str {
//     type Item = Result<LoxToken, LexerError>;
//     type IntoIter = LoxScanner<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
