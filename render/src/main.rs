use std::{
    env,
    f32::consts::PI,
    fs::File,
    io::{Result, Write},
};

#[derive(Default, Debug)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Default, Debug)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec2 {
    fn yx(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    fn xyyx(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.x,
        }
    }

    fn mul(&self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }

    fn div(&self, s: f32) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }

    fn add(&self, s: f32) -> Self {
        Self {
            x: self.x + s,
            y: self.y + s,
        }
    }

    fn sub_vec(&self, b: &Self) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }

    fn add_vec(&self, b: &Self) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }

    fn mul_vec(&self, b: &Self) -> Self {
        Self {
            x: self.x * b.x,
            y: self.y * b.y,
        }
    }

    fn dot(&self, b: &Self) -> f32 {
        self.x * b.x + self.y * b.y
    }

    fn cos(&self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
        }
    }
}

impl Vec4 {
    fn sin(&self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
            w: self.w.sin(),
        }
    }

    fn exp(&self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
            w: self.w.exp(),
        }
    }

    fn tanh(&self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
            z: self.z.tanh(),
            w: self.w.tanh(),
        }
    }

    fn add(&self, s: f32) -> Self {
        Self {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
            w: self.w + s,
        }
    }

    fn mul(&self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }

    fn add_vec(&self, b: &Self) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w: self.w + b.w,
        }
    }

    fn div_vec(&self, b: &Self) -> Self {
        Self {
            x: self.x / b.x,
            y: self.y / b.y,
            z: self.z / b.z,
            w: self.w / b.w,
        }
    }

    fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

fn main() -> Result<()> {
    const W: usize = 16 * 60;
    const H: usize = 9 * 60;

    let args: Vec<String> = env::args().collect();

    if args.contains(&("plasma".to_string())) {
        for f in 0usize..240 {
            println!("Step {}", f);

            let mut file: File = File::create(format!("output-{:03}.ppm", f))?;
            let mut buf: [u8; 1555200] = [0u8; W * H * 3];

            file.write_all(format!("P6\n{} {}\n255\n", W, H).as_bytes())?;

            const R: Vec2 = Vec2 {
                x: W as f32,
                y: H as f32,
            };

            let t: f32 = ((f as f32) / 240f32) * 2f32 * PI;

            for y in 0..H {
                for x in 0..W {
                    let mut o: Vec4 = Vec4::default();
                    let fc: Vec2 = Vec2 {
                        x: x as f32,
                        y: y as f32,
                    };
                    //////////////////////////////
                    // https://x.com/XorDev/status/1894123951401378051
                    let p: Vec2 = (fc.mul(2f32).sub_vec(&R)).div(R.y);
                    let mut l: Vec2 = Vec2::default();
                    let mut i: Vec2 = Vec2::default();
                    l = l.add(4f32 - 4f32 * ((0.7f32 - p.dot(&p)).abs()));
                    let mut v: Vec2 = p.mul_vec(&l);
                    while i.y < 8f32 {
                        i = Vec2 {
                            x: i.x,
                            y: i.y + 1f32,
                        };
                        v = v
                            .add_vec(&(v.yx().mul(i.y).add_vec(&i).add(t)).cos().div(i.y).add(0.7));
                        o = o.add_vec(&v.xyyx().sin().add(1f32).mul((v.x - v.y).abs()));
                    }
                    o = Vec4 {
                        x: -1.0,
                        y: 1.0,
                        z: 2.0,
                        w: 0.0,
                    }
                    .mul(p.y)
                    .add(4f32)
                    .negate()
                    .add(l.x)
                    .exp()
                    .mul(5f32)
                    .div_vec(&o)
                    .tanh();
                    //////////////////////////////
                    let offset: usize = 3 * (y * W + x);
                    buf[offset + 0] = (o.x * 255f32) as u8;
                    buf[offset + 1] = (o.y * 255f32) as u8;
                    buf[offset + 2] = (o.z * 255f32) as u8;
                }
            }

            file.write_all(&buf)?;
        }
    } else {
        for f in 0usize..60 {
            let mut file: File = File::create(format!("output-{:02}.ppm", f))?;
            let mut buf: [u8; 1555200] = [0u8; W * H * 3];

            file.write_all(format!("P6\n{} {}\n255\n", W, H).as_bytes())?;

            for y in 0..H {
                for x in 0..W {
                    let offset: usize = 3 * (y * W + x);
                    if ((x + f) / 60 + (y + f) / 60) % 2 == 1 {
                        buf[offset + 0] = 0xff;
                        buf[offset + 1] = 0x00;
                        buf[offset + 2] = 0x00;
                    } else {
                        buf[offset + 0] = 0x00;
                        buf[offset + 1] = 0x00;
                        buf[offset + 2] = 0x00;
                    }
                }
            }

            file.write_all(&buf)?;
        }
    }

    Ok(())
}
