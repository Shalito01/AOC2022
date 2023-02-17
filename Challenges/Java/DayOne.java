import java.util.Scanner;
import java.io.File;
import java.util.List;
import java.util.LinkedList;

public class DayOne {
  public static void main(String[] args) {
    Scanner inputStream = null;
    List<Elf> elves = new LinkedList<>();
    elves.add(new Elf());

    try{
      if(args.length != 1) throw new Exception("Specificare nome file");
      inputStream = new Scanner(new File(args[0]));
    } catch (Exception e){
      System.out.println(e.toString());
      System.exit(1);
    }

    while(inputStream.hasNext()) {
      String line = inputStream.nextLine();
      if(line.isEmpty()) {
        elves.add(new Elf());
      } else {
        int cal = Integer.parseInt(line);
        elves.get(elves.size() - 1).addCalories(cal);
      }
    }

    int max = 0;
    int max2 = 0;
    int max3 = 0;
    for(Elf x: elves) {
      if(x.getCalories() > max) {
        max3 = max2;
        max2 = max;
        max = x.getCalories();
      } else if (x.getCalories() > max2) {
        max3 = max2;
        max2 = x.getCalories();
      } 
    }
    System.out.println(max + max2 + max3);
  }
}

class Elf {
  private int calories;

  public Elf() {
    calories = 0;
  }

  public void addCalories(int cal) {
    this.calories += cal;
  }

  public int getCalories() {
    return calories;
  }
}