fn main() {
    // Recursive structure must be boxed.
    // let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    // println!("{:?}",list);
}


// creating recursive data structre
// #[derive(Debug)]
// enum List <T> {
//     Cons(T, Box<List<T>>), // recursive type being call continuously  so its need to put in box 
//     Nil
// }

// To associate actual code with a type, we use imple blocks

// :: <- namespacing seperator 

/*
 
fn foo(self, arg: Type1) -> ReturnType {
    // body
}
*/

// There are 3 forms that foo can take: self, &mut self, , &self

/*
    1. self - Value
    2. &mut self - mutable reference
    3. &self - share reference

*/

// pub enum List {
//     Empty,
//     More(Box<Node>),    
 
// }


pub trait Drop {
    fn drop(&mut self) ;
}