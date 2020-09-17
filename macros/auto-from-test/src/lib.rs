use auto_from::{def_it_works, AutoFrom};

def_it_works!();

#[derive(AutoFrom)]
pub struct ModelA<'a> {
    #[from(target = auto_from_test::ModelB, rename = "_id")]
    pub id: &'a i64,
    #[from(target = auto_from_test::ModelB)]
    pub text: &'a str,
}

pub struct ModelB {
    pub _id: i64,
    pub text: String,
}

impl<'a> From<&'a ModelB> for ModelA<'a> {
    fn from(b: &'a ModelB) -> Self {
        ModelA {
            id: &b._id,
            text: &b.text,
        }
    }
}
