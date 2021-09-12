mod sublib;

pub mod parent_mod {
    pub use crate::sublib::child_mod as mode;

    pub fn parent_fun() {
        println!("Parent");
        mode::child_func();
    }
}


