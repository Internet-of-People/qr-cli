extern crate qrcode;
extern crate image;
extern crate ansi_term;
use std::env;
use qrcode::QrCode;
use ansi_term::Style;
use ansi_term::Colour::{Black, White};

fn consoleqr(text: &str, qz : bool, eclec : i32){
    let errorlev : qrcode::types::EcLevel;
    match eclec{
        1 => {
            errorlev = qrcode::types::EcLevel::L
        },
        2 => {
            errorlev = qrcode::types::EcLevel::M
        },
        3 => {
            errorlev = qrcode::types::EcLevel::Q
        },
        4 => {
            errorlev = qrcode::types::EcLevel::H
        },
        _ => {
            errorlev = qrcode::types::EcLevel::M
        },
    };
    let code = QrCode::with_error_correction_level(text.as_bytes(), errorlev).unwrap();
    let string : String = code.render::<char>()
        .quiet_zone(qz)
        .dark_color(' ')
        .light_color('█')
        .module_dimensions(2, 1)
        .build();
    println!("{}", Style::new().fg(White).on(Black).paint(string));
}

fn arg_to_int(arg : &str)->i32{
    match arg.parse(){
        Ok(int)=>{
            int
        },
        Err(_)=>{
            println!("Error: wrong error correction parameter(not int)");
            2
        }
    }
}
fn arg_to_bool(arg : String)->bool{
    match arg.parse(){
        Ok(bool)=>{
            bool
        },
        Err(_)=>{
            println!("Error: wrong quiet zone parameter(not bool)");
            true
        }
    }
}

fn main(){
    let mut args = env::args();
    match env::args().count(){
        2 => {
            consoleqr(&args.nth(1).unwrap_or_default(), true, 2);
        },
        3 => {
            consoleqr(&args.nth(1).unwrap_or_default(), arg_to_bool(args.nth(0).unwrap_or_default()), 2);
        },
        4 => {
            consoleqr(&args.nth(1).unwrap_or_default(), arg_to_bool(args.nth(0).unwrap_or_default()), arg_to_int(&args.nth(0).unwrap_or_default()));
        },
        _ => {
            println!("Up to 3 parameters
            1. paremeter: what you want to qr encode.
            2. parameter: if you want quiet zone(true/false otherwise true)
            3. parameter: level of error correction(1-4 otherwise 2) ");
        },

    }
}
