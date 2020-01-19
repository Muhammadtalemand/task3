//using library package and call locally
use task3;
//using use keyward
use task3::grandfather::father;

fn main() {
    
    crate::task3::grandfather::father::childern();//absolute path
    father::childern();
}
