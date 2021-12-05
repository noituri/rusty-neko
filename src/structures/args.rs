use crate::enums::arg_types::arg_types;

pub struct Args {
    pub list: Vec<arg_types>
}

pub trait ArgsTrait {
    fn get_unsafe(&self, index: usize) -> &arg_types;
    fn get_number(&self, index: usize) -> Result<&i64, ()>;
    fn get_string(&self, index: usize) -> Result<&str, ()>;
}

impl ArgsTrait for Args {
    fn get_unsafe(&self, index: usize) -> &arg_types {
        self.list.get(index).unwrap()
    }

    fn get_string(&self, index: usize) -> Result<&str, ()> {
        let item = self.get_unsafe(index);

        match item {
            arg_types::String(item) => Ok(item),
            _ => Err(())
        }
    }

    fn get_number(&self, index: usize) -> Result<&i64, ()> {
        let item = self.get_unsafe(index);

        match item {
            arg_types::Int(item) => Ok(item),
            _ => Err(())
        }
    }
}