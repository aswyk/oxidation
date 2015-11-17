use std::collections::HashMap;

use object::values::Value;

pub struct Table {
  array_part: Vec<Value>,
  hash_part:  HashMap<Value, Value>
}

