// This struct is public but inner field is private
// This mean if you need to add , remove or update the list field
// You have to implement a method like add, or remove 

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {

        let result = self.list.pop() ;
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None
        }


    }   
    pub fn average(&self) -> f64{
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}


pub trait Draw {
    fn draw(&self);
}
/*
    dyn : 

    In computer science, dynamic dispatch is the process of selecting which implementation of a polymorphic operation (method or function) to call at run time.
     It is commonly employed in, and considered a prime characteristic of, object-oriented programming (OOP) languages and systems.[1]

*/
pub struct Screen <T>{
    // Box not only value assign to heap memory with ownership  but also get fix know size.

    pub components: Vec<T>
    // pub components  : Vec<Box<dyn Draw>>
}

// impl Screen {
//     pub fn run(&self) {
//         for component in &self.components {
//             component.draw()
//         }
//     }
// }

// Generic type

impl <T> Screen <T>
    where
        T: Draw
{
    pub fn run(&self) {
        for component in &self.components {
            component.draw()
        }
    }
    
}


// Implementing the trait