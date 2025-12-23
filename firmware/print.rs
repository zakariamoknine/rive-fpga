use crate::uart8250::{putc};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print!("{}\n", format_args!($($arg)*));
    })
}

pub(crate) struct Print;

impl core::fmt::Write for Print {
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        for c in s.as_bytes() {
            unsafe {
                if *c == b'\n' {
                    putc(b'\r');
                }

                putc(*c);
            }
        }
        Ok(())
    }
}

pub(crate) fn _print(args: core::fmt::Arguments)
{
    use core::fmt::Write;

    let mut print_dummy = Print {};

    // To avoid to calling unwrap
    let _ = print_dummy.write_fmt(args);
}
