import java.io.File;
import java.util.Scanner;

public class DayFive {
    public static void main(String[] args) {
        Scanner inputStream = null;

        try {
            if (args.length < 1) throw new Exception("Specify filePath");
            inputStream = new Scanner(new File("dayfive.txt"));
        } catch (Exception e) {
            System.out.println(e.getMessage());
            System.exit(-1);
        }

    }
}
