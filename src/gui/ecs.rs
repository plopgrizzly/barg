//! Extremely simple entity component system.

/// Storage for a component in an entity component system.
pub struct Storage<T: Clone, U: Into<u32>> {
    // Vec element for each entity.
    entities: Vec<T>,
    _phantom_data: std::marker::PhantomData<U>,
}

impl<T, U> Storage<T, U>
    where T: Clone, U: Into<u32>
{
    /// Create a new storage with default value `none`.  `none` represents an entity not having this
    /// component.
    pub fn new(none: T) -> Storage<T, U> {
        Storage {
            entities: vec![none],
            _phantom_data: std::marker::PhantomData,
        }
    }

    /// Get the value of the component for an entity.
    pub fn set(&mut self, entity: U, value: T) {
        // Get Id as index.
        let id = entity.into() as usize + 1;

        // Allocate space if not enough.
        if id >= self.entities.len() {
            self.entities.resize(id + 1, self.entities[0].clone() /* none */);
        }
        // Set entity's component.
        self.entities[id] = value;
    }

    /// Get the value of the component for an entity.
    pub fn get(&mut self, entity: U) -> T {
        // Get Id as index.
        let id = entity.into() as usize + 1;

        // Get exists.
        if let Some(rtn) = self.entities.get(id) {
            rtn.clone()
        } else {
            self.entities[0].clone() /* none */
        }
    }
}
