pub fn egui_svg_path(
    path: &str,
    src_rect: egui::Rect,
    dst_rect: egui::Rect,
    stroke_style: egui::Stroke,
) -> impl Iterator<Item = egui::epaint::Shape> + '_ {
    let to_point = move |(x, y): (f32, f32), cursor: egui::Pos2, absolute: bool| -> egui::Pos2 {
        if absolute {
            egui::pos2(
                egui::remap(x, src_rect.x_range(), dst_rect.x_range()),
                egui::remap(y, src_rect.y_range(), dst_rect.y_range()),
            )
        } else {
            egui::pos2(
                cursor.x + x * dst_rect.width() / src_rect.width(),
                cursor.y + y * dst_rect.width() / src_rect.width(),
            )
        }
    };

    let mut path_start = egui::Pos2::ZERO;
    let mut cursor = egui::Pos2::ZERO;
    let mut next_control_point = egui::Pos2::ZERO;

    let mut path = svgtypes::PathParser::from(path);
    std::iter::from_fn(move || loop {
        let Some(Ok(cmd)) = path.next() else {
            return None;
        };

        match cmd {
            svgtypes::PathSegment::MoveTo { abs, x, y } => {
                cursor = to_point((x as f32, y as f32), cursor, abs);
                path_start = cursor;
            }
            svgtypes::PathSegment::ClosePath { abs: _ } => {
                let from = cursor;
                let to = path_start;
                cursor = path_start;
                return Some(egui::epaint::Shape::line_segment([from, to], stroke_style));
            }
            svgtypes::PathSegment::LineTo { abs, x, y } => {
                let from = cursor;
                let to = to_point((x as f32, y as f32), cursor, abs);
                cursor = to;
                return Some(egui::epaint::Shape::line_segment([from, to], stroke_style));
            }
            svgtypes::PathSegment::HorizontalLineTo { abs, x } => {
                let from = cursor;
                let to = egui::pos2(to_point((x as f32, 0.0), cursor, abs).x, cursor.y);
                cursor = to;
                return Some(egui::epaint::Shape::line_segment([from, to], stroke_style));
            }
            svgtypes::PathSegment::VerticalLineTo { abs, y } => {
                let from = cursor;
                let to = egui::pos2(cursor.x, to_point((0.0, y as f32), cursor, abs).y);
                cursor = to;
                return Some(egui::epaint::Shape::line_segment([from, to], stroke_style));
            }
            #[rustfmt::skip]
            svgtypes::PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y, } => {
                let start = cursor;
                let c1 = to_point((x1 as f32, y1 as f32), cursor, abs);
                let c2 = to_point((x2 as f32, y2 as f32), cursor, abs);
                let end = to_point((x as f32, y as f32), cursor, abs);

                cursor = end;
                next_control_point = end + (c2 - end);
                return Some(egui::epaint::Shape::CubicBezier(egui::epaint::CubicBezierShape::from_points_stroke(
                    [start, c1, c2, end],
                    false,
                    egui::Color32::TRANSPARENT,
                    stroke_style,
                )));
            }
            svgtypes::PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                let start = cursor;
                let c1 = next_control_point;
                let c2 = to_point((x2 as f32, y2 as f32), cursor, abs);
                let end = to_point((x as f32, y as f32), cursor, abs);
                cursor = end;
                next_control_point = end + (c2 - end);
                return Some(egui::epaint::Shape::CubicBezier(
                    egui::epaint::CubicBezierShape::from_points_stroke(
                        [start, c1, c2, end],
                        false,
                        egui::Color32::TRANSPARENT,
                        stroke_style,
                    ),
                ));
            }
            svgtypes::PathSegment::Quadratic { abs, x1, y1, x, y } => {
                let start = cursor;
                let c1 = to_point((x1 as f32, y1 as f32), cursor, abs);
                let end = to_point((x as f32, y as f32), cursor, abs);
                cursor = end;
                next_control_point = end + (c1 - end);
                return Some(egui::epaint::Shape::QuadraticBezier(
                    egui::epaint::QuadraticBezierShape::from_points_stroke(
                        [start, c1, end],
                        false,
                        egui::Color32::TRANSPARENT,
                        stroke_style,
                    ),
                ));
            }
            svgtypes::PathSegment::SmoothQuadratic { abs, x, y } => {
                let start = cursor;
                let c1 = next_control_point;
                let end = to_point((x as f32, y as f32), cursor, abs);
                cursor = end;
                next_control_point = end + (c1 - end);
                return Some(egui::epaint::Shape::QuadraticBezier(
                    egui::epaint::QuadraticBezierShape::from_points_stroke(
                        [start, c1, end],
                        false,
                        egui::Color32::TRANSPARENT,
                        stroke_style,
                    ),
                ));
            }
            svgtypes::PathSegment::EllipticalArc { .. } => {
                panic!("EGUI does not support elliptical arcs");
            }
        }
    })
}

pub fn cut_at_length(
    len: f32,
    shapes: impl Iterator<Item = egui::Shape>,
) -> impl Iterator<Item = egui::Shape> {
    let mut remaining = len;
    shapes.map_while(move |shape| {
        if remaining <= 0.0 {
            return None;
        }

        let shape_len = match shape {
            egui::Shape::LineSegment {
                points: [from, to], ..
            } => from.distance(to),
            egui::Shape::CubicBezier(shape) => shape.points[0].distance(shape.points[3]),
            egui::Shape::QuadraticBezier(shape) => shape.points[0].distance(shape.points[2]),
            _ => return Some(shape),
        };

        if remaining > shape_len {
            remaining -= shape_len;
            return Some(shape);
        }

        let t = remaining / shape_len;
        remaining = -1.0;
        match shape {
            egui::Shape::LineSegment {
                points: [from, to],
                stroke,
            } => {
                let mid = from + (to - from) * t;
                Some(egui::Shape::line_segment([from, mid], stroke))
            }
            egui::Shape::CubicBezier(shape) => Some(split_cubic_bezier(t, shape).0.into()),
            egui::Shape::QuadraticBezier(shape) => {
                let (left, _) = split_quadratic_bezier(t, shape);
                Some(left.into())
            }
            _ => unreachable!(),
        }
    })
}

/// De Casteljau's algorithm
pub fn split_cubic_bezier(
    t: f32,
    shape: egui::epaint::CubicBezierShape,
) -> (
    egui::epaint::CubicBezierShape,
    egui::epaint::CubicBezierShape,
) {
    let invt = 1.0 - t;

    let a = shape.points[0].to_vec2();
    let b = shape.points[1].to_vec2();
    let c = shape.points[2].to_vec2();
    let d = shape.points[3].to_vec2();

    let a1 = a * invt + b * t;
    let a2 = b * invt + c * t;
    let a3 = c * invt + d * t;
    let abc = a1 * invt + a2 * t;
    let bcd = a2 * invt + a3 * t;
    let abcd = abc * invt + bcd * t;

    (
        egui::epaint::CubicBezierShape::from_points_stroke(
            [a.to_pos2(), a1.to_pos2(), abc.to_pos2(), abcd.to_pos2()],
            false,
            shape.fill,
            shape.stroke,
        ),
        egui::epaint::CubicBezierShape::from_points_stroke(
            [abcd.to_pos2(), bcd.to_pos2(), a3.to_pos2(), d.to_pos2()],
            false,
            shape.fill,
            shape.stroke,
        ),
    )
}

/// De Casteljau's algorithm
pub fn split_quadratic_bezier(
    t: f32,
    shape: egui::epaint::QuadraticBezierShape,
) -> (
    egui::epaint::QuadraticBezierShape,
    egui::epaint::QuadraticBezierShape,
) {
    let invt = 1.0 - t;

    let a = shape.points[0].to_vec2();
    let b = shape.points[1].to_vec2();
    let c = shape.points[2].to_vec2();

    let a1 = a * invt + b * t;
    let a2 = b * invt + c * t;
    let ab = a1 * invt + a2 * t;

    (
        egui::epaint::QuadraticBezierShape::from_points_stroke(
            [a.to_pos2(), a1.to_pos2(), ab.to_pos2()],
            false,
            shape.fill,
            shape.stroke,
        ),
        egui::epaint::QuadraticBezierShape::from_points_stroke(
            [ab.to_pos2(), a2.to_pos2(), c.to_pos2()],
            false,
            shape.fill,
            shape.stroke,
        ),
    )
}
