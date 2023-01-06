use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("test-sdl2", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

        let mut orth_proj: Vec<Vec<f32>> = Vec::new();

        let mut points: Vec<Vec<f32>> = Vec::new();
        
        points.push(vec![-0.5, -0.5, -0.5]);
        points.push(vec![0.5, -0.5, -0.5]);
        points.push(vec![0.5, -0.5, 0.5]);
        points.push(vec![-0.5, 0.5, 0.5]);
        points.push(vec![0.5, 0.5, -0.5]);
        points.push(vec![-0.5, 0.5, -0.5]);
        points.push(vec![-0.5, -0.5, 0.5]);
        points.push(vec![0.5, 0.5, 0.5]);

        let mut proj_points_2d: Vec<Vec<f32>> = Vec::new();
        for p in points{
            let x = p.get(0).unwrap().clone();
            let y = p.get(1).unwrap().clone();
            let z = p.get(2).unwrap().clone();
            proj_points_2d.push(vec![x, y, z]);
            //println!("x:{}, y:{}", x , y);
        }

        let mut rot_matr_z: Vec<Vec<f32>> = vec![vec![0f32; 3]; 3];
        let mut rot_matr_x: Vec<Vec<f32>> = vec![vec![0f32; 3]; 3];
        let mut rot_matr_y: Vec<Vec<f32>> = vec![vec![0f32; 3]; 3];
        let mut delta: f32 = 0.;
        


    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        
        delta += 0.01;
        //z-axis
        rot_matr_z[0][0] = f32::cos(delta);
        rot_matr_z[0][1] = -f32::sin(delta);
        rot_matr_z[0][2] = 0.;
        rot_matr_z[1][0] = f32::sin(delta);
        rot_matr_z[1][1] = f32::cos(delta);
        rot_matr_z[1][2] = 0.;
        rot_matr_z[2][0] = 0.;
        rot_matr_z[2][1] = 0.;
        rot_matr_z[2][2] = 1.;
        //x-axis
        rot_matr_x[0][0] = 1.;
        rot_matr_x[0][1] = 0.;
        rot_matr_x[0][2] = 0.;
        rot_matr_x[1][0] = 0.;
        rot_matr_x[1][1] = f32::cos(delta);
        rot_matr_x[1][2] = -f32::sin(delta);
        rot_matr_x[2][0] = 0.;
        rot_matr_x[2][1] = f32::sin(delta);
        rot_matr_x[2][2] = f32::cos(delta);
        //y-axis
        rot_matr_y[0][0] = f32::cos(delta);
        rot_matr_y[0][1] =0.;
        rot_matr_y[0][2] = f32::sin(delta);
        rot_matr_y[1][0] = 0.;
        rot_matr_y[1][1] = 1.;
        rot_matr_y[1][2] = 0.;
        rot_matr_y[2][0] = -f32::sin(delta);
        rot_matr_y[2][1] = 0.;
        rot_matr_y[2][2] = f32::cos(delta);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let mut i: usize = 0;
        let mut point_to_line: Vec<Point> = vec![Point::new(0, 0); 8];
        for pj in &proj_points_2d{

            let mut v_r = mat_vec_mul(&rot_matr_x, &vec![pj[0], pj[1], pj[2]]);
            v_r = mat_vec_mul(&rot_matr_y, &v_r);
            //v_r = mat_vec_mul(&rot_matr_z, &v_r);
            let d = 2.;
            let z = 1. / (d - v_r[2]); 
            orth_proj = vec![vec![z, 0., 0.], vec![0., z, 0.], vec![0., 0., 0.]];
            let proj_2d = mat_vec_mul(&orth_proj, &v_r);
            let proj_x: f32 = WIDTH/2. + (proj_2d[0]) * 200.;
            let proj_y: f32 = HEIGHT/2. + (proj_2d[1]) * 200.;
            println!("i:{} x:{} y:{} z:{} ", i%8, v_r[0], v_r[1], v_r[2]);
            
            point_to_line[i] = Point::new(proj_x as i32, proj_y as i32);
            canvas.draw_rect(Rect::from_center(point_to_line[i], 5, 5)).unwrap();
            i+= 1;
        }
        canvas.draw_line(point_to_line[0], point_to_line[1]).unwrap();
        canvas.draw_line(point_to_line[1], point_to_line[2]).unwrap();
        canvas.draw_line(point_to_line[2], point_to_line[6]).unwrap();
        canvas.draw_line(point_to_line[6], point_to_line[0]).unwrap();

        canvas.draw_line(point_to_line[0], point_to_line[5]).unwrap();
        canvas.draw_line(point_to_line[1], point_to_line[4]).unwrap();
        canvas.draw_line(point_to_line[2], point_to_line[7]).unwrap();
        canvas.draw_line(point_to_line[6], point_to_line[3]).unwrap();

        canvas.draw_line(point_to_line[5], point_to_line[4]).unwrap();
        canvas.draw_line(point_to_line[4], point_to_line[7]).unwrap();
        canvas.draw_line(point_to_line[7], point_to_line[3]).unwrap();
        canvas.draw_line(point_to_line[3], point_to_line[5]).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn mat_vec_mul(mat: &Vec<Vec<f32>>, vec: &Vec<f32>) -> Vec<f32>{
    let mut res: Vec<f32> = vec![0f32; 3];
    res[0] = mat[0][0] * vec[0] + mat[0][1] * vec[1] + mat[0][2] * vec[2];
    res[1] = mat[1][0] * vec[0] + mat[1][1] * vec[1] + mat[1][2] * vec[2];
    res[2] = mat[2][0] * vec[0] + mat[2][1] * vec[1] + mat[2][2] * vec[2];
    return res;
}
