import java.io.File;
import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;

public class DayTwo {
    public static void main(String[] args) {
        Map<String, Integer> dict = new HashMap<>();
        Map<String, String> winDict = new HashMap<>();
        Map<String, String> looseDict = new HashMap<>();
        //Map<String, String> pairDict = new HashMap<>();
        Scanner inputStream = null;
        int score = 0;

        // Part Two
        dict.put("X", 0);
        dict.put("Y", 3);
        dict.put("Z", 6);

        dict.put("A", 1);
        dict.put("B", 2);
        dict.put("C", 3);

        winDict.put("A", "C");
        winDict.put("B", "A");
        winDict.put("C", "B");
        looseDict.put("C", "A");
        looseDict.put("A", "B");
        looseDict.put("B", "C");

        /* Part One
        dict.put("X", 1);
        dict.put("Y", 2);
        dict.put("Z", 3);

        winDict.put("X", "C");
        winDict.put("Y", "A");
        winDict.put("Z", "B");

        pairDict.put("X", "A");
        pairDict.put("Y", "B");
        pairDict.put("Z", "C");
        */
        try {
            if (args.length < 1) throw new Exception("Specificare percorso file!");
            inputStream = new Scanner(new File(args[0]));
        } catch (Exception e) {
            System.out.println(e);
            System.exit(1);
        }

        while(inputStream.hasNext()) {
            String line = inputStream.nextLine();
            String my_move = line.split(" ")[1];
            String opp_move = line.split(" ")[0];

            /* Part One
            if (winDict.get(my_move).equals(opp_move)) score += 6;
            if (pairDict.get(my_move).equals(opp_move)) score += 3;
             */

            /* Part Two */
            score += dict.get((my_move));
            switch (dict.get(my_move)) {
                case 0 -> score += dict.get(winDict.get(opp_move));
                case 3 -> score += dict.get(opp_move);
                case 6 -> score += dict.get(looseDict.get(opp_move));
            }
        }

        System.out.println(score);
    }
}
