#![allow(warnings)]
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};
// use std::thread;
// use std::sync::mpsc::*;
// use std::thread::JoinHandle;
use rayon::prelude::*;

fn main() {

    print!("Reading Data in ...");
    let (width2,height2,data2D): (u32,u32,Vec<Vec<f32>>) = read_in_file2D().unwrap();
    println!("Done!");

    // VEC
    let mut total_vec:f32 = 0.0; 
    let runs = 200;
    for _ in 0..runs {
        let now = Instant::now();
        vec_rayon(width2,height2,&data2D);
        let new_now = Instant::now();
        let time = new_now.duration_since(now).as_secs_f32();
        total_vec= total_vec+time;
    }

    println!("Average time Vec parallel   :{:?} ", total_vec/(runs as f32));

    // Slice
    // let mut total_slice:f32 = 0.0; 
    // let runs = 200;
    // for _ in 0..runs {
    //     let slice = &data2[..];
    //     let now = Instant::now();
    //     findbasin_slice(width,height,slice);
    //     let new_now = Instant::now();
    //     let time = new_now.duration_since(now).as_secs_f32();
    //     total_slice= total_slice+time;
    // }

    // println!("Average time Slice :{:?} ", total_slice/(runs as f32));
    // println!("Speedup :{:?} ", total_vec/total_slice );

}

//   _                        _   _____          _         
//  | |                      | | |  __ \        | |        
//  | |      ___    __ _   __| | | |  | |  __ _ | |_  __ _ 
//  | |     / _ \  / _` | / _` | | |  | | / _` || __|/ _` |
//  | |____| (_) || (_| || (_| | | |__| || (_| || |_| (_| |
//  |______|\___/  \__,_| \__,_| |_____/  \__,_| \__|\__,_|
                                                        
                                                   

fn read_in_file() -> std::io::Result<(u32,u32,Vec<f32>)> {

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
    

    let data: Vec<f32> = linenew.split_whitespace()
                                .into_iter()
                                .map(|x| x.parse::<f32>().unwrap())
                                .collect();

 
    Ok((width,height,data))
}

fn read_in_file2D() -> std::io::Result<(u32,u32,Vec<Vec<f32>>)> {

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

    let data: Vec<f32> = linenew.split_whitespace()
                                .into_iter()
                                .map(|x| x.parse::<f32>().unwrap())
                                .collect();

    let mut data2D: Vec<Vec<f32>> = Vec::new();

    for r in 0..height{
        let mut innerVec: Vec<f32> = Vec::new();
        for c in 0..width{
            let value:f32 = data[(r*width+c) as usize];
            innerVec.push(value); 
        }
        data2D.push(innerVec);
    }

    Ok((width,height,data2D))

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
                    
fn vec_rayon(width:u32,height:u32,data:&Vec<Vec<f32>>){
    
    let x:Vec<(u32,u32)> = data.par_iter()
                .enumerate()
                .map(|(row,value)| findbasin(width,height,row,data))
                .flatten()
                .collect();
}

//  VEC
fn findbasin(width:u32, height:u32, row:usize, data:&Vec<Vec<f32>>) -> Vec<(u32,u32)>{
    let mut result: Vec<(u32,u32)> = Vec::new();
    
    if row==0 || row as u32 == height-1{
        return Vec::new();
    }

    for c in 1 .. width-1 {
        if isbasin(row,c as usize,&data){
            result.push((row as u32,c))
        }
    };
    result
}

fn isbasin(r:usize,c:usize,data:&Vec<Vec<f32>>)-> bool{

    // Indexing straight is considered dangerous and can cause crashes
    // Just like C++ or Java
    let cen_m:f32 = data[r][c] + 0.01;
    let cen_l:f32 = data[r][c-1];
    let cen_r:f32 = data[r][c+1];

    let top_m:f32 = data[r-1][c];
    let top_l:f32 = data[r-1][c-1];
    let top_r:f32 = data[r-1][c+1];

    let bot_m:f32 = data[r+1][c];
    let bot_l:f32 = data[r+1][c-1];
    let bot_r:f32 = data[r+1][c+1];

    let cen_check = cen_m <= cen_l && cen_m <= cen_r;
    let top_check = cen_m <= top_l && cen_m <= top_r && cen_m <= top_m;
    let bot_check = cen_m <= bot_l && cen_m <= bot_r && cen_m <= bot_m;

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
                          

// //  SLICE
// fn findbasin_slice(width:u32,height:u32,data:&[f32]){
//     for r in 1..height-1 {
//         for c in 1..width-1 {
//             if isbasin_slice(r,c,width,data){
//                 // println!("{}  {}",r,c);
//             }
//         }
//     }
// }


// fn isbasin_slice(r:u32,c:u32,w:u32,data:&[f32])-> bool{
    
//     let cen:u32 = r*w+c as u32;
//     let cen_cen = data.get(cen as usize).unwrap() + 0.01;
//     let cen_l = *data.get((cen-1) as usize).unwrap();
//     let cen_r = *data.get((cen+1) as usize).unwrap();

//     let top:u32 = ((r-1)*w +c) as u32;
//     let top_m = *data.get((top) as usize).unwrap();
//     let top_l = *data.get((top-1) as usize).unwrap();
//     let top_r = *data.get((top+1) as usize).unwrap();

//     let bot:u64 = ((r+1)*w +c) as u64;
//     let bot_m = *data.get((bot) as usize).unwrap();
//     let bot_l = *data.get((bot-1) as usize).unwrap();
//     let bot_r = *data.get((bot+1) as usize).unwrap();


//     let cen_check = cen_cen <= cen_l && cen_cen <= cen_r;
//     let top_check = cen_cen <= top_l && cen_cen <= top_r && cen_cen <= top_m;
//     let bot_check = cen_cen <= bot_l && cen_cen <= bot_r && cen_cen <= bot_m;

//     if cen_check && top_check && bot_check {
//         return true;
//     }
//     return false;
// }
