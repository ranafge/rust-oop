use oop::{Button, Screen};

fn main() {
    use oop::AveragedCollection;

    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);
    collection.average();
    collection.average();

    use oop::Draw;
    use oop::Button;
    use oop::SelectBox;
 
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("Button")
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            })
            
        ],
        
    };
    println!("{:?}",screen.run());
    println!("Finished main.")

        

    
}
//