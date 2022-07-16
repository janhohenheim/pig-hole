use crate::menu::browse_lobbies::BrowseLobbiesSubMenu;
use crate::menu::create_lobby::CreateLobbySubMenu;

pub enum SubMenu {
    Main,
    CreateLobby(CreateLobbySubMenu),
    BrowseLobbies(BrowseLobbiesSubMenu),
}

impl Default for SubMenu {
    fn default() -> Self {
        SubMenu::Main
    }
}
