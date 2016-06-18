//
// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

extern crate open_aoe_drs as drs;
extern crate open_aoe_slp as slp;

extern crate minifb;

use std::io;
use std::path;

fn main() {
    let graphics_drs = match drs::DrsFile::read_from_file("data/graphics.drs") {
        Ok(drs) => drs,
        Err(err) => {
            println!("Failed to read graphics.drs: {}", err);
            return;
        }
    };

    // Just trying to get a graphic rendered on the screen for now
    // There are a lot of things wrong with this code
    let graphics_table = &graphics_drs.tables[0];
    let sample_slp_contents = &graphics_table.contents[15];
    let sample_slp = match slp::SlpFile::read_from(&mut io::Cursor::new(sample_slp_contents),
            path::PathBuf::from("test").as_path()) {
        Ok(slp) => slp,
        Err(err) => {
            println!("Failed to read SLP: {}", err);
            return;
        }
    };

    let sample_shape = &sample_slp.shapes[0];
    let width = sample_shape.header.width as usize;
    let height = sample_shape.header.height as usize;
    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let palette_index = sample_shape.pixels[index];
            // Don't have a palette parsed out yet, so just make up colors for now
            buffer[index] = (palette_index as u32) << 16;
        }
    }

    let mut window = match minifb::Window::new("OpenAOE", width, height,
            minifb::WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("Failed to create window: {}", err);
            return;
        }
    };

    while window.is_open() {
        window.update_with_buffer(&buffer);
    }
}