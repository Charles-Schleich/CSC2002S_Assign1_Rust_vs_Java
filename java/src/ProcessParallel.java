
import java.util.ArrayList;
import java.util.concurrent.RecursiveTask;

public class ProcessParallel extends RecursiveTask<ArrayList<String>>  {
	  int r_lo; // arguments
	  int r_hi;

	  int width;
	  int height;

	  double[] arr;
	  static final int SEQUENTIAL_CUTOFF=200;

	  int ans = 0; // result
	    
	  ProcessParallel(double[] a, int width, int height, int r_lo, int r_hi) { 
        arr=a;
        this.r_lo=r_lo;
        this.r_hi=r_hi;

	    this.width = width;
	    this.height = height;
	  }

	  protected ArrayList<String> compute(){
        //   System.out.println("r_hi: "+r_hi+"   r_lo: " +r_lo);
          ArrayList<String> output = new ArrayList<String>();


		  if((r_hi-r_lo) < SEQUENTIAL_CUTOFF) {
              ArrayList<String> basins = find_basin_list(arr, height, width, r_lo, r_hi);
		      return basins;
		  }
		  else {
              int r_half = (r_hi+r_lo)/2;
			  ProcessParallel left  = new ProcessParallel(arr, width, height, r_lo, r_half );
			  ProcessParallel right = new ProcessParallel(arr, width, height, r_half, r_hi);
			  
			  // order of next 4 lines
			  // essential – why?
			  left.fork();
			  ArrayList<String> rightAns = right.compute();
			  ArrayList<String> leftAns  = left.join();
            
              rightAns.addAll(leftAns);
			  return rightAns;
		  }
	 }






    static ArrayList<String> find_basin_list(double [] arr, int height, int width, int r_lo, int r_hi){
        ArrayList<String> basin_list = new ArrayList<String>();

        // System.out.println("r_hi: "+r_hi+"   r_lo: " +r_lo);
        if (r_lo==0){
            r_lo = 1;
        }
        if (r_hi==width){
            r_hi = r_hi -1;
        }



        for (int r = r_lo; r < r_hi; r++){
            for (int c = 1; c < width-1; c++){

                // These ones are ones that are missing from the outfile 
                // 1014 123
                // 1022 124
                // 859 467
                // 649 507
                // 752 308
                // 589 262
                // if (r ==1014 && c== 123){
                //     is_basin_list(arr,r,c,width,true);
                // }


                if ( is_basin_list(arr,r,c,width,false) ){
                    String basin =  r+" "+c;
                    basin_list.add(basin);

                }

            }
        }
        return basin_list;
    }



    static boolean is_basin_list(double [] arr, int r, int c, int w, boolean print ){
        // w*r + c
        // System.out.println("Error: ");
        
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

        boolean cen_check = cen < cen_l && cen < cen_r;
        boolean top_check = cen < top_l && cen < top_r && cen < top_m;
        boolean bot_check = cen < bot_l && cen < bot_r && cen < bot_m;
        
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


