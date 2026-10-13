use crate::common::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[\x1b[38;2;{};{};{}m\u{2588}\x1b[39m] ", self.r, self.g, self.b)?;
        // weird formatting I never remember the syntax of: #xxxxxxxx
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b, self.a)?;
        Ok(())
    }
}

impl From<u64> for Pixel {
    fn from(n: u64) -> Self {
        Pixel {
            r: ((n & 0xff000000) >> 24) as u8,
            g: ((n & 0x00ff0000) >> 16) as u8,
            b: ((n & 0x0000ff00) >> 8) as u8,
            a: ((n & 0x000000ff)) as u8,
        }
    }
}

#[derive(Debug)]
pub struct Sprite<'a> {
    pub fields: HashMap<&'a str, &'a str>,
    pub buffer: Box<[Box<[Pixel]>]>,
}

impl<'a> Sprite<'a> {
    pub fn parse(data: &'a str) -> anyhow::Result<Self> {
        // Very basic format inspired in HTTP.
        let (raw_fields, raw_buffer) = data.split_once("\n\n").unwrap_or(("", data));
        let fields = HashMap::from_iter(
            raw_fields.lines()
                // ~ shouldn't be neccessary but just in case:
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let (k, v) = line.split_once(':').unwrap_or((line, ""));
                    (k.trim(), v.trim())
                })
        );
        let mut buffer = vec![];
        for raw_buffer_line in raw_buffer.lines() {
            let mut buffer_line = vec![];
            for b in raw_buffer_line.bytes() {
                let px = match b {
                    b'B' => Pixel::from(0x111111ff), // black
                    b'D' => Pixel::from(0x404040ff), // dark gray
                    b'G' => Pixel::from(0x808080ff), // gray
                    b'L' => Pixel::from(0xbbbbbbff), // light gray
                    b'W' => Pixel::from(0xf0f0f0ff), // white
                    b'r' => Pixel::from(0xff5050ff), // red
                    b'o' => Pixel::from(0xffaa50ff), // orange
                    b'y' => Pixel::from(0xffff50ff), // yellow
                    b'l' => Pixel::from(0xaaff50ff), // lime
                    b'g' => Pixel::from(0x50ff50ff), // green
                    b't' => Pixel::from(0x50ffaaff), // turquoise
                    b'c' => Pixel::from(0x50ffffff), // cyan
                    b'd' => Pixel::from(0x50aaffff), // dodger blue
                    b'b' => Pixel::from(0x5050ffff), // blue
                    b'v' => Pixel::from(0xaa50ffff), // violet
                    b'm' => Pixel::from(0xff50ffff), // magenta
                    b'p' => Pixel::from(0xff50aaff), // pink
                    _ => Pixel::default(),
                };
                buffer_line.push(px);
            }
            buffer.push(buffer_line.into_boxed_slice());
        }
        let buffer = buffer.into_boxed_slice();
        Ok(Sprite { fields, buffer })
    }
}

impl fmt::Display for Sprite<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let mut row = 0;
        let mut buf = self.buffer.iter();
        while let Some(buf_line_hi) = buf.next() {
            let mut line = String::with_capacity(buf_line_hi.len() * 6);
            if let Some(buf_line_lo) = buf.next() {
                for pixels in
                    Itertools::zip_longest(
                        buf_line_hi.iter().copied(),
                        buf_line_lo.iter().copied(),
                    )
                {
                    match pixels {
                        | EitherOrBoth::Both(pixel_hi, Pixel { a: 0, .. })
                        | EitherOrBoth::Left(pixel_hi) => {
                            line.push_str(
                                format!(
                                    "\x1b[38;2;{};{};{}m{PIXEL_HI}\x1b[39m",
                                    pixel_hi.r,
                                    pixel_hi.g,
                                    pixel_hi.b,
                                ).as_str()
                            );
                        }
                        | EitherOrBoth::Both(Pixel { a: 0, .. }, pixel_lo)
                        | EitherOrBoth::Right(pixel_lo) => {
                            line.push_str(
                                format!(
                                    "\x1b[38;2;{};{};{}m{PIXEL_LO}\x1b[39m",
                                    pixel_lo.r,
                                    pixel_lo.g,
                                    pixel_lo.b,
                                ).as_str()
                            );
                        }
                        EitherOrBoth::Both(pixel_hi, pixel_lo) => {
                            line.push_str(
                                format!(
                                    "\x1b[38;2;{};{};{};48;2;{};{};{}m{PIXEL_HI}\x1b[49;39m",
                                    pixel_hi.r,
                                    pixel_hi.g,
                                    pixel_hi.b,
                                    pixel_lo.r,
                                    pixel_lo.g,
                                    pixel_lo.b,
                                ).as_str()
                            );
                        }
                    }
                    // row += 1;
                }
            } else {
                for pixel_hi in buf_line_hi.iter() {
                    line.push_str(
                        format!(
                            "\x1b[38;2;{};{};{}m{PIXEL_HI}\x1b[39m",
                            pixel_hi.r,
                            pixel_hi.g,
                            pixel_hi.b,
                        ).as_str()
                    );
                }
            }
            writeln!(f, "{line}\r")?;
        }
        Ok(())
    }
}
