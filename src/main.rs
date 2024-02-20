fn main() {
    // Recursive structure must be boxed.
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}",list);
}


// creating recursive data structre
#[derive(Debug)]
enum List <T> {
    Cons(T, Box<List<T>>), // recursive type being call continuously  so its need to put in box 
    Nil
}