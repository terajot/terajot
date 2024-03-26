use color_eyre::eyre::Result;

/// Defines How to read a given model from the database.
pub trait DbReader<T>{

    /// Saves the model to the database.
    fn save(model:&T) -> Result<()>{
        Ok(())
    }
    
    /// Updates the model in the database.
    fn update(&mut self) -> Result<()>{
        Ok(())
    }

    /// Gets all entries from the database.
    fn get_all() -> Vec<T>;

    /// Gets a single entry from the database.
    fn get_by_id(id:u32)->Option<Result<T>>{
        None
    }

    /// Deletes a single entry from the database.
    fn delete(&self)->Result<()>{
        Ok(())
    }

}