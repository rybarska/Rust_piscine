fn main() {
    if cfg!(debug_assertions)
    	{println!("Hey! I'm the other bin target!");}
    else
    	{ println!("Hey! I'm the other bin target!\nI'm in release mode!"); }
}
