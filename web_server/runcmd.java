import java.io.IOException;
import java.io.InputStreamReader;
import java.io.LineNumberReader;
import java.lang.Runtime;

public class runcmd {
    public static void main(String[] args) {
        String result = "";
        try {
            String jsonobj = new String("{\"boolean\":[1,2,3,4,5],\"string\":\"1\",\"list\":1,\"int\":2}");
            Process process = Runtime.getRuntime().exec("C:/Users/YMZ/anaconda3/envs/tf2/python.exe c:/Users/YMZ/Desktop/test____/client.py " + jsonobj  );
            InputStreamReader ir = new InputStreamReader(process.getInputStream());
            LineNumberReader input = new LineNumberReader(ir);
            result = input.readLine();
            while ((result = input.readLine()) != null) { 
                System.out.println(result); 
            }
            input.close();
            ir.close();
//            process.waitFor();
        } catch (IOException e) {
        }
    }
}