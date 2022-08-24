use gpu_attempt::Color3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let color: Color3 = Color3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_width - 1) as f64,
                0.25,
            );

            color.write(100);
        }
        println!("");
    }
}
