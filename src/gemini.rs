use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Pixel, Vec2D,
};


#[derive(Component)]
pub struct Blob {
    pub pixel: Pixel,
}

impl Blob {
    pub fn new(pos: Vec2D, color: ColChar) -> Self {
        Self {
            pixel: Pixel::new(pos, color),
        }
    }
}


#[derive(Resource)]
pub struct Gemini {
    pub view: View,
}

pub struct GeminiPlugin {
    pub canvas_w: usize,
    pub canvas_h: usize,
}

impl Plugin for GeminiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, clear_screen)
            .add_systems(Update, render_blobs.after(clear_screen).before(display_screen))
            .add_systems(Update, display_screen.after(clear_screen))
            .insert_resource(Gemini {
                view: View::new(self.canvas_w, self.canvas_h, ColChar::BACKGROUND),
            });
    }
}

fn clear_screen(
    mut gemini: ResMut<Gemini>,
) {
    gemini.view.clear();
}

fn render_blobs(
    mut gemini: ResMut<Gemini>,
    query: Query<&Blob>,
) {
    for blob in query.iter() {
        gemini.view.blit(&blob.pixel, Wrapping::Wrap);
    }
}

fn display_screen(
    gemini: Res<Gemini>,
) {
    gemini.view.display_render().unwrap();
}


