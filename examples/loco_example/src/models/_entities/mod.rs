pub mod prelude;

pub mod files;
pub mod notes;
pub mod users;

seaography::register_entity_modules!([files, notes, users,]);
