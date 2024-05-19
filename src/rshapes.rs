use crate::Color;
use crate::Rectangle;
use crate::Texture2D;
use crate::Vector2;

/// Set texture and rectangle to be used on shapes drawing
pub fn set_shapes_texture(texture: Texture2D, source: Rectangle) {
    unsafe {
        raylib_ffi::SetShapesTexture(texture, source);
    }
}

/// Draw a pixel
pub fn draw_pixel(pos_x: i32, pos_y: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawPixel(pos_x, pos_y, color);
    }
}

/// Draw a pixel (Vector version)
pub fn draw_pixel_v(position: Vector2, color: Color) {
    unsafe {
        raylib_ffi::DrawPixelV(position, color);
    }
}

/// Draw a line
pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
    }
}

/// Draw a line (using gl lines)
pub fn draw_line_v(start_pos: Vector2, end_pos: Vector2, color: Color) {
    unsafe {
        raylib_ffi::DrawLineV(start_pos, end_pos, color);
    }
}

/// Draw a line (using triangles/quads)
pub fn draw_line_ex(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawLineEx(start_pos, end_pos, thick, color);
    }
}

/// Draw lines sequence (using gl lines)
pub fn draw_line_strip(points: &[Vector2], color: Color) {
    unsafe {
        raylib_ffi::DrawLineStrip(points.as_ptr() as *mut Vector2, points.len() as i32, color);
    }
}

/// Draw line segment cubic-bezier in-out interpolation
pub fn draw_line_bezier(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawLineBezier(start_pos, end_pos, thick, color);
    }
}

/// Draw a color-filled circle
pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawCircle(center_x, center_y, radius, color);
    }
}

/// Draw a piece of a circle
pub fn draw_circle_sector(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawCircleSector(center, radius, start_angle, end_angle, segments, color);
    }
}

/// Draw circle sector outline
pub fn draw_circle_sector_lines(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawCircleSectorLines(center, radius, start_angle, end_angle, segments, color);
    }
}

/// Draw a gradient-filled circle
pub fn draw_circle_gradient(
    center_x: i32,
    center_y: i32,
    radius: f32,
    color1: Color,
    color2: Color,
) {
    unsafe {
        raylib_ffi::DrawCircleGradient(center_x, center_y, radius, color1, color2);
    }
}

/// Draw a color-filled circle (Vector version)
pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawCircleV(center, radius, color);
    }
}

/// Draw circle outline
pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawCircleLines(center_x, center_y, radius, color);
    }
}

/// Draw circle outline (Vector version)
pub fn draw_circle_lines_v(center: Vector2, radius: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawCircleLinesV(center, radius, color);
    }
}

/// Draw ellipse
pub fn draw_ellipse(center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawEllipse(center_x, center_y, radius_h, radius_v, color);
    }
}

/// Draw ellipse outline
pub fn draw_ellipse_lines(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawEllipseLines(center_x, center_y, radius_h, radius_v, color);
    }
}

/// Draw ring
pub fn draw_ring(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawRing(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        );
    }
}

/// Draw ring outline
pub fn draw_ring_lines(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawRingLines(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        );
    }
}

/// Draw a color-filled rectangle
pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawRectangle(pos_x, pos_y, width, height, color);
    }
}

/// Draw a color-filled rectangle (Vector version)
pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
    unsafe {
        raylib_ffi::DrawRectangleV(position, size, color);
    }
}

/// Draw a color-filled rectangle
pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    unsafe {
        raylib_ffi::DrawRectangleRec(rec, color);
    }
}

/// Draw a color-filled rectangle with pro parameters
pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawRectanglePro(rec, origin, rotation, color);
    }
}

/// Draw a vertical-gradient-filled rectangle
pub fn draw_rectangle_gradient_v(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color1: Color,
    color2: Color,
) {
    unsafe {
        raylib_ffi::DrawRectangleGradientV(pos_x, pos_y, width, height, color1, color2);
    }
}

/// Draw a horizontal-gradient-filled rectangle
pub fn draw_rectangle_gradient_h(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color1: Color,
    color2: Color,
) {
    unsafe {
        raylib_ffi::DrawRectangleGradientH(pos_x, pos_y, width, height, color1, color2);
    }
}

/// Draw a gradient-filled rectangle with custom vertex colors
pub fn draw_rectangle_gradient_ex(
    rec: Rectangle,
    col1: Color,
    col2: Color,
    col3: Color,
    col4: Color,
) {
    unsafe {
        raylib_ffi::DrawRectangleGradientEx(rec, col1, col2, col3, col4);
    }
}

/// Draw rectangle outline
pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawRectangleLines(pos_x, pos_y, width, height, color);
    }
}

/// Draw rectangle outline with extended parameters
pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawRectangleLinesEx(rec, line_thick, color);
    }
}

/// Draw rectangle with rounded edges
pub fn draw_rectangle_rounded(rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawRectangleRounded(rec, roundness, segments, color);
    }
}

/// Draw rectangle with rounded edges outline
pub fn draw_rectangle_rounded_lines(
    rec: Rectangle,
    roundness: f32,
    segments: i32,
    line_thick: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawRectangleRoundedLines(rec, roundness, segments, line_thick, color);
    }
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe {
        raylib_ffi::DrawTriangle(v1, v2, v3, color);
    }
}

/// Draw triangle outline (vertex in counter-clockwise order!)
pub fn draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe {
        raylib_ffi::DrawTriangleLines(v1, v2, v3, color);
    }
}

/// Draw a triangle fan defined by points (first vertex is the center)
pub fn draw_triangle_fan(points: &[Vector2], point_count: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawTriangleFan(points.as_ptr() as *mut Vector2, point_count, color);
    }
}

/// Draw a triangle strip defined by points
pub fn draw_triangle_strip(points: &[Vector2], point_count: i32, color: Color) {
    unsafe {
        raylib_ffi::DrawTriangleStrip(points.as_ptr() as *mut Vector2, point_count, color);
    }
}

/// Draw a regular polygon (Vector version)
pub fn draw_poly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawPoly(center, sides, radius, rotation, color);
    }
}

/// Draw a polygon outline of n sides
pub fn draw_poly_lines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawPolyLines(center, sides, radius, rotation, color);
    }
}

/// Draw a polygon outline of n sides with extended parameters
pub fn draw_poly_lines_ex(
    center: Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    line_thick: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawPolyLinesEx(center, sides, radius, rotation, line_thick, color);
    }
}

/// Draw spline: Linear, minimum 2 points
pub fn draw_spline_linear(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawSplineLinear(
            points.as_ptr() as *mut Vector2,
            points.len() as i32,
            thick,
            color,
        );
    }
}

/// Draw spline: B-Spline, minimum 4 points
pub fn draw_spline_basis(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawSplineBasis(
            points.as_ptr() as *mut Vector2,
            points.len() as i32,
            thick,
            color,
        );
    }
}

/// Draw spline: Catmull-Rom, minimum 4 points
pub fn draw_spline_catmull_rom(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawSplineCatmullRom(
            points.as_ptr() as *mut Vector2,
            points.len() as i32,
            thick,
            color,
        );
    }
}

/// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
pub fn draw_spline_bezier_quadratic(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawSplineBezierQuadratic(
            points.as_ptr() as *mut Vector2,
            points.len() as i32,
            thick,
            color,
        );
    }
}

/// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
pub fn draw_spline_bezier_cubic(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawSplineBezierCubic(
            points.as_ptr() as *mut Vector2,
            points.len() as i32,
            thick,
            color,
        );
    }
}

/// Draw spline segment: Linear, 2 points
pub fn draw_spline_segment_linear(p1: Vector2, p2: Vector2, thick: f32, color: Color) {
    unsafe {
        raylib_ffi::DrawSplineSegmentLinear(p1, p2, thick, color);
    }
}

/// Draw spline segment: B-Spline, 4 points
pub fn draw_spline_segment_basis(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawSplineSegmentBasis(p1, p2, p3, p4, thick, color);
    }
}

/// Draw spline segment: Catmull-Rom, 4 points
pub fn draw_spline_segment_catmull_rom(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thick, color);
    }
}

/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
pub fn draw_spline_segment_bezier_quadratic(
    p1: Vector2,
    c2: Vector2,
    p3: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawSplineSegmentBezierQuadratic(p1, c2, p3, thick, color);
    }
}

/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
pub fn draw_spline_segment_bezier_cubic(
    p1: Vector2,
    c2: Vector2,
    c3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe {
        raylib_ffi::DrawSplineSegmentBezierCubic(p1, c2, c3, p4, thick, color);
    }
}

/// Get (evaluate) spline point: Linear
pub fn get_spline_point_linear(start_pos: Vector2, end_pos: Vector2, t: f32) -> Vector2 {
    unsafe { raylib_ffi::GetSplinePointLinear(start_pos, end_pos, t) }
}

/// Get (evaluate) spline point: B-Spline
pub fn get_spline_point_basis(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    unsafe { raylib_ffi::GetSplinePointBasis(p1, p2, p3, p4, t) }
}

/// Get (evaluate) spline point: Catmull-Rom
pub fn get_spline_point_catmull_rom(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    unsafe { raylib_ffi::GetSplinePointCatmullRom(p1, p2, p3, p4, t) }
}

/// Get (evaluate) spline point: Quadratic Bezier
pub fn get_spline_point_bezier_quad(p1: Vector2, c2: Vector2, p3: Vector2, t: f32) -> Vector2 {
    unsafe { raylib_ffi::GetSplinePointBezierQuad(p1, c2, p3, t) }
}

/// Get (evaluate) spline point: Cubic Bezier
pub fn get_spline_point_bezier_cubic(
    p1: Vector2,
    c2: Vector2,
    c3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    unsafe { raylib_ffi::GetSplinePointBezierCubic(p1, c2, c3, p4, t) }
}

/// Check collision between two rectangles
pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    unsafe { raylib_ffi::CheckCollisionRecs(rec1, rec2) }
}

/// Check collision between two circles
pub fn check_collision_circles(
    center1: Vector2,
    radius1: f32,
    center2: Vector2,
    radius2: f32,
) -> bool {
    unsafe { raylib_ffi::CheckCollisionCircles(center1, radius1, center2, radius2) }
}

/// Check collision between circle and rectangle
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe { raylib_ffi::CheckCollisionCircleRec(center, radius, rec) }
}

/// Check if point is inside rectangle
pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    unsafe { raylib_ffi::CheckCollisionPointRec(point, rec) }
}

/// Check if point is inside circle
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe { raylib_ffi::CheckCollisionPointCircle(point, center, radius) }
}

/// Check if point is inside a triangle
pub fn check_collision_point_triangle(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
) -> bool {
    unsafe { raylib_ffi::CheckCollisionPointTriangle(point, p1, p2, p3) }
}

/// Check if point is within a polygon described by array of vertices
pub fn check_collision_point_poly(point: Vector2, points: &[Vector2]) -> bool {
    unsafe {
        raylib_ffi::CheckCollisionPointPoly(
            point,
            points.as_ptr() as *mut Vector2,
            points.len() as i32,
        )
    }
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
pub fn check_collision_lines(
    start_pos1: Vector2,
    end_pos1: Vector2,
    start_pos2: Vector2,
    end_pos2: Vector2,
) -> Option<Vector2> {
    let mut collision_point = Vector2 { x: 0.0, y: 0.0 };
    let collision = unsafe {
        raylib_ffi::CheckCollisionLines(
            start_pos1,
            end_pos1,
            start_pos2,
            end_pos2,
            &mut collision_point,
        )
    };
    if collision {
        Some(collision_point)
    } else {
        None
    }
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
pub fn check_collision_point_line(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    threshold: i32,
) -> bool {
    unsafe { raylib_ffi::CheckCollisionPointLine(point, p1, p2, threshold) }
}

/// Get collision rectangle for two rectangles collision
pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    unsafe { raylib_ffi::GetCollisionRec(rec1, rec2) }
}
