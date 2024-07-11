mod Tri;
mod Vector;

use std::fs::File;
use std::io::BufReader;

use stl_io::{create_stl_reader, TriangleIterator};

use image::{ImageBuffer, Rgb};

use crate::Tri::Triangle;
use crate::Vector::Vector3;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the STL file

    let file = File::open("monke.stl")?;
    let mut buf_reader = BufReader::new(file);

    let origin = Vector3 { x: 0.0, y: -5.0, z: 0.0 };
    let mut direction = Vector3 { x: 0.0, y: 1.0, z: 0.0 };

    let k_amb = 0.25;
    let k_diff = 0.7;
    let k_spec = 0.05;

    let light = Vector3 { x: 5.0, y: -5.0, z: 5.0 };


    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(200, 200);
    for X in 0..=199 {
        for Z in 0..=199 {
            let mut dist = 999999999999999.999999999999999;

            direction = Vector3 { x: (X as f64 - 100.0) / 50.0, y: 0.0, z: (Z as f64 - 100.0) / 50.0  } - origin;
            let mut r = 0 as u8;
            let mut g = 0 as u8;
            let mut b = 0 as u8;

            let mut pointi = Vector3 { x: 9.699, y: 9.699, z: 9.699 as f64 };

            // Create an STL reader
            let  stl_reader = create_stl_reader(&mut buf_reader)?;

            // Iterate through the triangles
            for (index, triangle_result) in stl_reader.enumerate() {
                match triangle_result {
                    Ok(triangle) => {
                        let vertices: Vec<_> = triangle.vertices.iter().enumerate().take(3).collect();
                        let triangle = Triangle{
                            a: Vector3 { x: vertices[0].1[0] as f64, y: vertices[0].1[1] as f64, z: vertices[0].1[2] as f64 },
                            b: Vector3 { x: vertices[1].1[0] as f64, y: vertices[1].1[1] as f64, z: vertices[1].1[2] as f64 },
                            c: Vector3 { x: vertices[2].1[0] as f64, y: vertices[2].1[1] as f64, z: vertices[2].1[2] as f64 }
                        };

                        match triangle.moller_trumbore_intersection(origin, direction) {
                            Some(point) => {
                                let newd = (point - origin).mod1();
                                if newd < dist {

                                    let light_dir = (light - point).unitise();
                                    let norm = triangle.unit_normal();

                                    let diff = light_dir.dot(&norm);


                                    let refl = direction.reflect(&norm).unitise();
                                    let view_dir = (origin - point).unitise();

                                    let spec = refl.dot(&view_dir).powf(5.0);


                                    r = (((k_amb + k_diff * diff) * 19.0) + (k_spec * spec * 255.0)) as u8;
                                    g = (((k_amb + k_diff * diff) * 243.0) + (k_spec * spec * 255.0)) as u8;
                                    b = (((k_amb + k_diff * diff) * 200.0) + (k_spec * spec * 255.0)) as u8;
                                    pointi = point;

                                    dist = newd;
                                }
                            },
                            None => ()
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading triangle: {}", e);
                    }
                }
            }

            let pix = [r, g, b];

            image.put_pixel(X, Z, Rgb(pix));
        }
    }

    image.save("output1.png");


    Ok(())
}
