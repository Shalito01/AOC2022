import java.io.File;
import java.util.*;

public class DaySeven {
    public static void main(String[] args) {

        Scanner fileInput;

        try {
            if (args.length != 2 || !args[0].equals("--file")) throw new Exception("Please specify:\n\t--file FILEPATH");

            fileInput = new Scanner(new File(args[1]));

            Stack<String> pwd = new Stack<>();
            Map<String, Integer> dirs = new HashMap<>();

            dirs.put("/", 0);
            pwd.push("/");

            while (fileInput.hasNextLine()) {
                String[] buffer = fileInput.nextLine().split(" ");

                if(buffer[0].equals("$") && buffer[1].equals("cd")) {
                    switch (buffer[2]) {
                        case ".." -> pwd.pop();
                        case "/" -> {
                            while (!pwd.peek().equals("/")) {
                                pwd.pop();
                            }
                        }
                        default -> {
                            dirs.putIfAbsent(buffer[2], 0);
                            pwd.push(buffer[2]);
                        }
                    }
                } else if (!buffer[0].equals("$") && !buffer[0].equals("dir")) {
                    for (String x: pwd) {
                        dirs.replace(x, dirs.get(x) + Integer.parseInt(buffer[0]));
                    }
                }


            }

            Integer sum = 0;
            for(Integer i: dirs.values()) {
                if (i <= 100000) {
                    sum += i;
                }
            }
            for(String key: dirs.keySet()) {
                System.out.println(key + ": " + dirs.get(key));
            }

            System.out.println(sum);
            fileInput.close();

        } catch (Exception e) {
            System.out.println(e.getMessage());
            System.exit(1);
        }

    }


}


