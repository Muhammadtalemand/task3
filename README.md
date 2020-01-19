# First task
this is a demo rust library published on crates.io

to use this library you have to add following line in dependency section of cargo.toml

`task1 = "0.1.0"`

your cargo.toml file should look like this:
```
[package]
name = "task1"
version = "0.1.0"
authors = ["muhammadtalemand <talemandkhokhar@gmail.com>"]
edition = "2018"

[dependencies]
task1 = "0.1.0"
```

In `src/main.rs` you can use like this:

```
mod ThisPc{
    pub mod Localdisk{
       pub fn pics(){
            println!("Please display the Talemand`s pics");
        }
    } 
}
//using use we can decrease the lenght of paths
use ThisPc::Localdisk;

fn main() {
    crate::ThisPc::Localdisk::pics(); //absolute path
    ThisPc::Localdisk::pics();

    //using use keyward we can access directly
    Localdisk::pics();
}
```

now `cargo run` for results
