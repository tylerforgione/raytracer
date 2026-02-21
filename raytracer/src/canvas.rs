use crate::colour::Colour;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Colour>,
}

impl Canvas {
    // creates a new canvas object
    pub fn new(width: usize, height: usize) -> Canvas {
        let black: Colour = Colour::new(0.0, 0.0, 0.0);
        let pixels: Vec<Colour> = vec![black; (width * height) as usize];

        Canvas {
            width,
            height,
            pixels,
        }
    }

    // writes a pixel to the canvas at the given coordinates
    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        let index = (y * self.width) + x;
        self.pixels[index] = colour;
    }

    // returns the colour of the pixel at the given coordinates
    pub fn pixel_at(&self, x: usize, y: usize) -> Colour {
        let index = (y * self.width) + x;
        self.pixels[index]
    }

    // scales a colour value from 0.0-1.0 to 0-255 and clamps it to the range
    fn scale_colour(value: f64) -> u8 {
        let clamped = value.clamp(0.0, 1.0);
        (clamped * 255.0).round() as u8
    }

    // returns a string representation of the canvas in ppm format
    pub fn to_ppm(&self) -> String {
        // header
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            let mut line: String = String::new();

            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);
                let tokens = [
                    Self::scale_colour(pixel.r).to_string(),
                    Self::scale_colour(pixel.g).to_string(),
                    Self::scale_colour(pixel.b).to_string(),
                ];

                // if adding the next token would exceed 70 characters, write the current line to the ppm string and start a new line
                for t in tokens {
                    let sep = if line.is_empty() { 0 } else { 1 };

                    if line.len() + sep + t.len() > 70 {
                        ppm.push_str(&line);
                        ppm.push('\n');
                        line.clear();
                    }
                    if !line.is_empty() {
                        line.push(' ');
                    }
                    line.push_str(&t);
                }
            }

            // write any remaining tokens in the line to the ppm string
            ppm.push_str(&line);
            ppm.push('\n');
        }

        ppm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c: Canvas = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c: Canvas = Canvas::new(10, 20);
        let red: Colour = Colour::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c: Canvas = Canvas::new(5, 3);
        let ppm: String = c.to_ppm();
        let ppm_lines: Vec<&str> = ppm.lines().collect();

        assert_eq!(ppm_lines[0], "P3");
        assert_eq!(ppm_lines[1], "5 3");
        assert_eq!(ppm_lines[2], "255");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c: Canvas = Canvas::new(5, 3);
        let c1: Colour = Colour::new(1.5, 0.0, 0.0);
        let c2: Colour = Colour::new(0.0, 0.5, 0.0);
        let c3: Colour = Colour::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm: String = c.to_ppm();
        let ppm_lines: Vec<&str> = ppm.lines().collect();

        assert_eq!(ppm_lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(ppm_lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(ppm_lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let col = Colour::new(1.0, 0.8, 0.6);

        for y in 0..c.height {
            for x in 0..c.width {
                c.write_pixel(x, y, col);
            }
        }

        let ppm = c.to_ppm();
        let ppm_lines: Vec<&str> = ppm.lines().collect();

        // lines 4-7 in the book are 1-indexed; in Rust vec indexing they are [3..=6]
        assert_eq!(
            ppm_lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm_lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            ppm_lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm_lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert!(ppm.ends_with('\n'));
    }
}
