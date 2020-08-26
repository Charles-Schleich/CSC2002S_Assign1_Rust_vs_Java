#![allow(warnings)]
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Duration, Instant};

fn main() {
    let file = String::from("./Data/large_in.txt");
    let output = String::from("./Data/large_out.txt");

    // Read in data
    let (width,height,data): (u32,u32,Vec<Vec<f32>>) = read_in_input(&file).unwrap();

    let data2 = data.clone();
    let vec_slice:Vec<&[f32]> = data2
        .iter()
        .map(|x| &x[..])
        .collect();

    let slice = &vec_slice[..];

    let mut total_vec:f32 = 0.0; 
    let runs = 100;
    println!("Start Vec : runs:{} ", runs );

    let mut result_vec: Vec<String> = Vec::new();
    for _ in 0..runs {
        let now = Instant::now();
        result_vec = find_basin_vec(width,height,&data);
        let new_now = Instant::now();
        let time = new_now.duration_since(now).as_secs_f32();
        total_vec = total_vec + time;
    }


    let mut total_slice:f32 = 0.0; 
    let mut result_slice: Vec<String> = Vec::new();
    for _ in 0..runs {
        let now = Instant::now();
        result_slice = find_basin_slice(width,height,slice);
        let new_now = Instant::now();
        let time = new_now.duration_since(now).as_secs_f32();
        total_slice= total_slice+time;
    }



    println!("Results\n--------------", );

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

fn read_in_input(file:&String) -> std::io::Result<(u32,u32,Vec<Vec<f32>>)> {

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

    let mut data: Vec<Vec<f32>> = Vec::new();
    let mut row: Vec<f32> = Vec::new();
    let mut count = 1;
    for i in linenew.split_whitespace().into_iter() {

        let num = parse_f32(i);
        row.push(num);
        
        if count == width{
            data.push(row);
            row = Vec::new();
            count = 1;
        } else {
            count = count + 1;
        }
    }

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
}


//  __      __         
//  \ \    / /         
//   \ \  / /___   ___ 
//    \ \/ // _ \ / __|
//     \  /|  __/| (__ 
//      \/  \___| \___|
                    
fn find_basin_vec(width:u32,height:u32,data:&Vec<Vec<f32>>) -> Vec<String>{
    let mut answers = Vec::new();

    for r in 1..width-1 {
        for c in 1..height-1{
            if is_basin_vec(r as usize,c as usize,width as usize,data){
                answers.push(format!("{} {}",r,c));
            }
        }
    }
    // println!("Answers Len {}", answers.len());
    // println!("Answers  {:?}", answers);
    answers
}

fn is_basin_vec(r:usize, c:usize,w:usize,data:&Vec<Vec<f32>>)-> bool{
    //  ["177 273", "195 343", "298 342"]
    
 
    let top_l = *data.get(r-1).unwrap()
                           .get(c-1).unwrap();
    let top_m = *data.get(r-1).unwrap()
                           .get(c).unwrap();
    let top_r = *data.get(r-1).unwrap()
                           .get(c+1).unwrap();

    let cen_l = *data.get(r).unwrap()
                          .get(c-1).unwrap();
    let cen_cen = *data.get(r).unwrap()
                            .get(c).unwrap() + 0.01 ;
    let cen_r = *data.get(r).unwrap()
                          .get(c+1).unwrap();


    let bot_l = *data.get(r+1).unwrap()
                           .get(c-1).unwrap();
    let bot_m = *data.get(r+1).unwrap()
                           .get(c).unwrap();
    let bot_r = *data.get(r+1).unwrap()
                           .get(c+1).unwrap();


    let top_check = (cen_cen <= top_l) && (cen_cen <= top_r) && (cen_cen <= top_m);
    let cen_check = (cen_cen <= cen_l) && (cen_cen <= cen_r);
    let bot_check = (cen_cen <= bot_l) && (cen_cen <= bot_r) && (cen_cen <= bot_m);

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
                          
fn find_basin_slice(width:u32,height:u32,data:&[&[f32]]) -> Vec<String>{
  let mut answers = Vec::new();

    for r in 1..width-1 {
        for c in 1..height-1{
            if is_basin_slice(r as usize,c as usize,width as usize,data){
                answers.push(format!("{} {}",r,c));
            }
        }
    }
    answers
}

fn is_basin_slice(r:usize, c:usize,w:usize,data:&[&[f32]])-> bool{
    
 
    let top_l = *data.get(r-1).unwrap()
                           .get(c-1).unwrap();
    let top_m = *data.get(r-1).unwrap()
                           .get(c).unwrap();
    let top_r = *data.get(r-1).unwrap()
                           .get(c+1).unwrap();

    let cen_l = *data.get(r).unwrap()
                          .get(c-1).unwrap();
    let cen_cen = *data.get(r).unwrap()
                            .get(c).unwrap() + 0.01 ;
    let cen_r = *data.get(r).unwrap()
                          .get(c+1).unwrap();


    let bot_l = *data.get(r+1).unwrap()
                           .get(c-1).unwrap();
    let bot_m = *data.get(r+1).unwrap()
                           .get(c).unwrap();
    let bot_r = *data.get(r+1).unwrap()
                           .get(c+1).unwrap();


    let top_check = (cen_cen <= top_l) && (cen_cen <= top_r) && (cen_cen <= top_m);
    let cen_check = (cen_cen <= cen_l) && (cen_cen <= cen_r);
    let bot_check = (cen_cen <= bot_l) && (cen_cen <= bot_r) && (cen_cen <= bot_m);

    if cen_check && top_check && bot_check {
        return true;
    }
    return false;

}
