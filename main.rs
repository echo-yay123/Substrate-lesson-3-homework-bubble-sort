//use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Bubble sort start @_@");

    //generate array 
        
    const ARR_LEN : usize = 10;

    let mut vec = vec![0;ARR_LEN];

    println!("The origin numbers are:");
    for i in 0..ARR_LEN{
        vec[i] = rand::thread_rng().gen_range(1..=100);
        println!("{}",vec[i]);
    }

    //bubble sort
    let mut swapped = false;

    for i in 0..(ARR_LEN-1){
        
        for j in 0..(ARR_LEN-1-i){
            
            match vec[j].cmp(&vec[j+1]){
                Ordering::Less => {
                    vec.swap(j,j+1);
                    swapped = true;
                },
                Ordering::Greater => continue,
                Ordering::Equal => continue,
            }
        }
        
        if !swapped {
           break;  
        }
    }

    
    //show result
    println!("The discending order array:");
    for i in 0..ARR_LEN
    {
        println!( "{}",vec[i]);
    }


}
