use naia_shared::Protocolize;

mod auth;
mod entity_assignment;

pub use auth::Auth;
pub use entity_assignment::EntityAssignment;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    EntityAssignment(EntityAssignment),
}
