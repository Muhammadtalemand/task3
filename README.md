# Third task
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`task3a = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "task3a"
version = "0.1.0"
authors = ["muhammadtalemand <talemandkhokhar@gmail.com>"]
edition = "2018"

[dependencies]
task3a = "0.1.0"
```
In `src/lib.rs` you can use like this:

```
pub mod grandfather{
    pub mod father{
        pub fn childern(){
            println!("Your granfather was a LandLord");
        }
    }
}
```

In `src/main.rs` you can use like this:

```
//using library package and call locally
use task3;
//using use keyward
use task3::grandfather::father;

fn main() {
    
    crate::task3::grandfather::father::childern();//absolute path
    father::childern();
}
```

now `cargo run` for results
