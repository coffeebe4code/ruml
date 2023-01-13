use std::fmt::Display;

//pub trait Tag: Display {
//    fn start_open(&self, attrs: bool) -> String {
//            format_args!("<{} ", self)
//        } else {
//            format_args!("<{}", self)
//        }
//    }
//    fn start_close() -> &'static str {
//        ">"
//    }
//    fn end(&self) -> fmt::Arguments {
//        format_args!("</{}>", self)
//    }
//}
//
pub trait Tag: Display + 'static {}

pub trait TagValue: ToString {
    fn render(&self) -> String {
        return self.to_string();
    }
}
