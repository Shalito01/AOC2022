import java.util.*;
import java.io.File;

public class DayThree {
    public static void main(String[] args) {
        Scanner inputStream = null;
        int sumPriorities = 0;

        try {
            if (args.length != 1) throw new Exception("Specify file path");
            inputStream = new Scanner(new File(args[0]));
        } catch (Exception e) {
            System.out.println(e.getMessage());
            System.exit(-1);
        }

        while(inputStream.hasNext()) {
            String line = inputStream.nextLine();
            /* Part One
            String[] compartments = { line.substring(0, line.length() / 2), line.substring(line.length() / 2) };
            for (char c : compartments[0].toCharArray()) {
                if (compartments[1].indexOf(c) != -1) {
                    sumPriorities += getPriority(c);
                    break;
                }
            }
             */
            ArrayList<String> compartments = new ArrayList<>(3);
            compartments.add(line);
            line = inputStream.nextLine();
            compartments.add(line);
            line = inputStream.nextLine();
            compartments.add(line);

            for (char c : compartments.get(0).toCharArray()) {
                if (compartments.get(1).indexOf(c) != -1 && compartments.get(2).indexOf(c) != -1) {
                    sumPriorities += getPriority(c);
                    break;
                }
            }
        }

        System.out.println(sumPriorities);
    }

    static int getPriority(char letter) {
        int ret = Character.isUpperCase(letter) ? 26 : 0;
        ret += ((Character.toLowerCase(letter) - 'a') + 1);

        System.out.println(letter + " : " + ret);
        return ret;
    }
}