fn main() {
    let mut x = 5;
    { //Create a new scope
        let y = &mut x; 
        *y = 6;
    }
    { //Create another scope
        let z = &mut x;  
        *z = 7;
    } 
}