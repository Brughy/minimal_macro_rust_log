#[warn(unused_macros)]
#[macro_use] extern crate log;

/// Defined as : file, line, phrase(optional)
macro_rules! ftrace {
    ($($a: expr),*) => { format!("file: {} line: {} {}", file!(), line!(), format!("{}", $( format!("{}", $a))*)); }
}

/// Defined as : enable, level, phrase to print(optional)
macro_rules! plog {
		($a:expr, $b:expr $(, $c:expr )*) => {
			match $a {
				1 => match $b {
						"debug" =>  debug!( "{}", ftrace!($($c)*) ),
						"error" =>	error!( "{}", ftrace!($($c)*) ),
						"warn"  =>	warn!(  "{}", ftrace!($($c)*) ),
						"info"  =>  info!(  "{}", ftrace!($($c)*) ),
						_       =>	trace!( "{}", ftrace!($($c)*) ),
					}
				_ => (),
		   }
		}	          
}

fn main() {
	sensible_env_logger::init!();
    plog!(1, "debug", "");
    plog!(1, "error", "");    
    plog!(1, "warn", "");
    plog!(1, "info", "");    
    plog!(1, "trace", ""); 
    plog!(0, "debug", "");
    plog!(0, "error", "");    
    plog!(0, "warn", "");
    plog!(0, "info", "");    
    plog!(0, "trace", "");  
    plog!(1, "debug", "dgfiuwgf");
    plog!(1, "error", "mj,mk,");    
    plog!(1, "warn", "rfgrgr");
    plog!(1, "info", ",,o");    
    plog!(1, "trace", "xwdev");          
}
