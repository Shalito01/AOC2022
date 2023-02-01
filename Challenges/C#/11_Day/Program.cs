string[] lines = File.ReadAllLines("input.txt");
int monkeyCount = 0;
List<Monkey> monkeys = new List<Monkey>();
for(int i = 0; i < lines.Count(); i++) {
    if(lines[i].Equals(string.Format("Monkey {0}:", monkeyCount))) {
        int monkeyNum = i;
        int[] items = Array.ConvertAll(lines[++i].Split(":")[1].Split(","), s => int.Parse(s));
        string op = lines[++i].Split("=")[1];
        int test = int.Parse(lines[++i].Split(" ").Last());
        int goTrue = int.Parse(lines[++i].Split(" ").Last());
        int goFalse = int.Parse(lines[++i].Split(" ").Last());

        monkeys.Add(new Monkey(items, op, test, goTrue, goFalse));
        ++monkeyCount;
    }
}

for(int i = 0; i < 20; i++) {
    for(int j = 0; j < monkeys.Count(); ++j) {
        monkeys[j].inspected += monkeys[j].items.Count();
        for(int k = 0; k < monkeys[j].items.Count(); ++k) {
            monkeys[j].update(monkeys[j].items[k]);
            monkeys[monkeys[j].testItem(monkeys[j].items[k])].addItem(monkeys[j].items[k]);
            monkeys[j].items.Remove(monkeys[j].items[k]);
        }
    }
}
int max1 = 0, max2 = 0;
foreach(Monkey m in monkeys) {
    if(m.inspected > max1) {
        max2 = max1;
        max1 = m.inspected;
    }
}

Console.WriteLine(max1 + " " + max2 + " " + max1*max2);

public class Monkey {
	public List<int> items;
	public string operation;
	public int test;
	public int goIfTrue;
	public int goIfFalse;
    public int inspected;

    public Monkey(int[] item, string operation, int test, int goIfTrue, int goIfFalse) {
        this.items = item.ToList();
        this.operation = operation;
        this.test = test;
        this.goIfTrue = goIfTrue;
        this.goIfFalse = goIfFalse;
        this.inspected = 0;
    }

    public void addItem(int item) {
        this.items.Add(item);
    }

    public void update(int index) {
        string op = (operation.Contains("+")) ? "+" : "*";
        string addend = this.operation.Split(op)[1];
        int x = index;
        int y;
        if(!int.TryParse(addend,out y)) y = x;
        Console.WriteLine("old: " + index + " : " + y);
        switch(op) {
            case "+": x += y;
                      break;
            case "*": x *= y;
                      break;
        }
        this.items[this.items.IndexOf(index)] = x;


    }

    public int testItem(int index) {
        int x = index;
        if (x % test == 0) {
            return goIfTrue;
        } else {
            return goIfFalse;
        }
    }
}

