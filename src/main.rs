fn main() {
    println!("Hello, world!");
    println!("baker telling recepie to person 1");
    paper();
    println!("baker telling recepie to person 2");
    paper();
    square(5);

    let number= 10;
    if number<0{
        println!("number is negative");
    }
    else if number==0{
        println!("number is zero");
    }
    else{
        println!("number is positive");
    }
    let mut counter =0;
    // loop{
    //     println!("hello world");
    //     counter = counter+1;
    //     if (counter==3) {
    //         break
    //     }
    // }
    while counter<3{
        println!("hello world");
        counter=counter+1;
    }
    for a in (0..5).rev(){
        println!("{},hello world",a);
    }
}
fn paper() {
    println!("1, add flour");
    println!("2,add sugar");
    println!("3,add milk");
    println!("4,stir it");
    println!("5,add eggs");
    println!("6,heat on gentle flame");
}
fn square(x:u32){
  let x =x*x;
  let result = x+1;
  println!("the square of x is {} and result is {}",x,result);
}