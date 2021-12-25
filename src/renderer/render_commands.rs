use crate::{Colour, Point2, Vector2, TextureHandle, Section};

// The render command presented to the library user
pub enum RenderCommand<'s> {
    Clear(Colour),
    DrawLine(LineDescription),
    Draw2D(TwoDDescription),
    DrawText(Section<'s>)
}

// Description of the line to be drawn
pub struct LineDescription {
    pub start: Point2,
    pub end: Point2,
    pub width: f32,
    pub colour: Colour,
}

// 2D types
#[derive(Debug)]
pub enum TwoDTypes {
    Rectangle,
    Circle,
    Triangle,
    Hexagon,
}

impl TwoDTypes {
    pub fn to_int(&self) -> u32 {
        match self {
            TwoDTypes::Rectangle => { 1 },
            TwoDTypes::Circle =>    { 2 },
            TwoDTypes::Triangle =>  { 3 },
            TwoDTypes::Hexagon =>   { 4 },
        }
    }
}

// Drescription of the 2D thing to be drawn
#[derive(Debug)]
pub struct TwoDDescription {
    pub position: Point2,
    pub size: Vector2,
    pub colour: Colour,
    pub texture: Option<TextureHandle>,
    pub opacity: f32,
    pub line_width: f32,
    pub corner_radius: f32,
    pub rotation: f32,
    pub shape: TwoDTypes,
}

impl TwoDDescription {
    pub fn default() -> Self {
        Self {
            position: Point2::new(0.0,0.0),
            size: Vector2::new(0.0,0.0),
            colour: Colour::TRANSPARENT,
            texture: None,
            opacity: 1.0,
            line_width: 0.0,
            corner_radius: 0.0,
            rotation: 0.0,
            shape: TwoDTypes::Rectangle,
        }
    }
}