using System.Net.Http.Headers;

string[] lines = File.ReadAllLines("input.txt");
int monkeyCount = 0;
List<Monkey> monkeys = new List<Monkey>();
for(int i = 0; i < lines.Count(); i++) {
    if(lines[i].Equals(string.Format("Monkey {0}:", monkeyCount))) {
        long[] items = Array.ConvertAll(lines[++i].Split(":")[1].Split(","), s => long.Parse(s));
        string op = lines[++i].Split("=")[1];
        int test = int.Parse(lines[++i].Split(" ").Last());
        int goTrue = int.Parse(lines[++i].Split(" ").Last());
        int goFalse = int.Parse(lines[++i].Split(" ").Last());

        monkeys.Add(new Monkey(monkeyCount, items, op, test, goTrue, goFalse));
        ++monkeyCount;
    }
}



for(int i = 0; i < 20; i++) {
    foreach(Monkey m in monkeys)
    {
        while(m.items.TryDequeue(out long oldVal))
        {
            m.inspected++;
            long newVal = m.update(oldVal);
            newVal /= 3;
            monkeys[m.testItem(newVal)].addItem(newVal);
        }

    }

}
int max1 = 0, max2 = 0;
foreach(Monkey m in monkeys) {
    Console.WriteLine(m.inspected);
    if(m.inspected > max1) {
        max2 = max1;
        max1 = m.inspected;
    } else if (m.inspected > max2) {
        max2 = m.inspected;
    }
}

Console.WriteLine(max1 + " " + max2 + " " + max1*max2);

public class Monkey {
    public int id;
	public Queue<long> items;
	public string operation;
	public int test;
	public int trueDest;
	public int falseDest;
    public int inspected;

    public Monkey(int id, long[] item, string operation, int test, int trueDest, int falseDest) {
        this.id = id;
        items = new Queue<long>(item);
        this.operation = operation;
        this.test = test;
        this.trueDest = trueDest;
        this.falseDest = falseDest;
    }

    public void addItem(long item) {
        this.items.Enqueue(item);
    }

    public long update(long oldVal) {
        string op = (operation.Contains("+")) ? "+" : "*";
        string addend = operation.Split(op).Last();
        long newVal = oldVal;
        long y = (long.TryParse(addend, out long valore)) ? valore : oldVal;
        Console.Write("old: " + newVal + " "+op+" " + y);
        switch(op) {
            case "+": newVal += y;
                      break;
            case "*": newVal *= y;
                      break;
        }
        Console.WriteLine(" = " + newVal);
        return newVal;
    }

    public void prinMonk() {
        foreach(var item in items) {
            Console.Write(item+", ");
        }
        Console.WriteLine();
        Console.WriteLine(operation);
        Console.WriteLine("Divisible by {0}",test);
        Console.WriteLine("If True: {0} If False: {1}", trueDest,falseDest);
    }


    public int testItem(long testVal) {
        if (testVal % test == 0)
        {
            return trueDest;
        }
        else
        {
            return falseDest;
        }
    }
}

