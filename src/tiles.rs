use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};

pub trait ToF32<T> {
    fn to_f32(n: T) -> f32;
}

impl ToF32<i32> for i32 { fn to_f32(n: i32) -> f32 {n as f32}}
impl ToF32<i16> for i16 { fn to_f32(n: i16) -> f32 {n as f32}}
impl ToF32<f32> for f32 { fn to_f32(n: f32) -> f32 {n}}


pub fn from_tile(n: f32) -> f32  {
    return n * 32.0
}

pub fn rec_from_tiles<T, N>(ctx: &mut Context, x: N, y: N, w: N, h: N, dw: graphics::DrawMode, color: T) -> GameResult<graphics::Mesh>
    where T: Into<graphics::Color>,
          N: ToF32<N> 
{
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            dw,
            graphics::Rect::new(from_tile(N::to_f32(x)), from_tile(N::to_f32(y)), from_tile(N::to_f32(w)), from_tile(N::to_f32(h))),
            color.into()
        )?;
    Ok(mesh)
}

pub fn gen_platforms() -> Vec<(i16, i16, i16)> {
    let mut rng = thread_rng();
    let mut v = Vec::new();
    for _ in 0..10 {
        let x: i16 = rng.gen_range(0, 35) as i16;
        let y: i16 = rng.gen_range(1, 24) as i16;
        let w: i16 = rng.gen_range(3, 11) as i16;
        v.push((x,y,w))
    }
    v
}

