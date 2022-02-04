
#![warn(arithmetic_overflow)]

// Rust allows another macro type: derive. It allows to "auto-implement"
// supported traits. Clone, Debug, Copy are typically handy to derive.
#[derive(Clone, Debug, Copy)]
struct MyCustomStruct {
    a: i32,
    b: u32,
    pub c: f32
}

// A typical Rust struct has an impl block for behaviour
impl MyCustomStruct {

    // The new function is static function, and by convention a 
    // constructor
    pub fn new(a: i32, b: u32, c: f32) -> MyCustomStruct {
        MyCustomStruct {
            a: a, b: b, c: c
        }
    }

    // Instance functions feature a "self" reference as the first
    // parameter
    // This self reference can be mutable or owned, just like other
    // variables
    pub fn sum(&self) -> f32 { 
        self.a as f32 + self.b as f32 + self.c
    }
}


#[cfg(test)]
mod tests {

    use super::MyCustomStruct;
    use std::mem;
    
    #[test]
    fn test_custom_struct() {
        assert_eq!(mem::size_of::<MyCustomStruct>(),
            mem::size_of::<i32>() + mem::size_of::<u32>() +
            mem::size_of::<f32>());
    
        let m = MyCustomStruct::new(1, 2, 3_f32);
        assert_eq!(m.a, 1);
        assert_eq!(m.b, 2);
        assert_eq!(m.c, 3_f32);
    
        assert_eq!(m.sum(), 6_f32);
        let m2 = m.clone();
        assert_eq!(format!("{:?}", m2), "MyCustomStruct { a: 1, b: 2, c: 3.0 }");
    
        let mut m3 = m;
        m3.a = 100;
    
        assert_eq!(m2.a, 1);
        assert_eq!(m.a, 1);
        assert_eq!(m3.a, 100)
    
    }


    #[test]
    fn basic_math_stuff(){
        assert_eq!(2 + 2, 4);
    
        assert_eq!(3.14 + 22.86, 26_f32);
    
        assert_eq!(2_i32.pow(2), 4);
        assert_eq!(4_f32.sqrt(), 2_f32);
    
        let a: u64 = 32;
        let b: u64 = 64;
    
        // Risky, this could overflow
        assert_eq!(b - a, 32);
        assert_eq!(a.overflowing_sub(b), (18446744073709551584, true));
        let mut c = 100;
        c += 1;
        assert_eq!(c, 101);
    }
  

    #[test]
    #[should_panic]
    fn attempt_overflows(){
        let a = 10_u32;
        let b = 11_u32;
    
        // This will panic since the result is going to be an
        // unsigned type which cannot handle negative numbers
        // Note: _ means ignore the result
        let _ = a - b;
    }
        
    
}



