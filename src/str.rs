#[doc(hidden)]
pub use crate::write::compile_time::Buffer;

#[doc(hidden)]
pub const fn len_as_ansi_bytes(buf: &Buffer<[u8;25]>) -> usize {
    let mut result: usize = 0;
    let mut i: usize = 0;
    while i < buf.len {
        result += number_of_digits(buf.array[i]);
        i += 1;
    }
    if buf.len > 0 {
        result += "\x1B[".len();
        result += (buf.len - 1) * ";".len();
        result += "m".len();
    }
    result
}

#[doc(hidden)]
pub const fn to_ansi_bytes<const N: usize>(buf: &Buffer<[u8;25]>) -> [u8; N] {
    let mut writer = AnsiWriter::<N>::new();
    let mut i: usize = 0;
    if buf.len > 0 { writer = writer.write_str("\x1B["); }
    while i < buf.len {
        if i > 0 { writer = writer.write_str(";"); }
        writer = writer.write_digits(buf.array[i]);
        i += 1;
    }
    if buf.len > 0 { writer = writer.write_str("m"); }
    writer.take().array
}

const fn number_of_digits(mut value: u8) -> usize {
    let mut len: usize = 1;
    while value > 9 {
        value = value / 10;
        len += 1;
    }
    len
}

struct AnsiWriter<const N: usize> { state: Buffer<[u8; N]> }

impl<const N: usize> AnsiWriter<N> {
    const fn new() -> Self { Self { state: Buffer { array: [0u8; N], len: 0 } } }

    const fn write_str(mut self, value: &'static str) -> Self {
        let bytes = value.as_bytes();
        let mut i = 0usize;
        loop {
            if i == value.len() { break }
            self.state.array[self.state.len + i] = bytes[i];
            i += 1;
        }
        self.state.len += value.len();
        self
    }

    const fn write_digits(mut self, mut value: u8) -> Self {
        let number_of_digits = number_of_digits(value);
        let mut i = 0usize;
        loop {
            let digit = value % 10;
            self.state.array[self.state.len + number_of_digits - 1 - i] = b'0' + digit as u8;
            value = value / 10;
            i += 1;
            if i == number_of_digits { break }
        }
        self.state.len += number_of_digits;
        self
    }

    const fn take(self) -> Buffer<[u8; N]> { self.state }
}
