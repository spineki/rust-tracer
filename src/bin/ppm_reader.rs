use core::panic;
use std::{env, fs};

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    magic_number: String,
    width: u32,
    height: u32,
    max_color: u32,
    values: Vec<(f32, f32, f32)>,
}

fn model(_app: &App) -> Model {
    // loading the inital model
    let args: Vec<String> = env::args().collect();

    // the name of the file shoul de the second env parameter
    if args.len() != 2 {
        panic!("Only the name of the file is expected");
    }

    // extracting the lines from the file
    let raw_lines = fs::read_to_string(args[1].to_string()).expect("Unable to open the file");
    let lines: Vec<&str> = raw_lines
        .split('\n')
        // and omiting lines starting with #
        .filter(|line| !line.starts_with('#'))
        // removing extra spaces
        .map(|line| line.trim())
        // removing empty lines
        .filter(|line| !line.is_empty())
        .collect();

    let magic_number = lines[0].to_string();
    if magic_number != "P3" {
        panic!("only P3 supported");
    }

    let mut geometry_line = lines[1].split_ascii_whitespace();
    let (width, height) = (
        geometry_line
            .next()
            .expect("no width found")
            .parse::<u32>()
            .expect("width not a number"),
        geometry_line
            .next()
            .expect("no height found")
            .parse::<u32>()
            .expect("height not a number"),
    );

    let max_color = lines[2].parse::<u32>().expect("max color not a number");

    let values = lines[3..]
        .iter()
        .map(|line| {
            let rgbs = line
                .split_ascii_whitespace()
                .map(|value| value.parse::<f32>().expect("cannot convert one value"))
                .collect::<Vec<_>>();
            (
                rgbs[0] / max_color as f32,
                rgbs[1] / max_color as f32,
                rgbs[2] / max_color as f32,
            )
        })
        .collect::<Vec<_>>();

    Model {
        magic_number,
        width,
        height,
        max_color,
        values,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();

    let nb_rows = model.height;
    let nb_colums = model.width;

    let window = frame.rect();
    let tile_width = window.w() / nb_colums as f32;
    let tile_height = window.h() / nb_rows as f32;

    dbg!(model.values.len());

    for i in 0..nb_rows {
        for j in 0..nb_colums {
            let rbgs = model.values[(i * nb_colums + j) as usize];
            let x = j as f32 * tile_width - window.w() / 2.0 + tile_width / 2.0;
            let y = (nb_rows - i - 1) as f32 * tile_height - window.h() / 2.0 + tile_height / 2.0;

            draw.rect()
                .color(Rgb::from_components(rbgs))
                .w_h(tile_width, tile_height)
                .x_y(x, y);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
