
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

use state::LuaState;
use object::values::{Value, GCVariant};

const INITIAL_GC_THRESHOLD: usize = 100;

pub type GCInnards = (Cell<GCHeader>, RefCell<GCVariant>);

#[derive(Clone, Debug)]
pub struct GCObject {
  pub ptr: Rc<GCInnards>
}

pub type Root = Rc<(Cell<usize>, GCObject)>;

#[derive(Clone, Copy, Debug)]
struct GCHeader {
  marked: bool
}

impl GCObject {
  pub fn new(s: &mut LuaState, val: GCVariant) -> GCObject {
    let gch = GCHeader {
      marked: false
    };

    let obj = GCObject {
      ptr: Rc::new((Cell::new(gch), RefCell::new(val)))
    };

    s.heap.push(obj.clone());
    obj
  }

  fn mark(obj: &GCObject) {
    let (ref gch, ref val) = *obj.ptr;
  
    if gch.get().marked {
      return;
    }

    gch.set(GCHeader { marked: true, .. gch.get() });
  
    // TODO: Trace references and mark them
  }
}

fn obj_id(a: GCObject, b: GCObject) -> bool {
  let aptr = &*a.ptr as *const GCInnards;
  let bptr = &*b.ptr as *const GCInnards;

  aptr == bptr
}


impl Hash for GCObject {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    let (_, ref val) = *self.ptr;
    val.borrow().hash(state);
  }
}

impl PartialEq for GCObject {
  fn eq(&self, other: &Self) -> bool {
    let (_, ref a) = *self.ptr;
    let (_, ref b) = *other.ptr;
    a == b
  }
}

impl Eq for GCObject {}

