fn main() {
    println!("Simon is saying Hello to the world!");

    //----------------_________________-------------------- INTRODUCTION TO RUST ---------------------____________________------------------>
    // Variables are declared using the let keyword
    println!(" Rust will be able to infer the type of your variable");
    let a = "Simon";
    let b = 40;
    let c = 'c';
    let d = 20.5;
    let f = true;
    println!("String:{}", a);
    println!("Integer:{}", b);
    println!("Character:{}", c);
    println!("Floating Point:{}", d);
    println!("Boolean:{}", f);
    println!();

    println!(" Explicit Declaration of Variables");
    //Signed Integers(i8,132,i64,i128)
    let g: i32 = -9;
    println!("Signed Int:{}", g);
    //Unsigned Integers(u8.u32,u64,u128)
    let h: u64 = 10;
    println!("Unsigned Int:{}",h);
    //floating points(f32,f64)
    let i: f32 = 7.85;
    println!("Floating Point:{}",i);
    println!();
    
    //<--------------________________----------------------- STRINGS ---------------___________________---------------------->
    //Strings and &str
    println!("We have strings and string slices");
    // Strings can be modified and changed when made mutable using String 
    //functions i.e .push_str() and therefore are used for string values 
    //that might keep on changing. String slices on the other hand are used 
    //for string values that are used a lot and should not be modified.

    //Different ways of declaring Strings in rust 
    //Declaration 1
    let k = String::from("This is a string");

    //Declaration 2
    let k = "This is a string". to_string();

    //Declaration 3
    //Format macro can be used to compose a string from different parts 
    let k = format!("This is a string");
    let l = format!("{} plus {} is {}",1,2,1+2);
    println!("{}",k);
    println!("{}",l);

    //String Slice 
    //Declaration 1
    let j: &str ="This is a string slice";
    println!("String slice : {}",j);

    //Declaration 2
    let string_slice="This is another string slice";
    println!("{}",string_slice);

     //Declaration 3
     // you can get a slice by creating a reference to a String
     let p: &str = &j;
     println!("{}",p);
     println!();

    //String functions
    //concatenating with a single character using .push
    let mut n = format!("Hell");
    n.push('o');
    println!("{}",n);

    //concatenating with another string using .push_str
     let mut m = String::from("hi");
     m.push_str(" there");
     println!("{}",m);
     println!();

     //Constants 
     //Should be in capital seperated by a  
     //must give the datatype
     const PI_OF_A_CIRCLE: f32 = 3.142;
     println!("{}",PI_OF_A_CIRCLE);
     println!();

     //-----------------___________________--------------- TUPLES ---------------_________________----------------->
     //can hold multiple data types, accessed via indexing
     // name: (datatypes) = (values);
     let q: (u16,&str,bool) = (12,"hey",false);
     println!("{}",q.2);
     //to change values within a tuple declare the tuple as mutable
     let mut r: (u16,&str,bool) = (12,"hey",false);
     r.2 = true;
     println!("{}",r.2);
     println!();

     //----------------___________________---------------- ARRAYS ---------------___________________----------------->
    //only hold a specified number of values that belong to the same datatype 
    // declare as mutable if you want to change values 
    // name: [type;no of values] = [values];
    let s: [u32;4] = [10,20,30,40];
    println!("{}",s[0]);
    println!();
    //we can still declare and initialize a string without having to define the datatypes and no of elements 
    let mut t = [1,2,3,4,5,6,7];
    t[0] = 50;
    println!("{}",t[0]);
    println!(); 

    //----------------_________________---------------- SCOPE --------------________________------------------->
    // variable shadowing illustarated using inner and outer scope
    //outer scope can access inner scope but inner scope cannot access outer 
    let e = 2;
    println!("{}", e);
    {
        let e = e + 1;
        println!("{}", e);
        
        let e = e + 1;
        println!("{}", e);

    }
    let e = e + 3;
    println!("{}", e);


    








}
