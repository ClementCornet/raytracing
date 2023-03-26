
fn main() {
    // Image

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255",IMAGE_HEIGHT,IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
        for i in 0..IMAGE_WIDTH{
            let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let ir = (255.999*r) as u32;
            let ig = (255.999*g) as u32;
            let ib = (255.999*b) as u32;

            println!("{} {} {}",ir,ig,ib);
        }
    }
    eprintln!("Done");

}