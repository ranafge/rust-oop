use std::process::Output;

pub trait Ord: Eq + PartialOrd {
    fn cmp(self, other: Self) -> Ordering;

}

pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1
}



fn main() {

    let col = Clousre {
        data: (1,2), fun: do_it
    };
    println!("{}", col.call());
    
}

fn compute(input: &i32, output: &mut i32) {
    if *input  > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output = 2;
    }
}


// Rust's Fn traits are little bit magic. For instace, we can write the following code.

struct Clousre<F> {
    data: (u8, u16),
    fun: F,
    
}

impl <F> Clousre<F> 
    where
        F:  Fn(&(u8, u16)) -> &u8,


{
    fn call(&self) -> &u8 {
        (self.fun) (&self.data)

    }
    
    
}

fn do_it (data: &(u8, u16)) -> &u8 {
    &data.0
}