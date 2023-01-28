
int numTreesVisible = 0;

string[] lines = System.IO.File.ReadAllLines("input.txt");

int maxScore = 1;
int currScore;

for(int k = 1; k < lines.Length-1; ++k)
{
    for(int i = 1; i < lines[0].Length-1; ++i)
    {
        // Part 1
        // if(CheckLeft(k,i) || CheckUp(k,i) || CheckRight(k,i) || CheckBottom(k,i)) {
        //     numTreesVisible++;
        // }
        currScore = CheckLeft2(k,i) * CheckRight2(k,i) * CheckBottom2(k,i) * CheckUp2(k,i);
        if(currScore > maxScore) {
            maxScore = currScore;
        }

    }
}
//Console.WriteLine(numTreesVisible);
Console.WriteLine(maxScore);


// Part 2


int CheckLeft2(int x, int y) {
    int view = 0;
    for(int i = y-1; i >= 0; --i) {

        view++;
        if ( (int)lines[x][y] <= (int)lines[x][i])
        {
            return view;
        }
    }
    return view;
}
int CheckRight2(int x, int y) {
    int view = 0;
    for(int i = y+1; i < lines[0].Length; ++i) {
        view++;
        if ((int)lines[x][y] <= (int)lines[x][i])
        {
            return view;
        }
    }
    return view;
}
int CheckBottom2(int x, int y) {
    int view = 0;
    for(int i = x+1; i < lines.Length; ++i) {
        view++;
        if ((int)lines[x][y] <= (int)lines[i][y])
        {
            return view;
        }
    }
    return view;
}
int CheckUp2(int x, int y) {
    int view = 0;
    for(int i = x-1; i >= 0; --i) {
        view++;
        if ((int)lines[x][y] <= (int)lines[i][y])
        {
            return view;
        }
    }
    return view;
}



// Part 1
bool CheckLeft(int x, int y) {
    for(int i = y-1; i >= 0; --i) {
        if ( (int)lines[x][y] <= (int)lines[x][i])
        {
            return false;
        }
    }
    return true;
}
bool CheckRight(int x, int y) {

    for(int i = y+1; i < lines[0].Length; ++i) {
        if ((int)lines[x][y] <= (int)lines[x][i])
        {
            return false;
        }
    }
    return true;
}
bool CheckBottom(int x, int y) {

    for(int i = x+1; i < lines.Length; ++i) {
        if ((int)lines[x][y] <= (int)lines[i][y])
        {
            return false;
        }
    }
    return true;
}
bool CheckUp(int x, int y) {

    for(int i = x-1; i >= 0; --i) {
        if ((int)lines[x][y] <= (int)lines[i][y])
        {
            return false;
        }
    }
    return true;
}
