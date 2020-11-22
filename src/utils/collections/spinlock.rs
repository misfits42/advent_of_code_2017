use std::collections::VecDeque;

pub struct Spinlock {
    buffer: VecDeque<i64>,
    skip_size: usize,
    cursor: usize
}

impl Spinlock {
    /// Creates a new Spinlock with the buffer initialise with first value of 0.
    pub fn new(skip_size: usize) -> Self {
        Self {
            buffer: VecDeque::from(vec![0]),
            skip_size: skip_size,
            cursor: 0
        }
    }

    /// Moves the cursor forward by the skip size of the Spinlock.
    pub fn skip_forward(&mut self) {
        self.cursor = (self.cursor + self.skip_size) % self.buffer.len();
    }

    /// Inserts the given value at the index after the current cursor location.
    /// 
    /// Cursor location is updated to the location after the previous value.
    pub fn insert_after_cursor(&mut self, new_value: i64) {
        // Update cursor location to the index of the new value inserted
        self.cursor += 1;
        if self.cursor == self.buffer.len() { // Check if new value is being inserted at end
            self.buffer.push_back(new_value);
        } else { // Insert at the new cursor location
            self.buffer.insert(self.cursor, new_value);
        }
    }

    /// Returns the value in the location after the cursor.
    pub fn peek_after_cursor(&self) -> i64 {
        let peek_index = (self.cursor + 1) % self.buffer.len();
        return self.buffer[peek_index];
    }
}
