
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc::*;
use std::thread::JoinHandle;

fn main() {
    // Read in data
    let (width,height,data): (u32,u32,Vec<f64>) = read_in_file().unwrap();
    let data2 = data.clone();

    // VEC
    let mut total_vec:f64 = 0.0; 
    let runs = 200;
    for _ in 0..runs {
        let now = Instant::now();
        compute_threaded_vec(width,height,&data);
        let new_now = Instant::now();
        let time = new_now.duration_since(now).as_secs_f64();
        total_vec= total_vec+time;
    }   
    println!("Average time Vec   :{:?} ", total_vec/(runs as f64));


    // Slice
    let mut total_slice:f64 = 0.0; 
    let runs = 200;
    for _ in 0..runs {
        let slice = &data2[..];
        let now = Instant::now();
        findbasin_slice(width,height,slice);
        let new_now = Instant::now();
        let time = new_now.duration_since(now).as_secs_f64();
        total_slice= total_slice+time;
    }

    println!("Average time Slice :{:?} ", total_slice/(runs as f64));
    println!("Speedup :{:?} ", total_vec/total_slice );
}

//   _                        _   _____          _         
//  | |                      | | |  __ \        | |        
//  | |      ___    __ _   __| | | |  | |  __ _ | |_  __ _ 
//  | |     / _ \  / _` | / _` | | |  | | / _` || __|/ _` |
//  | |____| (_) || (_| || (_| | | |__| || (_| || |_| (_| |
//  |______|\___/  \__,_| \__,_| |_____/  \__,_| \__|\__,_|
                                                        
                                                   

fn read_in_file() -> std::io::Result<(u32,u32,Vec<f64>)> {

    let f = File::open("./Data/large_in.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let dimensions:Vec<u32> = line.split_ascii_whitespace().into_iter()
                                .map(|x| parse_u32(x))
                                .collect();
    let width = *dimensions.get(0).unwrap();
    let height = *dimensions.get(1).unwrap();

    let mut linenew = String::new();
    reader.read_line(&mut linenew)?;
    

    let data: Vec<f64>= linenew.split_whitespace()
                                .into_iter()
                                .map(|x| parse_thing(x))
                                .collect();
    println!("line2: {}",data.len());

    Ok((width,height,data))
}

fn parse_thing(input: &str ) -> f64 {
    input.parse::<f64>().unwrap()
}

fn parse_u32(input: &str ) -> u32 {
    input.parse::<u32>().unwrap()
}






//  __      __         
//  \ \    / /         
//   \ \  / /___   ___ 
//    \ \/ // _ \ / __|
//     \  /|  __/| (__ 
//      \/  \___| \___|
                    
fn compute_threaded_vec(width:u32,height:u32,data:&Vec<f64>){
    
    // let num_threads =10;
    // let handlearr:Vec<JoinHandle<()>> = Vec::new();

    // // for i in 1..num_threads{
    // let thread_n = thread::Builder::new()
    //                                                         .name("Listen".to_string())
    //                                                         .spawn(move || vec_thread(1,0,0,0,data) );
    // // }

    // let handle = thread::spawn(|| {
    //         for i in 1..10 {
    //             println!("hi number {} from the spawned thread!", i);
    //             thread::sleep(Duration::from_millis(1));
    //         }
    // });

}

fn vec_thread(threadnum:u8, r:u32, c:u32, w:u32, data:&'static std::vec::Vec<f64>) ->  Vec<String>{

    return vec![String::new()];
}

                   


//  VEC
fn findbasin(width:u32,height:u32,data:&Vec<f64>){

    for r in 1..height-1 {
        for c in 1..width-1 {
            if isbasin(r,c,width,&data){
                // println!("{}  {}",r,c);
            }
        }
    }

}

fn isbasin(r:u32,c:u32,w:u32,data:&Vec<f64>)-> bool{
    
    let cen:u32 = r*w+c as u32;
    let cen_cen = data.get(cen as usize).unwrap() + 0.01;
    let cen_l = *data.get((cen-1) as usize).unwrap();
    let cen_r = *data.get((cen+1) as usize).unwrap();

    let top:u32 = ((r-1)*w +c) as u32;
    let top_m = *data.get((top) as usize).unwrap();
    let top_l = *data.get((top-1) as usize).unwrap();
    let top_r = *data.get((top+1) as usize).unwrap();

    let bot:u64 = ((r+1)*w +c) as u64;
    let bot_m = *data.get((bot) as usize).unwrap();
    let bot_l = *data.get((bot-1) as usize).unwrap();
    let bot_r = *data.get((bot+1) as usize).unwrap();


    let cen_check = cen_cen <= cen_l && cen_cen <= cen_r;
    let top_check = cen_cen <= top_l && cen_cen <= top_r && cen_cen <= top_m;
    let bot_check = cen_cen <= bot_l && cen_cen <= bot_r && cen_cen <= bot_m;

    if cen_check && top_check && bot_check {
        return true;
    }
    return false;
}





//    _____  _  _            
//   / ____|| |(_)           
//  | (___  | | _   ___  ___ 
//   \___ \ | || | / __|/ _ \
//   ____) || || || (__|  __/
//  |_____/ |_||_| \___|\___|
                          

//  SLICE
fn findbasin_slice(width:u32,height:u32,data:&[f64]){
    for r in 1..height-1 {
        for c in 1..width-1 {
            if isbasin_slice(r,c,width,data){
                // println!("{}  {}",r,c);
            }
        }
    }
}


fn isbasin_slice(r:u32,c:u32,w:u32,data:&[f64])-> bool{
    
    let cen:u32 = r*w+c as u32;
    let cen_cen = data.get(cen as usize).unwrap() + 0.01;
    let cen_l = *data.get((cen-1) as usize).unwrap();
    let cen_r = *data.get((cen+1) as usize).unwrap();

    let top:u32 = ((r-1)*w +c) as u32;
    let top_m = *data.get((top) as usize).unwrap();
    let top_l = *data.get((top-1) as usize).unwrap();
    let top_r = *data.get((top+1) as usize).unwrap();

    let bot:u64 = ((r+1)*w +c) as u64;
    let bot_m = *data.get((bot) as usize).unwrap();
    let bot_l = *data.get((bot-1) as usize).unwrap();
    let bot_r = *data.get((bot+1) as usize).unwrap();


    let cen_check = cen_cen <= cen_l && cen_cen <= cen_r;
    let top_check = cen_cen <= top_l && cen_cen <= top_r && cen_cen <= top_m;
    let bot_check = cen_cen <= bot_l && cen_cen <= bot_r && cen_cen <= bot_m;

    if cen_check && top_check && bot_check {
        return true;
    }
    return false;
}
