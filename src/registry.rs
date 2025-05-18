use mlua::Table;
use std::sync::{Arc, Mutex};

pub type RequestRegistry = Arc<Mutex<Vec<Table>>>;
