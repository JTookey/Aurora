use crate::{Colour, Point2};

pub enum RenderCommand {
    Clear(Colour),
    DrawLine(LineDescription),
}

pub struct LineDescription {
    pub start: Point2,
    pub end: Point2,
    pub width: f32,
    pub colour: Colour,
}