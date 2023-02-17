import org.jetbrains.annotations.NotNull;

import java.io.File;
import java.util.Scanner;

public class DayFour {
    public static void main(String[] args) {
        Scanner inputStream = null;
        int countFullyContained = 0;
        int countOverlaps = 0;

        try {
            if (args.length < 1) { throw new Exception("No input file provided!"); }
            inputStream = new Scanner(new File(args[0]));
        } catch (Exception e) {
            System.out.println(e.getMessage());
            System.exit(-1);
        }

        while(inputStream.hasNext()) {
            String line = inputStream.nextLine();
            String[] ranges = line.split(",");

            countFullyContained += isFullyContained(ranges[0].split("-"), ranges[1].split("-")) ? 1 : 0;
            countOverlaps += overlaps(ranges[0].split("-"), ranges[1].split("-")) ? 1 : 0;

        }
        System.out.println(countFullyContained);
        System.out.println(countOverlaps);

    }

    static boolean isFullyContained(String @NotNull [] s1, String @NotNull [] s2) {
        int i1 = Integer.parseInt(s1[0]);
        int i2 = Integer.parseInt(s1[1]);
        int f1 = Integer.parseInt(s2[0]);
        int f2 = Integer.parseInt(s2[1]);
        return (i1 >= f1 && i2 <= f2) || (f1 >= i1 && f2 <= i2);
    }

    static boolean overlaps(String @NotNull [] s1, String @NotNull [] s2) {
        int i1 = Integer.parseInt(s1[0]);
        int f1 = Integer.parseInt(s1[1]);
        int i2 = Integer.parseInt(s2[0]);
        int f2 = Integer.parseInt(s2[1]);

        return i2 <= f1 && f2 >= i1;
    }
}
