Dictionary<string,int> opcodes = new Dictionary<string, int>() {
    {"addx",2},
    {"noop",1},
};

bool checkStrength(int cycle) {
    return (cycle == 20 ||
            cycle == 60 ||
            cycle == 100 ||
            cycle == 140 ||
            cycle == 180 ||
            cycle == 220);
}

int getStrength(int cycle, int reg) {
    return cycle*reg;
}


string[] lines = File.ReadAllLines("input.txt");
List<string> CRT = new List<string>();
int X = 1;
int cycle = 0;
int lock_ck = 0;
int sum = 0;

foreach (string line in lines)
{
    string op = line.Split()[0];
    int val = (line.Split().Length > 1) ? Int32.Parse(line.Split()[1]) : 0;
    lock_ck = opcodes[op];

    do {
        if(checkStrength(cycle)) {
            sum += getStrength(cycle, X);
        }
        if(cycle%40 == X-1 || cycle%40 == X || cycle%40 == X+1) {
            CRT.Add("#");
        } else {
            CRT.Add(".");
        }
        Console.WriteLine(cycle %40 + " " + X + " " + CRT.Last());

    ++cycle;
    } while(--lock_ck > 0);

    X += val;

}

Console.WriteLine(cycle);
Console.WriteLine(sum+"\n");

for(int i = 0; i < 240; i++) {
    if(i%40==0) Console.WriteLine();
    Console.Write(CRT[i]);
}
