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

    let ls = vec![1,2,3];

    let list_of_string: Vec<String> = ls.iter().map(
        |i| i.to_string()
    ).collect();
    
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

// we can rewirte this code as 

struct Clousre2<F> {
    data: (u8, u16),
    // Fn type is call function pointer.
    func: F
}

impl <F> Clousre2<F> 
    where F: Fn(&(u8, u16)) -> u8

    {
        fn call(&self) -> u8 {
            (self.func) (&self.data)
        }
    }
    
    

fn do_it2(data: &(u8, u64)) -> u8 {
    data.0
}

// function pointer

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn add_three(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)  + f(arg) + f(arg)
}

// function pointer implement the all of the closure traits (Fn, FnMut, FnOnce)


// fn main() {
//     add_two(add_one, 3);
// }


// im map we can use cluser inline or named functino pointer



fn returns_closure() -> Box<dyn Fn(i32) -> i32 >{
    Box::new(|x| x + 1)
}
   