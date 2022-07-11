use bevy::prelude::*;

use super::FontAssets;

pub struct MenuAssets {
    pub button: TextAssets,
    pub text: TextAssets,
    pub textbox: TextAssets,
}

impl MenuAssets {
    pub fn new(font_assets: &FontAssets) -> Self {
        MenuAssets {
            button: TextAssets::button(font_assets),
            text: TextAssets::text(font_assets),
            textbox: TextAssets::textbox(font_assets),
        }
    }
}

pub struct TextAssets {
    pub colors: Colors,
    text_style: TextStyle,
    style: Style,
}

impl TextAssets {
    fn button(font_assets: &FontAssets) -> Self {
        let colors = Colors::button();
        Self {
            colors: colors.clone(),
            text_style: TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 40.0,
                color: colors.text.normal.0,
            },
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }

    fn textbox(font_assets: &FontAssets) -> Self {
        let colors = Colors::textbox();
        Self {
            colors: colors.clone(),
            text_style: TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 40.0,
                color: colors.text.normal.0,
            },
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }

    fn text(font_assets: &FontAssets) -> Self {
        let colors = Colors::text();
        Self {
            colors: colors.clone(),
            text_style: TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 40.0,
                color: colors.text.normal.0,
            },
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }

    pub fn create_button(&self, width: f32, height: f32) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(height)),
                ..self.style
            },
            color: self.colors.background.as_ref().unwrap().normal,
            ..default()
        }
    }

    pub fn create_textbox(&self, width: f32, height: f32) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Px(height)),
                ..self.style
            },
            color: self.colors.background.as_ref().unwrap().normal,
            ..default()
        }
    }

    pub fn create_text(&self, text: String) -> TextBundle {
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: self.text_style.clone(),
                }],
                alignment: default(),
            },
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    }
    pub fn create_subtext(&self, text: String) -> TextBundle {
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: self.text_style.clone(),
                }],
                alignment: default(),
            },

            ..default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub text: ColorSet,
    pub background: Option<ColorSet>,
}

#[derive(Debug, Clone)]
pub struct ColorSet {
    pub normal: UiColor,
    pub hovered: UiColor,
}

impl Colors {
    fn button() -> Self {
        Self {
            text: ColorSet {
                normal: Color::rgb(0.9, 0.9, 0.9).into(),
                hovered: Color::rgb(0.9, 0.9, 0.9).into(),
            },
            background: Some(ColorSet {
                normal: Color::rgb(0.15, 0.15, 0.15).into(),
                hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            }),
        }
    }

    fn textbox() -> Self {
        Self {
            text: ColorSet {
                normal: Color::rgb(0.1, 0.1, 0.1).into(),
                hovered: Color::rgb(0.2, 0.2, 0.2).into(),
            },
            background: Some(ColorSet {
                normal: Color::rgb(0.9, 0.9, 0.9).into(),
                hovered: Color::rgb(0.9, 0.9, 0.9).into(),
            }),
        }
    }

    fn text() -> Self {
        Self {
            text: ColorSet {
                normal: Color::rgb(0.1, 0.1, 0.1).into(),
                hovered: Color::rgb(0.2, 0.2, 0.2).into(),
            },
            background: None,
        }
    }
}
