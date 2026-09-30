pub trait TerminalLink<'a>: Sized {
    fn terminal_link(
        &'a self,
        url: &'a str,
    ) -> HasTerminalLink<'a, Self> {
        HasTerminalLink { data: self, url }
    }
}

impl<'a, T> TerminalLink<'a> for T {}

pub struct HasTerminalLink<'a, T: ?Sized> {
    data: &'a T,
    url: &'a str,
}

// Modified from owo_colors.
macro_rules! impl_fmt_for {
    ($($trait:path),* $(,)?) => {
        $(
            impl<'a, T: ?Sized + $trait> $trait for HasTerminalLink<'a, T> {
                #[inline(always)]
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    if ::supports_hyperlinks::supports_hyperlinks() {
                        f.write_str("\u{1b}]8;;")?;
                        f.write_str(self.url)?;
                        f.write_str("\u{1b}\\")?;
                        <T as $trait>::fmt(&self.data, f)?;
                        f.write_str("\u{1b}]8;;\u{1b}\\")
                    } else {
                        <T as $trait>::fmt(&self.data, f)
                    }
                }
            }
        )*
    };
}

impl_fmt_for! {
    std::fmt::Display,
    std::fmt::Debug,
    std::fmt::UpperHex,
    std::fmt::LowerHex,
    std::fmt::Binary,
    std::fmt::UpperExp,
    std::fmt::LowerExp,
    std::fmt::Octal,
    std::fmt::Pointer,
}
