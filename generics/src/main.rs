use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, RwLock};



struct Point<T,X> {
    x: T,
    y: X,
}

struct Ponto<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl<T> Ponto<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Ponto<f32> {
    fn get_y(&self) -> &f32 {
        &self.y
    }
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn get_middle(list: Vec<i32>) -> Result<i32, &'static str> {
    match list.len() {
        0 => Err("empty list"),
        x if x% 2 == 0 => Err("List has even number of elements"),
        _ => Ok(list[list.len() / 2]),
    }
}


fn main() {
    println!("Hello, world!");

    let number_list = vec![61, 45, 88, 22, 10];

    // T is explicitly specified
    let res = largest::<i32>(&number_list);
    println!("The largest number is: {}", res);

    let char_list = vec!['y', 'a', 'b', 'd', 'u'];

    // T can be inferred by the compiler
    let res = largest(&char_list);
    println!("The largest char is: {}", res);

    let _int = Point { x: 3, y: 5.2 };
    let _float = Point { x: 2.3, y: 4 };

    let _some_int = Option::Some(25);
    let _some_float = Option::Some(4.3);

    let _nil: Option<i32> = Option::None;

    let p = Ponto { x: 5.2, y: 10.4 };

    println!("Value of X is: {}", p.get_x());
    println!("Value of Y is: {}", p.get_y());


    println!("{:?}", get_middle(vec![1, 2, 3]));
    println!("{:?}", get_middle(vec![1, 2, 3, 4]));
    println!("{:?}", get_middle(Vec::new()));


    // heap allocated pointers
    let mybox: Box<i32> = Box::new(42);
    let myrc: Rc<i32> = Rc::new(42);

    // Cell with internal mutability
    let mycell: Cell<i32> = Cell::new(42);
    // mycell.set(0);
    // mycell.get();
    let myrefcell: RefCell<i32> = RefCell::new(42); // with read-write lock
    // myrefcell.borrow()
    // myrefcell.borrow_mut()

    //Thread-safe version
    let myarc: Arc<i32> = Arc::new(42); // automic reference counted Rc<T>
    let mymutex: Mutex<i32> = Mutex::new(42);
    // mutex.lock();
}
