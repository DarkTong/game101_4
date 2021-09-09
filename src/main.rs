extern crate nalgebra_glm as glm;
extern crate opencv;

use opencv::highgui;
use opencv::prelude::{Mat, MatTrait};
use opencv::core::{CV_8UC3, Vec3b, Size2i, Scalar, Scalar_, Point};
use opencv::imgproc::{circle, LINE_8};
use opencv::highgui::wait_key;

const WIDTH     :i32 = 700;
const HEIGHT    :i32 = 700;

fn get_pixel_point(x:f32, y:f32) -> Point{
    let u = (x * WIDTH as f32) as i32;
    let v = ((1.0 - y) * HEIGHT as f32) as i32;

    Point { x: u, y: v, }
}

fn set_pixel(canvas: &mut Mat, x: f32, y: f32, color: &glm::U8Vec3) {
    let p = get_pixel_point(x, y);
    let mut pixel = canvas.at_2d_mut::<Vec3b>(p.y, p.x).unwrap();
    pixel[0] = color[2];
    pixel[1] = color[1];
    pixel[2] = color[0];
}

// de Casteljau's algorithm
fn recursive_bezier(control_points: &Vec<glm::Vec2>, t: f32) -> glm::Vec2{
    let mut q = control_points.clone();
    let len = q.len();
    for i in 0..len - 1 {
        for j in 1..len - i {
            q[j - 1] = (1.0 - t) * q[j - 1] + t * q[j];
        }
    }
    return q[0];
}

fn native_bezier(points: &Vec<glm::Vec2>, canvas: &mut Mat) {
    let p_0 = &points[0];
    let p_1 = &points[1];
    let p_2 = &points[2];
    let p_3 = &points[3];
    let mut t = 0.0f32;
    while t < 1.0 {

        let p =
                1.0 * (1.-t).powf(3.0) * p_0 +
                3.0 * t * (1.-t).powf(2.0) * p_1 +
                3.0 * t.powf(2.0) * (1.-t) * p_2 +
                1.0 * t.powf(3.0) * p_3;
        t += 0.001;

        set_pixel(canvas, p.x, p.y, &glm::vec3(255u8, 0, 0));
    }
}

fn bezier(points: &Vec<glm::Vec2>, canvas: &mut Mat) {
    let mut t = 0.0f32;
    while t < 1.0 {
        let p = recursive_bezier(points, t);
        set_pixel(canvas, p.x, p.y, &glm::vec3(255u8, 0, 0));
        t += 0.001;
    }
}

const REAL_TIME : bool = true;

fn main() {
    let win_name = "Bezier Curve";
    highgui::named_window(win_name, highgui::WINDOW_NORMAL).unwrap();

    let mut control_points = Vec::new();
    control_points.push(glm::vec2(0.2, 0.4));
    control_points.push(glm::vec2(0.3, 0.7));
    control_points.push(glm::vec2(0.7, 0.8));
    control_points.push(glm::vec2(0.8, 0.5));

    let scalar = Scalar::new(0., 0., 0., 0.);
    let mut canvas = Mat::new_rows_cols_with_default(WIDTH, HEIGHT, CV_8UC3, scalar).unwrap();
    let mut key = 0i32;
    while key != 27 {

        for p in control_points.iter() {
            circle(&mut canvas, get_pixel_point(p.x, p.y),
                   3, Scalar::new(255.0, 255.0, 255.0,255.0),
                   3, LINE_8, 0);
        }

        if !REAL_TIME {
        }

        // native_bezier(&control_points, &mut canvas);
        bezier(&control_points, &mut canvas);
        highgui::imshow(win_name, &canvas).unwrap();

        key = highgui::wait_key(20).unwrap();
    }
}
