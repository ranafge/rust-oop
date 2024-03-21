fn main() {
    use oop::AveragedCollection;

    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);
    collection.average();
    collection.average();


}
//