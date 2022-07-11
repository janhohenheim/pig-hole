use bevy::prelude::*;

use super::FontAssets;

pub struct MenuAssets {
    pub button: ButtonAssets,
}

impl MenuAssets {
    pub fn new(font_assets: &FontAssets) -> Self {
        MenuAssets {
            button: ButtonAssets::new(font_assets),
        }
    }
}

pub struct ButtonAssets {
    pub colors: ButtonColors,
    pub text_style: TextStyle,
    pub style: Style,
}

impl ButtonAssets {
    fn new(font_assets: &FontAssets) -> Self {
        ButtonAssets {
            colors: Default::default(),
            text_style: TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        }
    }

    pub fn create_button(&self, width: f32, height: f32) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(height)),
                ..self.style
            },
            color: self.colors.normal,
            ..Default::default()
        }
    }

    pub fn create_text(&self, text: String) -> TextBundle {
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: self.text_style.clone(),
                }],
                alignment: Default::default(),
            },
            ..Default::default()
        }
    }
}

pub struct ButtonColors {
    pub normal: UiColor,
    pub hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}
