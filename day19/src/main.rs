use std::marker::PhantomData;
// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<STATE> {
    _state: PhantomData<STATE>, 
}


// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Sleigh<Empty> {
        Sleigh {
            _state: PhantomData
        }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData
        }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh {
            _state: PhantomData
        }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh {
            _state: PhantomData
        }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData
        }
    }
}