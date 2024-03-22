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
pub struct Screen {
    // Box not only value assign to heap memory with ownership  but also get fix know size.

    pub components: Vec<Box<dyn Draw>>
    // pub components  : Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw()
        }
    }
}

// Generic type

// impl <T> Screen <T>
//     where
//         T: Draw
// {
//     pub fn run(&self) {
//         for component in &self.components {
//             component.draw()
//         }
//     }
    
// }


// Implementing the trait

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("{} {} ", self.width, self.height)
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>
}


impl Draw for SelectBox {
    fn draw(&self) {
        println!("{} {}", self.width, self.height)
    }
}



// State design pattern

pub struct Post {

    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new(&self)  -> Post{
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
    }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {

    
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())

        }
            
    }
    
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    }

struct PendingReview{}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#defining-post-and-creating-a-new-instance-in-the-draft-state