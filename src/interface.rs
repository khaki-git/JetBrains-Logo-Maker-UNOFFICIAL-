use ::image;
use fltk::{
    enums::{Align, Color, FrameType},
    prelude::*,
    *,
};
use std::rc::Rc;
use std::cell::RefCell;
use image::Rgba;
use rand::prelude::*;

fn rng_color() -> Color {
    let mut thread_rng = rand::thread_rng();
    let r: i32 = thread_rng.gen_range(0..255);
    let g: i32 = thread_rng.gen_range(0..255);
    let b: i32 = thread_rng.gen_range(0..255);

    Color::from_rgb(r as u8, g as u8, b as u8)
}

#[derive(Clone)]
struct ColorButton {
    reference_button: Rc<RefCell<button::Button>>,
}

#[derive(Clone)]
struct Icon {
    gradients: Vec<Color>,
    movement: f64,
    inner_text: String
}

#[derive(Clone, Copy)]
struct Vector2 {
    x: i32,
    y: i32,
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

impl Icon {
    fn interpolate_color(&self, c1: Color, c2: Color, t: f64) -> (f64, f64, f64) {
        let c1_dimensions = c1.to_rgb();
        let c2_dimensions = c2.to_rgb();

        let resulting_dimensions = (
            lerp(c1_dimensions.0 as f64, c2_dimensions.0 as f64, t),
            lerp(c1_dimensions.1 as f64, c2_dimensions.1 as  f64, t),
            lerp(c1_dimensions.2 as f64, c2_dimensions.2 as f64, t),
            );
        resulting_dimensions
    }

    fn new(gradients: Vec<Color>, movement: f64, inner_text: &str) -> Self {
        Icon {
            gradients,
            movement,
            inner_text: inner_text.to_string()
        }
    }

    fn get_goal(&self, scale: Vector2) -> Vector2 {
        let mut luck = thread_rng();

        loop {
            let x = Vector2 {
                x: luck.gen_range(scale.x / 4..3 * scale.x / 4),
                y: luck.gen_range(scale.y / 4..3 * scale.y / 4),
            };

            if x.x < (512/2)-(150/2) || x.x > (512/2)+(150/2) {
                return x;
            } else {
                if x.y < (512/2)-(150/2) || x.y > (512/2)+(150/2) {
                    return x;
                }
            }
        }
    }

    fn center_square(&self, pos: Vector2, scale: i32) -> Vector2 {
        let x = pos.x - (scale/2);
        let y = pos.y - (scale/2);

        Vector2 {
            x,
            y
        }
    }

    fn pick_pos_not_near(&self, dimensions: Vector2, not_near: Vec<Vector2>, max_range: u32) -> Vector2 {
        let mut rng = thread_rng();

        loop {
            let rng_pos = Vector2 {
                x: rng.gen_range(dimensions.x / 4..3 * dimensions.x / 4),
                y: rng.gen_range(dimensions.y / 4..3 * dimensions.y / 4),
            };
            
            if self.valid_coords(dimensions, rng_pos) {
                let mut is_far_enough = true;
                
                for entity in &not_near {
                    let distance = ((rng_pos.x - entity.x).pow(2) + (rng_pos.y - entity.y).pow(2)) as f64;
                    if distance.sqrt() <= (max_range * 2) as f64 {
                        is_far_enough = false;
                        break;
                    }
                }
                
                if is_far_enough {
                    return rng_pos;
                }
            }
        }
    }

    fn valid_coords(&self, dimensions: Vector2, position: Vector2) -> bool {
        position.x > 3 && position.x < dimensions.x - 3 && position.y > 3 && position.y < dimensions.y - 3
    }

    fn square(&self, pos: Vector2, mut img: image::RgbaImage, scale: f64, color: Color) -> image::RgbaImage {
        let dimensions_img = img.dimensions();
        let as_rgb = color.to_rgb();
        let color_rgba = Rgba([as_rgb.0, as_rgb.1, as_rgb.2, 255]);

        for x in 0..(scale.ceil() as i32 + 1) {
            for y in 0..(scale.ceil() as i32 + 1) {
                let pixel_x = pos.x + x;
                let pixel_y = pos.y + y;
                if pixel_x >= 0 && pixel_x < dimensions_img.0 as i32 && pixel_y >= 0 && pixel_y < dimensions_img.1 as i32 {
                    let dx = (x as f64 + 0.5).min(scale) - 0.5;
                    let dy = (y as f64 + 0.5).min(scale) - 0.5;
                    let alpha = ((1.0 - dx.max(0.0)) * (1.0 - dy.max(0.0))).min(1.0);

                    if alpha > 0.0 {
                        let current_color = img.get_pixel(pixel_x as u32, pixel_y as u32);
                        let new_color = image::Rgba([
                            ((1.0 - alpha) * current_color[0] as f64 + alpha * color_rgba[0] as f64) as u8,
                            ((1.0 - alpha) * current_color[1] as f64 + alpha * color_rgba[1] as f64) as u8,
                            ((1.0 - alpha) * current_color[2] as f64 + alpha * color_rgba[2] as f64) as u8,
                            255,
                        ]);
                        img.put_pixel(pixel_x as u32, pixel_y as u32, new_color);
                    }
                }
            }
        }
        img
    }

    fn gen(&self, export_location: &str) {
        let mut img = image::RgbaImage::new(512, 512);

        let dimensions = img.dimensions();
        let dim_v2 = Vector2 {
            x: dimensions.0 as i32,
            y: dimensions.1 as i32
        };

        let square_scale = thread_rng().gen_range(60..120) as f64;
        let pos = self.center_square(self.get_goal(dim_v2.clone()), square_scale as i32);
        let second = self.center_square(self.pick_pos_not_near(dim_v2.clone(), vec!(pos.clone()), (square_scale*((self.movement as f64) / 30.0)) as u32), square_scale as i32);
        let third = self.center_square(self.pick_pos_not_near(dim_v2.clone(), vec!(pos.clone(), second.clone()), (square_scale*((self.movement as f64) / 30.0)) as u32), square_scale as i32);
        let final_pos = self.center_square(self.pick_pos_not_near(dim_v2.clone(), vec!(pos.clone(), second.clone(), third.clone()), (square_scale*((self.movement as f64) / 30.0)) as u32), square_scale as i32);

        img = self.square(pos.clone(), img, square_scale, self.gradients[0]);
        img = self.square(second.clone(), img, square_scale * 1.25, self.gradients[1]);
        img = self.square(third.clone(), img, square_scale * 1.5, self.gradients[2]);
        img = self.square(final_pos.clone(), img, square_scale / 1.5, self.gradients[3]);

        // Interpolate the four positions

        for i in 1..100 {
            let t = (i as f64) / 100.0;
            let i_x = lerp(pos.x as f64, second.x as f64, t);
            let i_y = lerp(pos.y as f64, second.y as f64, t);

            let i_col = self.interpolate_color(self.gradients[0], self.gradients[1], t);
            let position = Vector2 {
                x: i_x as i32,
                y: i_y as i32
            };

            let l_square_scale = lerp(square_scale, square_scale * 1.25, t);

            img = self.square(position.clone(), img, l_square_scale,
            Color::from_rgb(
                i_col.0 as u8,
                i_col.1 as u8,
                i_col.2 as u8
            ));
        }

        for i in 1..100 {
            let t = (i as f64) / 100.0;
            let i_x = lerp(second.x as f64, third.x as f64, t);
            let i_y = lerp(second.y as f64, third.y as f64, t);

            let i_col = self.interpolate_color(self.gradients[1], self.gradients[2], t);
            let position = Vector2 {
                x: i_x as i32,
                y: i_y as i32
            };

            let l_square_scale = lerp(square_scale * 1.25, square_scale * 1.5, t);

            img = self.square(position.clone(), img, l_square_scale,
                              Color::from_rgb(
                                  i_col.0 as u8,
                                  i_col.1 as u8,
                                  i_col.2 as u8
                              ));
        }

        for i in 1..100 {
            let t = (i as f64) / 100.0;
            let i_x = lerp(third.x as f64, final_pos.x as f64, t);
            let i_y = lerp(third.y as f64, final_pos.y as f64, t);

            let i_col = self.interpolate_color(self.gradients[2], self.gradients[3], t);
            let position = Vector2 {
                x: i_x as i32,
                y: i_y as i32
            };

            let l_square_scale = lerp(square_scale * 1.5, square_scale / 1.5, t);

            img = self.square(position.clone(), img, l_square_scale,
                              Color::from_rgb(
                                  i_col.0 as u8,
                                  i_col.1 as u8,
                                  i_col.2 as u8
                              ));
        }

        img = self.square(self.center_square(Vector2 {
            x: 512 / 2,
            y: 512 / 2
        }, 200),
        img,
        200.0,
        Color::Black
        );

        img.save(export_location).unwrap();
    }
}

const WIDTH: u16 = 800;
const HEIGHT: u16 = 500;

impl ColorButton {
    fn new(pos_x_offset: i32, pos_y_offset: i32, id: &str, color_reference: Rc<RefCell<Color>>) -> ColorButton {
        let color_button = Rc::new(RefCell::new(
            button::Button::default().with_size(250, 50).with_pos(
                ((WIDTH/2) as i32) + pos_x_offset,
                ((HEIGHT/2) as i32) + pos_y_offset,
            ).with_label(format!("Select Gradient Color #{}", id).as_str())
        ));
        let clr = rng_color();
        color_button.borrow_mut().set_color(clr);
        color_button.borrow_mut().set_frame(FrameType::FlatBox);

        color_reference.swap(&RefCell::new(clr));

        color_button.borrow_mut().set_callback(move |_self| {
            let dialog_result = dialog::color_chooser("Gradient Picker", dialog::ColorMode::Rgb);
            match dialog_result {
                Some(t) => {
                    let clr_rgb = Color::from_rgb(
                        t.0,
                        t.1,
                        t.2
                    );
                    _self.set_color(clr_rgb);
                    color_reference.swap(&RefCell::new(clr_rgb));
                },
                None => {_self.set_color(rng_color());}
            }
        });

        ColorButton {
            reference_button: color_button
        }
    }
}

pub fn home() {

    let app = app::App::default();
    app::background(255,255,255);
    let mut wind = window::Window::new(100, 100, WIDTH as i32, HEIGHT as i32, "JetBrains Inspired Product Logo Generator");

    let mut bar =
        frame::Frame::new(0, 0, WIDTH as i32, 60, "     JetBrains Icon Generator (unofficial)").with_align(Align::Left | Align::Inside);

    let movement_slider = Rc::new(RefCell::new(
        valuator::HorNiceSlider::default().with_size(500, 30).with_pos(((WIDTH/2)-500/2) as i32, (HEIGHT/2) as i32)
    ));

    let slider_preview = Rc::new(RefCell::new(
        frame::Frame::default().with_size(500, 100)
            .with_pos(((WIDTH/2)-500/2) as i32, ((HEIGHT/2)-100) as i32)
    ));

    let c1 = Rc::new(RefCell::new(Color::from_rgb(0,0,0)));
    let c2 = Rc::new(RefCell::new(Color::from_rgb(0,0,0)));
    let c3 = Rc::new(RefCell::new(Color::from_rgb(0,0,0)));
    let c4 = Rc::new(RefCell::new(Color::from_rgb(0,0,0)));

    let c5 = c1.clone();
    let c6 = c2.clone();
    let c7 = c3.clone();
    let c8 = c4.clone();

    let col_clone1 = c5.clone();
    let col_clone2 = c6.clone();
    let col_clone3 = c7.clone();
    let col_clone4 = c8.clone();

    let col_1 = ColorButton::new(-375, 100, "1", c1);
    let col_2 = ColorButton::new(-125, 100, "2", c2);
    let col_3 = ColorButton::new(125, 100, "3", c3);
    let col_4 = ColorButton::new(-125, 150, "4", c4);

    let shuffle = Rc::new(RefCell::new(
        button::Button::default().with_size(150, 50)
            .with_pos(((WIDTH/2)-150/2) as i32, ((HEIGHT/2)+50) as i32).with_label("Shuffle")
    ));

    let generate_button = Rc::new(RefCell::new(
        button::Button::default().with_size(150, 50)
            .with_pos(((WIDTH/2)-150/2) as i32, ((HEIGHT/2)-150) as i32).with_label("Generate")
    ));

    generate_button.borrow_mut().set_callback(move |_self| {
        let mut export = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseSaveFile);
        export.set_title("Save Location");
        export.show();
        let icon = Icon::new(vec![
            c5.borrow().clone(),
            c6.borrow().clone(),
            c7.borrow().clone(),
            c8.borrow().clone(),
        ], 25.0, "LL");
        icon.gen(export.filename().to_str().unwrap())
    });

    generate_button.borrow_mut().set_frame(FrameType::FlatBox);
    generate_button.borrow_mut().set_color(Color::from_rgb(150, 150, 150));

    shuffle.borrow_mut().set_label_size(20);
    shuffle.borrow_mut().set_frame(FrameType::FlatBox);
    shuffle.borrow_mut().set_color(Color::from_rgb(200, 200, 200));

    let c1_clone = col_1.reference_button.clone();
    let c2_clone = col_2.reference_button.clone();
    let c3_clone = col_3.reference_button.clone();
    let c4_clone = col_4.reference_button.clone();
    let movement_slider_clone = movement_slider.clone();
    let slider_preview_clone = slider_preview.clone();

    shuffle.borrow_mut().set_callback(move |_| {
        c1_clone.borrow_mut().set_label("Select Gradient Color #1");
        c2_clone.borrow_mut().set_label("Select Gradient Color #2");
        c3_clone.borrow_mut().set_label("Select Gradient Color #3");
        c4_clone.borrow_mut().set_label("Select Gradient Color #4");

        let c1_col_new = rng_color();
        let c2_col_new = rng_color();
        let c3_col_new = rng_color();
        let c4_col_new = rng_color();

        c1_clone.borrow_mut().set_color(c1_col_new);
        c2_clone.borrow_mut().set_color(c2_col_new);
        c3_clone.borrow_mut().set_color(c3_col_new);
        c4_clone.borrow_mut().set_color(c4_col_new);

        col_clone1.swap(&RefCell::new(c1_col_new));
        col_clone2.swap(&RefCell::new(c2_col_new));
        col_clone3.swap(&RefCell::new(c3_col_new));
        col_clone4.swap(&RefCell::new(c4_col_new));

        let rng_movement = rand::thread_rng().gen_range(25..50) as f64;

        movement_slider_clone.borrow_mut().set_value(rng_movement);
        slider_preview_clone.borrow_mut().set_label(format!("Movement: {}%", rng_movement).as_str());
    });

    slider_preview.borrow_mut().set_label("Movement: 25%");
    slider_preview.borrow_mut().set_frame(FrameType::FlatBox);
    slider_preview.borrow_mut().set_label_size(18);

    movement_slider.borrow_mut().set_frame(FrameType::FlatBox);
    movement_slider.borrow_mut().set_label_size(16);
    movement_slider.borrow_mut().set_minimum(25.);
    movement_slider.borrow_mut().set_maximum(50.);

    bar.set_frame(FrameType::FlatBox);
    bar.set_label_size(22);
    bar.set_label_color(Color::White);
    bar.set_color(Color::from_rgb(196, 52, 21));

    let movement_slider_clone = movement_slider.clone();
    let slider_preview_clone = slider_preview.clone();

    movement_slider.borrow_mut().set_callback(move |_| {
        let value = movement_slider_clone.borrow().value();
        slider_preview_clone.borrow_mut().set_label(&format!("Movement: {}%", value.round()));
    });

    wind.end();
    wind.show();

    app.run().unwrap();
}