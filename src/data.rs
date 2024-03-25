pub mod stack;

pub trait DbReader {
    fn new() -> Self;
  

    fn get_stacks(&self) -> Vec<Stack>;
  
    fn create_stack(&mut self, name: &str);
  
    fn delete_stack(&mut self, id: u32); 

    fn get_entries(&mut self, stack_id: u32) -> Vec<StackEntry>;

    fn add_entry(&mut self, stack_id: u32, entry: &str);

    fn edit_entry(&mut self, stack_id: u32, entry_id: u32, entry: &str);

    fn delete_entry(&mut self, stack_id: u32, entry_id: u32);

    fn get_entry(&mut self, stack_id: u32, entry_id: u32) -> Option<StackEntry>;

  }
  