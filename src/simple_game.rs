use std::io;
extern crate rand;

use rand::Rng;

//from a java program I made 

	pub fn main() {
		println!("New Program.");
		
			println!("Welcome to a game.");
		
			println!("Please enter your name: ");
				let mut name = String::new();
				match io::stdin().read_line(&mut name) {
    					Ok(n) => {
						println!("{} bytes read", n);
      						  println!("Name: {}", name);
   					 }
   				 Err(error) => println!("error: {}", error),
				}				

		numgen(); // no errors in rust :)
	}
	pub fn numgen() {
		  //Random rand = new Random(); 
			let mut hp: i8 = 10;
			 let name: u8 = name;
	      // Generate random integers in range 0 to 50
    		let rng0 = rand::thread_rng().gen_range(0, 51);
    		let rng1 = rand::thread_rng().gen_range(0, 26);
		

	      println!("Starting HP: {}", hp);

	      // Print random integers 
	      if rng0 <= 20 {
	    	  println!("Dice Roll: {}", rng0);
	    	  hp -= 1;
	    	  println!("HP: {}", hp);
          }
        else{
          if rng0 <= 12 {
              println!("You took Damage again");
              hp -=1;
              println!("HP: {}", hp);
          }
        
	      else {
	    	  println!("Dice Roll: {}", rng0);
	    	  println!("{} lives a another day.", name);
	    	  println!("HP: {}", hp);
	      }
        }  
	      println!("2nd Dice Roll: {}", rng1); 
		if rng1 == 0 {
		hp = 0;
		println!("HP: {}", hp);
		}
		if hp == 0 {
		println!("You've died");
		}
	   
	      //do not place anything after this
}
