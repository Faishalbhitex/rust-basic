fn main() {
    let separator = "=".repeat(20);
    // # Ownership concept 
    println!("\n\n#{separator} Immutable Ownership & Borrow {separator}#\n");
    // ## immutable ownership allocation s for string 'Hello Word'
    let s = String::from("Hello Word!"); // immutable ownership
    println!("s ownership value immutable string: '{}' ", s); // use value from own s

    // !ERROR example 1 immutable ownership
    // this will be error because 
    // immutable is fix and can't change or modify theri value
    //s.push_str("and Hello GPT/Calude!");

    println!("\n#==== Immutable Borrow ====#\n");
    // ### immutable from s ownership
    let  a = &s; // a borrow value immutable string ownership s (read only)
    let  b = &s; // b borrow value immutable string s ownership s  (read only)
    // for me is just like a and b pointer to address s 

    // !ERROR example 1 immutable ownership
    // event if borrow from ownership immutable will be same error occurs as before
    //a.push_str("and Hello Gemini/Grok!");
    //b.push_str("and Hello Copilot/Deepseek!");

    println!("Borrow immutable a and b from s ownership ( read only )");
    println!("\nValue immutable string:\nborrow a: '{}'\nborrow b: '{}'", a, b);
    // ownership s can use or display their own value because a and b 
    // are borrow from s not moved ownership itself
    println!("\ns ownership stil can use/display value immutable string: '{}' ", s); // use value from own s

    // ## afrer borrow a and b is done used we can move or replace ownership s to c 
    let c = s; // move ownership s to c and s no longer ownership 'Hello Word'
    // free(s) or s will  deallocation automatically
    println!("\nMove ownership s to c");
    println!("c become ownership. value immutable string: '{}'", c);   // use value from own c
    println!("s no longer ownership and can't use/display value immutable string");

    // !ERROR example ownership 
    // this will be error because ownership value s to c
    // because s is no longer have value immutable string 'Hello Word'
    // and s will deallocation automatically
    //println!("diplay or use value immutable string from s: {}", s);

    println!("\n\n#{separator} Mutable Ownership & Borrow {separator}#\n");
    let mut t = String::from("Faishal");
    println!("\n#--- Start Outer Scope ---#\n");
    println!("Create ownership t, value mutable string: {} Outer scope", t);
    println!("Ownership t before update value (modify): {}", t);

    {
        println!("\n#---- Start Inner Scope ----#\n");
        println!("\n#==== Mutable Borrow Full Access  ====#\n");
        println!("reference borrow mutable r1 (modify): Inner scope");
        let r1 = &mut t;
        println!("borrow r1 before update value (modify): {}", r1);
        r1.push_str(", Farhan");
        println!("add name Farhan in borrow r1(modify) value");
        println!("borrow r1 after update value (modify): {}", r1);
        println!("\n#---- End Inner Scope ----#\n");
    }

    t.push_str(", Firman");
    println!("add name Firman in ownership t (modify) value");
    println!("Ownership t after update value (modify): {}", t);

    println!("\n#==== Immutable Borrow Read Only  ====#\n");
    println!("reference borrow immutable r2 (read-only): Outer scope");
    let r2 = &t;
    println!("borrow r2 read only value ownership t: {}", r2);

    println!("\n#==== Mutable Borrow Full Access  ====#\n");
    println!("reference borrow mutable r3 (modify): Outer scope");
    let r3 = &mut t;
    println!("borrow r3 before update value (modify): {}", r3);
    r3.push_str(", Fadhil");
    println!("add name Fadhil in borrow r3(modify) value");
    println!("borrow r3 after update value (modify): {}", r3);

    
    println!("\n#==== Final Value Ownership t ====#\n");
    println!("ownership t: {}", t);
    println!("\n#-- End Outer Scope --#");
} // free(c) or c will deallocation automatically after out of scope 
