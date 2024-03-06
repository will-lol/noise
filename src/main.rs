fn main() {
    let foo = 5;
    let mut foo = 5;

    // This is a macro
    println!("Hello, noob!");

    // Loops in rust
    for i in 0..10 {

    }
    // TypeScript equivilant:
    // for (let i = 0; i < 10; i++) {
    //
    // }
    
    for i in 0..=10 {

    }
    // TypeScript equivilant:
    // for (let i = 0; i <= 10; i++) {
    //
    // }
    
    // You can assign ranges to variables
    let foo = 0..10;

    // A vector is rust's list
    let foo = vec![1, 2, 3];
    // We can get its iterator
    let iter = foo.iter();
    for x in iter {
    
    }

    // Lambdas 
    let foo = |x| {
        return 4
    };
    let foo = |x: i32| x*4;
    foo(3); // 12

    // Classes, seperate data and implementation
    struct Foo {
        pub size: u8,
        pub active: bool,
    }
    impl Foo {
        // Static fn
        pub fn do_stuff(num: i32) {}

        // Instance fn
        fn this(&self, num: i32) {}
        fn this(&mut self, num: i32) {}
        pub fn this(mut self) {}
    }

    // Traits are interfaces like Go
    trait Bar {
        fn method(&self) -> i32;
    }
    struct Baz {
        size: u8
    }
    impl Bar for Baz {
        fn method(&self) -> i32 {
           print!(self.size) 
        }
    }

    let foo = Baz {
        size: 1,
    };
    foo.method();

    // Destructering on struct
    let Baz { size } = foo;
    print!(size);

    let mut a = vec![1, 2, 3];
    a.get(2);

    // Tuple. Fixed structure.
    let a = (5, String::from("hello"));
    // Pattern matching
    fn bar((my_num, my_str): (u32, String)) {

    }
    bar(a)

}

