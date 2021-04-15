use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicUsize, Ordering},
};

lazy_static! {
    static ref OBJECT_ID: AtomicUsize = AtomicUsize::new(0);
}

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID {
    id: usize,
}

impl InputCellID {
    fn new() -> Self {
        Self {
            id: OBJECT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }
}

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID {
    id: usize,
}

impl ComputeCellID {
    fn new() -> Self {
        Self {
            id: OBJECT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID {
    id: usize,
}

impl CallbackID {
    fn new() -> Self {
        Self {
            id: OBJECT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    graph: HashMap<CellID, Vec<CellID>>,
    input_values: HashMap<InputCellID, T>,
    compute_cell_funcs: HashMap<ComputeCellID, Box<dyn Fn(&[T]) -> T>>,
    callbacks: HashMap<ComputeCellID, HashMap<CallbackID, Box<dyn FnMut(T)>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq + std::fmt::Debug> Reactor<T> {
    pub fn new() -> Self {
        Self {
            graph: Default::default(),
            input_values: Default::default(),
            compute_cell_funcs: Default::default(),
            callbacks: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let input_cell_id = InputCellID::new();
        self.graph.entry(CellID::Input(input_cell_id)).or_default();
        self.input_values.insert(input_cell_id, initial);
        input_cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID>
    where
        F: Fn(&[T]) -> T + 'static,
    {
        for &dep in dependencies.iter() {
            if !self.graph.contains_key(&dep) {
                return Err(dep);
            }
        }

        let compute_cell_id = ComputeCellID::new();
        self.compute_cell_funcs
            .insert(compute_cell_id, Box::new(compute_func));
        self.graph.insert(
            CellID::Compute(compute_cell_id),
            dependencies.iter().copied().collect(),
        );
        Ok(compute_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(input_cell_id) => self.input_values.get(&input_cell_id).map(|&id| id),
            CellID::Compute(compute_cell_id) => self
                .compute_cell_funcs
                .get(&compute_cell_id)
                .and_then(|func| {
                    let mut evaluated_deps = vec![];
                    for &dep in self.graph[&id].iter() {
                        if let Some(dep_value) = self.value(dep) {
                            evaluated_deps.push(dep_value);
                        } else {
                            return None;
                        }
                    }

                    Some(func(&evaluated_deps))
                }),
        }
    }

    fn depends_on(&self, a: CellID, b: CellID) -> bool {
        let mut stack = vec![a];
        let mut seen = HashSet::new();

        while let Some(node) = stack.pop() {
            if node == b {
                return true;
            }

            if seen.insert(node) {
                if let Some(deps) = self.graph.get(&node) {
                    stack.extend(deps);
                }
            }
        }
        false
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let input_cell = CellID::Input(id);
        if self.input_values.contains_key(&id) {
            let mut current_values = vec![];
            let mut cells_to_compute = vec![];
            for &compute_cell_id in self.compute_cell_funcs.keys() {
                if self.depends_on(CellID::Compute(compute_cell_id), input_cell) {
                    cells_to_compute.push(compute_cell_id);
                }
            }

            for &cell in cells_to_compute.iter() {
                current_values.push((cell, self.value(CellID::Compute(cell))));
            }

            self.input_values.insert(id, new_value);

            let mut cells_to_callback = vec![];
            for (cell, current_value) in current_values.into_iter() {
                let new_value = self.value(CellID::Compute(cell));
                if new_value != current_value {
                    if let Some(new_value) = new_value {
                        cells_to_callback.push((cell, new_value));
                    }
                }
            }

            for (cell_to_callback, new_value) in cells_to_callback.into_iter() {
                if let Some(callbacks) = self.callbacks.get_mut(&cell_to_callback) {
                    for callback in callbacks.values_mut() {
                        callback(new_value);
                    }
                }
            }
            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F>(&mut self, id: ComputeCellID, callback: F) -> Option<CallbackID>
    where
        F: FnMut(T) + 'static,
    {
        if self.compute_cell_funcs.contains_key(&id) {
            let callback_id = CallbackID::new();
            self.callbacks
                .entry(id)
                .or_default()
                .insert(callback_id, Box::new(callback));
            Some(callback_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if self
            .callbacks
            .get_mut(&cell)
            .ok_or(RemoveCallbackError::NonexistentCell)?
            .remove(&callback)
            .is_none()
        {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        Ok(())
    }
}
