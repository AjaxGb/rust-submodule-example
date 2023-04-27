mod foo;

use foo::Foo;

fn main() {
    let mut baz = Foo::new(42);
    println!("{:?}", baz);

    baz.increment();
    println!("{:?}", baz);
}
