use bevy::prelude::*;

pub struct MenuAssets {
    pub button: ButtonAssets,
}

impl Default for MenuAssets {
    fn default() -> Self {
        MenuAssets {
            button: ButtonAssets::default(),
        }
    }
}

pub struct ButtonAssets {
    pub colors: ButtonColors,
}

impl Default for ButtonAssets {
    fn default() -> Self {
        ButtonAssets {
            colors: Default::default(),
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
