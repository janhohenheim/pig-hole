use crate::menu::create_lobby::CreateLobbySubMenu;

#[derive(Debug, Clone, PartialEq)]
pub enum SubMenu {
    None,
    CreateLobby(Option<CreateLobbySubMenu>),
}

impl Default for SubMenu {
    fn default() -> Self {
        SubMenu::None
    }
}
