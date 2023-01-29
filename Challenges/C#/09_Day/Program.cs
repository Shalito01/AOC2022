
Dictionary<string, int> direction = new Dictionary<string,int>
{
    {"U",1},
    {"D",-1},
    {"L",-1},
    {"R",1},
};

string[] lines = File.ReadAllLines("input.txt");


(int,int)[] Rope = new (int,int)[10];

for(int i = 0; i < 10; i++) {
    Rope[i] = (0,0);
}


//int xH = 4, yH = 0, xT = 4, yT = 0;
//int width = 6, height = 5;
HashSet<(int,int)> visited = new HashSet<(int,int)>();
visited.Add(Rope[0]);

foreach(string line in lines) {
    string cmd = line.Split()[0];
    int count = Int32.Parse(line.Split()[1]);

    for(int i = 0; i < count; i++) {
        // Move Head

        if(cmd == "U" || cmd == "D") {
            Rope[0].Item1 += direction[cmd];
        } else {
            Rope[0].Item2 += direction[cmd];
        }

        for(int k = 1; k < 10; k++) {

            if(heurisic(Rope[k-1],Rope[k]) > 1) {
                // Move Tail
                Rope[k] = moveTail(Rope[k-1],Rope[k]);
            }
        }
        visited.Add(Rope[9]);
    }
}
Console.WriteLine(visited.Count());


//Console.WriteLine("Width = {0} & Height = {1}", width, height);

int heurisic((int,int) H, (int,int) T) {
    return (int) Math.Sqrt(Math.Pow(H.Item1-T.Item1,2) + Math.Pow(H.Item2-T.Item2,2));
}
(int,int) moveTail((int,int) H, (int,int) T) {
    int x = T.Item1, y = T.Item2;
    if(H.Item1 > T.Item1) x++;
    if(H.Item1 < T.Item1) x--;

    if(H.Item2 > T.Item2) y++;
    if(H.Item2 < T.Item2) y--;


    return (x,y);
}


