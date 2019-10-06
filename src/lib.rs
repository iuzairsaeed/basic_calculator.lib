mod calculator_functions{
    pub mod basic_functions{
        pub fn add(x:i32 , y:i32)  -> i32 {
            let z:i32 = x+y;
            z
        }
        pub fn substract(x:i32 , y:i32) -> i32{
            let z:i32 = x-y;
            z
        }
        pub fn divide(x:i32 , y:i32) -> i32{
            let z:i32 = x/y;
            z
        }        
        pub fn multiply(x:i32 , y:i32) -> i32{
            let z:i32 = x*y;
            z
        }
    }
    pub mod power_functions{
        pub fn square_function(x:i32) -> i32{
            let z:i32 = x*x;
            z
        }
        pub fn cube_function(x:i32) -> i32{
            let z:i32 = x*x*x;
            z
        }
        
        pub fn power_function(x:i32 , y:i32) -> i32{
            let mut z:i32 = 0;
            for _i in 1..y+1{
                z = x*x;
                println!("{}" , _i);
            }
            z
        }
    }
}

