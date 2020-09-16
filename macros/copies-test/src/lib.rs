use copies::{def_it_works, AutoFrom};

def_it_works!();

#[derive(AutoFrom)]
pub struct ModelA {
    #[from(target = copies_test::ModelB)]
    #[from(target = copies_test::ModelC, rename = "_id")]
    pub id: i64,
}

pub struct ModelB {
    pub id: i64,
}

pub struct ModelC {
    pub _id: i64,
}
