import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList; 
import java.util.Scanner;

public class Sequential{

    static long startTime = 0;
	
	private static void tick(){
		startTime = System.currentTimeMillis();
	}

	private static float tock(){
		return (System.currentTimeMillis() - startTime) / 1000.0f; 
	}

//   __  __         _        
//  |  \/  |       (_)       
//  | \  / |  __ _  _  _ __  
//  | |\/| | / _` || || '_ \ 
//  | |  | || (_| || || | | |
//  |_|  |_| \__,_||_||_| |_|

    public static void main(String[] Args) {
        System.out.println("Sequential Start");
        System.out.println("Input file "+ Args[0]);
        // ARRAY METHOD
        // double [][] arr = read_in_data(Args[0]);
        // double [] f_line = arr[0];
        // int width  = f_line.length;
        // int height = arr.length;
        // tick();
        //  find_basin(arr, height, width);
		// float time = tock();
		// System.out.println("Run took "+ time +" seconds");
        // LIST METHOD 

        Tuple t = read_in_data_list(Args[0]);

        ArrayList<String> answer;
        double time ;
        double total =0;
        int runs = 200; 
        for(int run = 0; run<runs ; run++ ){
            tick();
            answer = find_basin_list ( t.list, t.height,t.width);
            time = tock();
            total=total+time;
            System.out.println("Run took "+ time +" seconds");
        }

        System.out.println("Average: "+ total/runs);

        // System.out.println("Output is "+ answer);
        System.out.println("Sequential end");
    }

                       
//      /\                
//     /  \    _ __  _ __ 
//    / /\ \  | '__|| '__|
//   / ____ \ | |   | |   
//  /_/    \_\|_|   |_|   

    static double[][] read_in_data(String input){
        String gridsize = new String("");
        String values   = new String("");
        String input_file = String.join("","./Data/",input);
        
        try (FileReader reader = new FileReader(input_file);
            BufferedReader br = new BufferedReader(reader) ){
            gridsize = br.readLine();
            values = br.readLine();
            br.close();
            reader.close();
        } catch(IOException e){
            System.err.format("IOException: %s%n",e);
            System.err.format("Exiting");
            System.exit(1);
        }
    
        Scanner sc = new Scanner(gridsize); 
        int width  = sc.nextInt();
        int height = sc.nextInt();
        
        double [] list = new double[height*width];
        sc = new Scanner(values);
        int index = 0 ;
        while (sc.hasNextDouble()) {
            list[index]= sc.nextDouble();
            index=index+1;
        }
        sc.close();

        double [][] arr = new double[width][height];
        index = 0;
        for (int r = 0; r < height; r++){
            for (int c = 0; c < width; c++){
                arr[r][c] = list[index] ;
                index = index +1;
            }
        }

        return arr;
    }

    static void find_basin(double [][] arr, int height, int width){


        for (int r = 1; r < height-1; r++){
            for (int c = 1; c < width-1; c++){
                
                // if (r == 154 && c==212){
                //     is_basin(arr,r,c,true);
                // }

                if ( is_basin(arr,r,c,false) ){
                    System.out.println(r+" "+c);
                }

            }
        }

    }

    static boolean is_basin(double [][] arr, int r, int c, boolean print ){
        double cen   = arr[r][c] + 0.01;
        double cen_l = arr[r][c-1];
        double cen_r = arr[r][c+1];

        double top_m = arr[r-1][c];
        double top_l = arr[r-1][c-1];
        double top_r = arr[r-1][c+1];

        double bot_m = arr[r+1][c];
        double bot_l = arr[r+1][c-1];
        double bot_r = arr[r+1][c+1];

        boolean cen_check = cen < cen_l && cen < cen_r;
        boolean top_check = cen < top_l && cen < top_r && cen < top_m;
        boolean bot_check = cen < bot_l && cen < bot_r && cen < bot_m;
        
        if (print){
            System.out.println(top_l+" " + top_m + " " + top_r);
            System.out.println(cen_l+" " + cen + " " + cen_r);
            System.out.println(bot_l+" " + bot_m + " " + bot_r);
        
            System.out.println("cen_check:  " + cen_check + " "  + (cen < cen_l) + " " + (cen < cen_r));
            System.out.println("cen_check:  "+ cen +" "+ (cen_l) + " " + (cen_r));
            
        }

        if (cen_check && top_check && bot_check) {
            return true;
        } 
        return false;
    }

//   _       _       _   
//  | |     (_)     | |  
//  | |      _  ___ | |_ 
//  | |     | |/ __|| __|
//  | |____ | |\__ \| |_ 
//  |______||_||___/ \__|

    static Tuple read_in_data_list(String input){
        String gridsize = new String("");
        String values   = new String("");
        String input_file = String.join("","./Data/",input);
        
        try (FileReader reader = new FileReader(input_file);
            BufferedReader br = new BufferedReader(reader) ){
            gridsize = br.readLine();
            values = br.readLine();
            br.close();
            reader.close();
        } catch(IOException e){
            System.err.format("IOException: %s%n",e);
            System.err.format("Exiting");
            System.exit(1);
        }

        Scanner sc = new Scanner(gridsize); 
        int width  = sc.nextInt();
        int height = sc.nextInt();
        
        double [] list = new double[height*width];
        sc = new Scanner(values);
        int index = 0 ;
        while (sc.hasNextDouble()) {
            list[index]= sc.nextDouble();
            index=index+1;
        }
        sc.close();

        Tuple t = new Tuple(width,height,list);

        return t;
    }

    // Find Basin List 
    static ArrayList<String> find_basin_list(double [] arr, int height, int width){

        ArrayList<String> result = new ArrayList<String>();

        String output = "\n";
        for (int r = 1; r < height-1; r++){
            for (int c = 1; c < width-1; c++){
                
                if ( is_basin_list(arr,r,c,width,false) ){
                    // output = output + r+" "+c + "\n";
                    result.add(r+" "+c);
                }

            }
        }
        return result;
    }

    static boolean is_basin_list(double [] arr, int r, int c, int w, boolean print ){
        // w*r + c

        double cen   = arr[(r*w)+c] + 0.01;
        double cen_l = arr[(r*w)+c-1];
        double cen_r = arr[(r*w)+c+1];

        int top = (r-1)*w;
        double top_m = arr[top + c];
        double top_l = arr[top + c-1];
        double top_r = arr[top + c+1];

        int bot = (r+1)*w;
        double bot_m = arr[bot + c];
        double bot_l = arr[bot + c-1];
        double bot_r = arr[bot + c+1];

        boolean cen_check = cen <= cen_l && cen <= cen_r;
        boolean top_check = cen <= top_l && cen <= top_r && cen <= top_m;
        boolean bot_check = cen <= bot_l && cen <= bot_r && cen <= bot_m;
        
        if (print){
            System.out.println(top_l+" " + top_m + " " + top_r);
            System.out.println(cen_l+" " + cen + " " + cen_r);
            System.out.println(bot_l+" " + bot_m + " " + bot_r);
            System.out.println("cen_check: " + cen_check + " "  + (cen < cen_l) + " " + (cen < cen_r));
            System.out.println("cen_check: "+ cen +" "+ (cen_l) + " " + (cen_r)); 
        }

        if (cen_check && top_check && bot_check) {
            return true;
        } 
        return false;
    }

}


// [1.00 0.90 0.95 0.80 1.00 0.95 0.90 0.80 0.85 0.60 0.80 0.75 1.00 1.00 1.00 1.00 ]

// 4 * 4  ==  w * h
// [ 1.00 0.90 0.95 0.80 
//   1.00 0.95 0.90 0.80 
//   0.85 0.60 0.80 0.75 
//   1.00 1.00 1.00 1.00 ]
// r=1
// c=1

// arr[r][c]      ->  list[(w*r) + col ]
// arr[1][1] = 95 ->  list[(4+1) +1 ]   -> list[(2)*(2)-1]  -> list[3]
// arr[0][1] = 90 ->  list[(4*(0)) +1]  -> list[11]


// arr[r][c]    ->  list[(w*r) + col ]  ->  list[]
// arr[2][1] =  ->  list[(4*2) + 1 ]    ->  list[8+1]
// arr[3][3] =  ->  list[(4*3) + 3 ]    ->  list[12+3]


// arr[3][3] =  ->  list[(3+1)*(3+1)-1]    ->  list[15]


//  if index is < w || 