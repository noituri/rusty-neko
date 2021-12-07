use crate::enums::arg_types::ArgTypes;

pub struct Args {
    pub list: Vec<ArgTypes>,
}

pub trait ArgsTrait {
    fn get_unsafe(&self, index: usize) -> &ArgTypes;
    fn get_number(&self, index: usize) -> Result<&i64, ()>;
    fn get_string(&self, index: usize) -> Result<&str, ()>;
    fn get_bool(&self, index: usize) -> Result<&bool, ()>;
    fn size(&self) -> usize;
}

impl ArgsTrait for Args {
    fn get_unsafe(&self, index: usize) -> &ArgTypes {
        self.list.get(index).unwrap()
    }

    fn get_bool(&self, index: usize) -> Result<&bool, ()> {
        let item = self.get_unsafe(index);

        match item {
            ArgTypes::Bool(item) => Ok(item),
            _ => Err(()),
        }
    }

    fn size(&self) -> usize {
        self.list.len()
    }

    fn get_string(&self, index: usize) -> Result<&str, ()> {
        let item = self.get_unsafe(index);

        match item {
            ArgTypes::String(item) => Ok(item),
            _ => Err(()),
        }
    }

    fn get_number(&self, index: usize) -> Result<&i64, ()> {
        let item = self.get_unsafe(index);

        match item {
            ArgTypes::Int(item) => Ok(item),
            _ => Err(()),
        }
    }
}
