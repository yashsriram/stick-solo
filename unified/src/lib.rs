use ggez::{Context, GameResult};

pub mod act;
pub mod plan;
pub mod sense;

pub trait Draw {
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
}
