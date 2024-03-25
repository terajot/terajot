use super::DbReader;

pub struct NativeDbReader{

}

impl NativeDbReader {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DbReader for NativeDbReader {
    
}