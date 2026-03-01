use std::io;

fn print_box(pos: &[i32; 2]) {
    for r in 0..10 {
        for c in 0..10 {
            //drawing time
            //the whole if/else block evaluates to a character
            let ch = 
            if r == pos[0] && c == pos[1] { 'o' }
            else { '.' };
            print!("{}", ch);
        }
        println!();
    }
}



fn main() {
    let mut pos: [i32; 2] = [2, 1];
    let mut cmd = String::new();

    loop {
        print_box(&pos); 
        println!("\nWhich direction? Which direction? (u)p (d)own (l)eft (r)right (q)uit");
        
        cmd.clear(); //we need to clear because read_line appends to the end of the string
        io::stdin().read_line(&mut cmd).expect("Failed");
        
        match cmd.trim() { //trim to remove \n
            "u" => pos[0] -= 1,
            "d" => pos[0] += 1,
            "l" => pos[1] -= 1,
            "r" => pos[1] += 1,
            "q" => break,
            _   => println!("Unknown command"),
        }
    }
}





