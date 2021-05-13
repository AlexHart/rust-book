struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    use_smart_pointer(&c);
    use_smart_pointer(&d);

    // How to use drop function manually,
    // can't use the one on the trait
    let e = CustomSmartPointer {
        data: String::from("Smart pointer forcedly dropped")
    };
    drop(e);
    println!("CustomSmartPointer e dropped before the end of main");

    println!("{}", c.data);
    println!("{}", d.data);
}

fn use_smart_pointer(x: &CustomSmartPointer) {
    println!("{}", x.data);
}
