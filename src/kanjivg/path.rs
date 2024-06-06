use std::str::FromStr;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Command {
    MoveTo(Coord),
    LineTo(Coord),
    CubicBezier(Coord, Coord, Coord),
    CubicSpline(Coord, Coord),
    QuadBezier(Coord, Coord),
    QuadSpline(Coord),
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}
impl Coord {
    fn distance(&self, other: &Coord) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Path(pub Vec<Command>);
impl Path {
    /// Parse an SVG path data string into a Path.
    /// Example:
    /// ```
    /// use jdict2::kanjivg::path::Path;
    /// let path = Path::parse_from_svg_path_data(
    ///     "M99.22,15.85c-4.09,15.34-17.51,64.94-20.89,77.06",
    ///     (0.0, 0.0, 200.0, 200.0),
    /// );
    /// ```
    pub fn parse_from_svg_path_data(d: &str, view_box: (f32, f32, f32, f32)) -> Self {
        // println!("d: {}", d);

        let mut path_builder = PathBuilder {
            path: Path::default(),
            _last_control: (0.0, 0.0),
            pos: (0.0, 0.0),
            viewbox: view_box,
        };

        let mut d = d;
        while !d.is_empty() {
            d = path_builder.parse_action(d);
        }

        path_builder.path
    }

    pub fn length(&self) -> f32 {
        let mut length = 0.0;
        let mut brush_position = Coord::default();

        for cmd in self.0.iter() {
            match cmd {
                Command::MoveTo(to) | Command::LineTo(to) => {
                    brush_position = *to;
                }
                Command::CubicBezier(c1, c2, to) => {
                    length += brush_position.distance(c1);
                    length += c1.distance(c2);
                    length += c2.distance(to);
                    brush_position = *to;
                }
                Command::CubicSpline(c2, to) => {
                    length += brush_position.distance(c2);
                    length += c2.distance(to);
                    brush_position = *to;
                }
                Command::QuadBezier(c1, to) => {
                    length += brush_position.distance(c1);
                    length += c1.distance(to);
                    brush_position = *to;
                }
                Command::QuadSpline(to) => {
                    length += brush_position.distance(to);
                    brush_position = *to;
                }
            }
        }

        length
    }
}

struct PathBuilder {
    path: Path,
    _last_control: (f32, f32),
    pos: (f32, f32),
    viewbox: (f32, f32, f32, f32),
}
impl PathBuilder {
    fn move_to_internal(&mut self, relative: bool, coord: (f32, f32)) -> Coord {
        self.pos = if !relative {
            coord
        }
        else {
            (self.pos.0 + coord.0, self.pos.1 + coord.1)
        };

        let (minx, miny, width, height) = self.viewbox;
        Coord {
            x: ((self.pos.0 - minx) / width),
            y: ((self.pos.1 - miny) / height),
        }
    }
    fn get_coord_internal(&self, relative: bool, coord: (f32, f32)) -> Coord {
        let position = if !relative {
            coord
        }
        else {
            (self.pos.0 + coord.0, self.pos.1 + coord.1)
        };

        let (minx, miny, width, height) = self.viewbox;
        Coord {
            x: ((position.0 - minx) / width),
            y: ((position.1 - miny) / height),
        }
    }

    fn move_to(&mut self, relative: bool, to: (f32, f32)) {
        let action = Command::MoveTo(self.move_to_internal(relative, to));
        self.path.0.push(action);
    }

    fn line_to(&mut self, relative: bool, to: (f32, f32)) {
        let action = Command::LineTo(self.move_to_internal(relative, to));
        self.path.0.push(action);
    }

    // C
    fn cubic_bezier(&mut self, relative: bool, c1: (f32, f32), c2: (f32, f32), to: (f32, f32)) {
        let c1 = self.get_coord_internal(relative, c1);
        let c2 = self.get_coord_internal(relative, c2);
        let to = self.move_to_internal(relative, to);
        self.path.0.push(Command::CubicBezier(c1, c2, to));
    }

    // S
    fn cubic_spline(&mut self, relative: bool, c2: (f32, f32), to: (f32, f32)) {
        let c2 = self.get_coord_internal(relative, c2);
        let to = self.move_to_internal(relative, to);
        self.path.0.push(Command::CubicSpline(c2, to));
    }

    // Q
    fn quad_bezier(&mut self, relative: bool, c1: (f32, f32), to: (f32, f32)) {
        let c1 = self.get_coord_internal(relative, c1);
        let to = self.move_to_internal(relative, to);
        self.path.0.push(Command::QuadBezier(c1, to));
    }

    // T
    fn quad_spline(&mut self, relative: bool, to: (f32, f32)) {
        let to = self.move_to_internal(relative, to);
        self.path.0.push(Command::QuadSpline(to));
    }

    fn parse_action<'a>(&mut self, mut s: &'a str) -> &'a str {
        // println!("s: {}", s);
        let cmd;
        (s, cmd) = snip(s, |c| c.is_ascii_alphabetic());
        (s, _) = snip_whitespace_or_comma(s);
        assert_eq!(
            cmd.len(),
            1,
            "Unexpected path action: '{}' followed by '{}'",
            cmd,
            s
        );
        let cmd = cmd.as_bytes()[0];
        let relative = cmd.is_ascii_lowercase();

        match cmd {
            b'M' | b'm' => loop {
                let c;
                (s, c) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                self.move_to(relative, c);

                if !s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+' || c == '.')
                {
                    break;
                }
            },
            b'L' | b'l' => loop {
                let c;
                (s, c) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                self.line_to(relative, c);

                if !s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+' || c == '.')
                {
                    break;
                }
            },
            b'C' | b'c' => loop {
                let c1;
                let c2;
                let to;

                (s, c1) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                (s, c2) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                (s, to) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                self.cubic_bezier(relative, c1, c2, to);

                if !s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+' || c == '.')
                {
                    break;
                }
            },
            b'S' | b's' => loop {
                let c1;
                let to;

                (s, c1) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                (s, to) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                self.cubic_spline(relative, c1, to);

                if !s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+' || c == '.')
                {
                    break;
                }
            },
            b'Q' | b'q' => loop {
                let c1;
                let to;
                (s, c1) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                (s, to) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                self.quad_bezier(relative, c1, to);

                if !s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+' || c == '.')
                {
                    break;
                }
            },
            b'T' | b't' => loop {
                let c2;
                (s, c2) = snip_xy(s);
                (s, _) = snip_whitespace_or_comma(s);
                self.quad_spline(relative, c2);

                if !s.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '+' || c == '.')
                {
                    break;
                }
            },
            _ => panic!(
                "Unexpected path action: {} {}",
                char::from_u32(cmd as u32).unwrap(),
                s
            ),
        }

        s
    }
}

fn snip(s: &str, p: impl Fn(char) -> bool) -> (&str, &str) {
    let end = s.find(|c| !p(c)).unwrap_or(s.len());
    let result = &s[..end];
    let remainder = &s[end..];
    (remainder, result)
}
fn snip_sign(s: &str) -> (&str, f32) {
    let (s, sign) = snip(s, |c| c == '-' || c == '+');
    let sign = if sign.contains('-') { -1.0 } else { 1.0 };
    (s, sign)
}
fn snip_number(s: &str) -> (&str, f32) {
    let (s, sign) = snip_sign(s);
    let (s, num) = snip(s, |c| c.is_ascii_digit() || c == '.');
    (s, sign * f32::from_str(num).unwrap())
}
fn snip_whitespace_or_comma(s: &str) -> (&str, &str) {
    snip(s, |c| char::is_ascii_whitespace(&c) || c == ',')
}
fn snip_xy(s: &str) -> (&str, (f32, f32)) {
    let (s, x) = snip_number(s);
    let (s, _) = snip(s, |c| c.is_whitespace() || c == ',');
    let (s, y) = snip_number(s);

    (s, (x, y))
}
