use mlua::Table;
use std::{cell::RefCell, rc::Rc};

pub type RequestRegistry = Rc<RefCell<Vec<Table>>>;
