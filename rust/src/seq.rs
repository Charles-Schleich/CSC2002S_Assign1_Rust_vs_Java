#![allow(warnings)]
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};



fn main() {
    let file = String::from("./Data/large_in.txt");
    let output = String::from("./Data/large_out.txt");

    // Read in data
    let (width,height,data): (u32,u32,Vec<f32>) = read_in_input(file).unwrap();
    let data2 = data.clone();

    let mut total_vec:f32 = 0.0; 
    let runs = 100;
    
    println!("Running {} times ", runs );
    let mut result_vec: Vec<String> = Vec::new();
    for _ in 0..runs {
        let now = Instant::now();
        result_vec = find_basin_vec(width,height,&data);
        let time = Instant::now().duration_since(now).as_secs_f32();
        total_vec= total_vec+time;
    }


    let mut total_slice:f32 = 0.0; 
    let mut result_slice: Vec<String> = Vec::new();
    for _ in 0..runs {
        let slice = &data2[..];
        let now = Instant::now();
        result_slice = find_basin_slice(width,height,slice);
        let time = Instant::now().duration_since(now).as_secs_f32();
        total_slice= total_slice+time;
    }

    println!("Results\n------");
 
    let (expectedNumBasins, expectedBasins) : (u32,Vec<String>) = read_in_output(output).unwrap();
    println!("Vec Expected Number {:?}, Actual {}", expectedNumBasins, result_vec.len());
    validate_output(&expectedBasins,result_vec);
    println!("Slice Expected Number {:?}, Actual {}", expectedNumBasins, result_slice.len());
    validate_output(&expectedBasins,result_slice);

    println!("Performance\n--------------", );
    println!("Average time Vec   :{:?} ", total_vec/(runs as f32));
    println!("Average time Slice :{:?} ", total_slice/(runs as f32));
    println!("Speedup :{:?} ", total_vec/total_slice );

}


//   _            _                         
//  | |          | |                        
//  | |__    ___ | | _ __    ___  _ __  ___ 
//  | '_ \  / _ \| || '_ \  / _ \| '__|/ __|
//  | | | ||  __/| || |_) ||  __/| |   \__ \
//  |_| |_| \___||_|| .__/  \___||_|   |___/
//                  | |                     
//                  |_|         

fn read_in_input(file: String) -> std::io::Result<(u32,u32,Vec<f32>)> {

    let f = File::open(file)?;
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
    

    let data: Vec<f32>= linenew.split_whitespace()
                                .into_iter()
                                .map(|x| parse_f32(x))
                                .collect();
    Ok((width,height,data))
}

fn parse_f32(input: &str ) -> f32 {
    input.parse::<f32>().unwrap()
}

fn parse_u32(input: &str ) -> u32 {
    input.parse::<u32>().unwrap()
}

fn read_in_output(file:String) -> std::io::Result<(u32,Vec<String>)> {

    let f = File::open(file)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let line = line.trim_end_matches("\n");
    let number_of_results = parse_u32(&line);

    let mut arr : Vec<String> = Vec::new();

    for _ in 0..number_of_results {
        let mut linenew = String::new();
        reader.read_line(&mut linenew)?;
        arr.push(String::from(linenew.trim_end_matches("\n")));
    }
    Ok((number_of_results,arr))
}

fn validate_output( expected:&Vec<String>, produced : Vec<String>){

    let extra:Vec<String> = produced
        .into_iter()
        .filter_map(|x| {
            if expected.contains(&x) {
                None
            } else{
                Some(x)
            }
        })
        .collect();

    println!("  Valid output: {:?}",extra.is_empty());
    if !extra.is_empty(){
        println!("  {:?}",extra);
    }
}


//  __      __         
//  \ \    / /         
//   \ \  / /___   ___ 
//    \ \/ // _ \ / __|
//     \  /|  __/| (__ 
//      \/  \___| \___|
                    
fn find_basin_vec(width:u32,height:u32,data:&Vec<f32>) -> Vec<String>{
    let mut result: Vec<String>= Vec::new();

    for r in 1..height-1 {
        for c in 1..width-1 {
            if is_basin_vec(r,c,width,&data){
                let ans = format!("{} {}",r,c);
                // result.append(String::from(ans));
                result.push(ans);
            }
        }
    }
    result
}

fn is_basin_vec(r:u32,c:u32,w:u32,data:&Vec<f32>)-> bool{
    
    let cen:usize = (r*w+c) as usize;
    let cen_cen = data[cen] + 0.01;
    let cen_l =  data[(cen-1)];
    let cen_r =  data[(cen+1)];

    let top:usize = ((r-1)*w +c) as usize;
    let top_m =  data[(top)];
    let top_l =  data[(top-1)];
    let top_r =  data[(top+1)];

    let bot:usize = ((r+1)*w +c) as usize;
    let bot_m =  data[(bot)];
    let bot_l =  data[(bot-1)];
    let bot_r =  data[(bot+1)];

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
                          
fn find_basin_slice(width:u32,height:u32,data:&[f32]) -> Vec<String>{

  let mut answers = Vec::new();
    for r in 1..height-1 {
        for c in 1..width-1 {
            if is_basin_slice(r,c,width,data){
                answers.push(format!("{} {}",r,c));
            }
        }
    }
    answers
}

fn is_basin_slice(r:u32,c:u32,w:u32,data:&[f32])-> bool{
    
    // Note indexing directly into data[cen] is unsafe, 
    // for critical projects rather use data.get(cen) and handle the Option<f32>

    let cen:usize = (r*w+c) as usize;
    let cen_cen = data[cen] + 0.01;
    let cen_l = data[cen-1];
    let cen_r = data[cen+1];

    let top:usize = ((r-1)*w +c) as usize;
    let top_m = data[top];
    let top_l = data[top-1];
    let top_r = data[top+1];

    let bot:usize = ((r+1)*w +c) as usize;
    let bot_m = data[bot];
    let bot_l = data[bot-1];
    let bot_r = data[bot+1];

    let cen_check = cen_cen <= cen_l && cen_cen <= cen_r;
    let top_check = cen_cen <= top_l && cen_cen <= top_r && cen_cen <= top_m;
    let bot_check = cen_cen <= bot_l && cen_cen <= bot_r && cen_cen <= bot_m;

    if cen_check && top_check && bot_check {
        return true;
    }
    return false;
}
