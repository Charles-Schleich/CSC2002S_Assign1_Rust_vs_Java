import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList; 
import java.util.Scanner;
import java.util.concurrent.ForkJoinPool;
import java.util.Iterator; 

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
        return fjPool.invoke(new ProcessParallel(arr, height, width, 0, width));
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

        ArrayList<String> answer= new ArrayList<String>();

        double time ;
        double total =0;        
        int runs = 1;
        for(int run = 0; run<runs; run++ ){
            tick();
            answer = find( t.list, t.height,t.width);
            time = tock();
            total=total+time;
            System.out.println("Run took "+ time +" seconds");
        }
        System.out.println("Average: "+ total/runs);
        // System.out.println("Answer: "+ answer);
        ArrayList<String> actual = read_in_answer_list(Args[1]);

        compare(actual,answer); 
        System.out.println("Parallel End");
    }


    public static void compare( ArrayList<String> actualAnswer, ArrayList<String> myAnswer ){
        System.out.println("Compare ");
        Iterator<String> iter  = myAnswer.iterator(); 

        System.out.println("    actualAnswer " + actualAnswer.size() );
        System.out.println("    myAnswer " + myAnswer.size() );

        while (iter.hasNext()) { 
            String elem = iter.next();
            if (!actualAnswer.contains(elem)){
                System.out.println(elem); 
            }
        } 
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



    static ArrayList<String> read_in_answer_list(String input){
        String gridsize = new String("");
        String values   = new String("");
        String input_file = String.join("","./Data/",input);
        ArrayList<String> list = new ArrayList<String>();
        int index = 0;

        try (FileReader reader = new FileReader(input_file);

            BufferedReader br = new BufferedReader(reader) ){
            gridsize = br.readLine();
            Scanner sc = new Scanner(gridsize); 
            int numberOfAnswers  = sc.nextInt();


            String str = br.readLine();
            while (str!=null){
                list.add(str); 
                index = index +1;
                str = br.readLine();
            }
            br.close();
            sc.close();
        } catch(IOException e){
            System.err.format("IOException: %s%n",e);
            System.err.format("Exiting");
            System.exit(1);
        }
        
        // for(int i =0; i <list.size(); i ++){
        //     System.out.println(list.get(i));
        // }
   
        return list;
    }
  
}