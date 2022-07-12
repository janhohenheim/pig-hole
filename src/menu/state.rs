use crate::menu::create_lobby::CreateLobbySubMenu;

#[derive(Clone, PartialEq)]
pub enum SubMenu {
    Main,
    CreateLobby(CreateLobbySubMenu),
}

impl Default for SubMenu {
    fn default() -> Self {
        SubMenu::Main
    }
}
