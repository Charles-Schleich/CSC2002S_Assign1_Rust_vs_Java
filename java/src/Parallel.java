import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList; 
import java.util.Scanner;
import java.util.concurrent.ForkJoinPool;


public class Parallel{
    
    static long startTime = 0;
	
	private static void tick(){
		startTime = System.currentTimeMillis();
	}

	private static float tock(){
		return (System.currentTimeMillis() - startTime) / 1000.0f; 
	}

	static final ForkJoinPool fjPool = new ForkJoinPool();
	
	static ArrayList<String> find(double[] arr, int width, int height){
        return fjPool.invoke(new ProcessParallel(arr, width, height, 0, width));
	}


//   __  __         _        
//  |  \/  |       (_)       
//  | \  / |  __ _  _  _ __  
//  | |\/| | / _` || || '_ \ 
//  | |  | || (_| || || | | |
//  |_|  |_| \__,_||_||_| |_|

    public static void main(String[] Args){
        System.out.println("Parallel Start");

        Tuple t = read_in_data_list(Args[0]);
        int width      = t.width;
        int height     = t.height;
        double [] list = t.list;

	    // tick();
    	// int sumArr = sum(arr);
		// time = tock();
		// System.out.println("Second run took "+ time +" seconds");


        ArrayList<String> answer;
        double time ;
        double total =0;        
        int runs = 200;
        for(int run = 0; run<runs; run++ ){
            tick();
            answer = find( t.list, t.height,t.width);
            time = tock();
            total=total+time;
            System.out.println("Run took "+ time +" seconds");
        }

        System.out.println("Average: "+ total/runs);
        System.out.println("Parallel End");
    }




    static Tuple read_in_data_list(String input){
        String gridsize = new String("");
        String values   = new String("");
        String input_file = String.join("","./Data/",input);

        try (FileReader reader = new FileReader(input_file);
            BufferedReader br = new BufferedReader(reader) ){
            gridsize = br.readLine();
            values = br.readLine();
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

        Tuple t = new Tuple(width,height,list);

        System.out.println("Running For file " + input_file);
        return t;
    }

    // Find Basin List 
    // static void find_basin_list(double [] arr, int height, int width){

    //     for (int r = 1; r < height-1; r++){
    //         for (int c = 1; c < width-1; c++){
                
    //             if ( is_basin_list(arr,r,c,width,true) ){
    //                 System.out.println(r+" "+c);
    //             }

    //         }
    //     }
    // }

    // static boolean is_basin_list(double [] arr, int r, int c, int w, boolean print ){
    //     // w*r + c

    //     double cen   = arr[(r*w)+c] + 0.01;
    //     double cen_l = arr[(r*w)+c-1];
    //     double cen_r = arr[(r*w)+c+1];

    //     int top = (r-1)*w;
    //     double top_m = arr[top + c];
    //     double top_l = arr[top + c-1];
    //     double top_r = arr[top + c+1];

    //     int bot = (r+1)*w;
    //     double bot_m = arr[bot + c];
    //     double bot_l = arr[bot + c-1];
    //     double bot_r = arr[bot + c+1];

    //     boolean cen_check = cen <= cen_l && cen <= cen_r;
    //     boolean top_check = cen <= top_l && cen <= top_r && cen <= top_m;
    //     boolean bot_check = cen <= bot_l && cen <= bot_r && cen <= bot_m;
        
    //     if (print){
    //         System.out.println(top_l+" " + top_m + " " + top_r);
    //         System.out.println(cen_l+" " + cen + " " + cen_r);
    //         System.out.println(bot_l+" " + bot_m + " " + bot_r);
    //         System.out.println("cen_check: " + cen_check + " "  + (cen < cen_l) + " " + (cen < cen_r));
    //         System.out.println("cen_check: "+ cen +" "+ (cen_l) + " " + (cen_r)); 
    //     }

    //     if (cen_check && top_check && bot_check) {
    //         return true;
    //     } 
    //     return false;
    // }

}