extern crate getset;

use getset::{Getters, MutGetters, CopyGetters, Setters};

#[derive(Getters, Setters, MutGetters, CopyGetters, Default)]
pub struct Foo<T>
where
    T: Copy + Clone + Default,
{
    /// Doc comments are supported!
    /// Multiline, even.
    #[get]
    #[set]
    #[get_mut]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    #[get_copy = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    public: T,
}

fn main() {
    let mut foo = Foo::default();
    foo.set_private(1);
    (*foo.private_mut()) += 1;
    assert_eq!(*foo.private(), 2);
    println!("{}",*foo.private());
}

