extern crate communicator;

fn main(){
    communicator::client::connect();
}


mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    // middle_function 函数公有，根据第一条原则，可以访问
    outermost::middle_function();
    // middle_secret_function 函数是私有的，根据第二条定义，它只能被 outermost及其子模块访问，try_me所属的根模块不符合，发生编译错误
    // outermost::middle_secret_function();
    // inside私有且没有子模块，所以只能被outermost访问，错误
    // outermost::inside::inner_function();
    // outermost::inside::secret_function();
}