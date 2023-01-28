using System;
using System.Collections.Generic;
using System.IO;

namespace DaySeven
{

    public class DaySeven {
        private static int sumLessThanCentomila = 0;
        public static void SumCheck(int x) {
            sumLessThanCentomila += x;
        }
        public static void Main(string[] args) {
            var lines = File.ReadLines("input.txt").ToList();
            var rootDir = new Directory("/",null);
            var currDir = rootDir;
            var directories = new List<Directory> { rootDir };

            for(int i = 0; i < lines.Count; ++i) {
                var cmd = lines[i].Split();

                switch (cmd[1])
                {
                    case "cd":
                        currDir = cmd[2] switch
                        {
                            "/" => rootDir,
                            ".." => currDir.Parent ?? currDir,
                            _ => currDir.SubDirectories.First(d => d.Name == cmd[2])
                        };
                        break;
                    case "ls":
                        var listOutput = lines.Skip(i+1).TakeWhile(s => !s.StartsWith("$"));
                        foreach (var item in listOutput)
                        {
                            var itemPart = item.Split();

                            if (itemPart[0] == "dir")
                            {
                                var subDir = new Directory(itemPart[1], currDir);
                                currDir.SubDirectories.Add(subDir);
                                directories.Add(subDir);
                            }
                            else
                            {
                                var fileSize = Int32.Parse(itemPart[0]);
                                currDir.Files.Add(new SystemFile(itemPart[1], fileSize));
                                currDir.Size += fileSize;
                            }
                        }

                        i += listOutput.Count();
                        break;
                }
            }
            long[] sizes = new long[directories.Count()] ;
            var totalSum = 0;
            int k = 0;
            foreach(var dir in directories)
            {
                int x = CalculateDirectorySize(dir);

                sizes[k] = x;
                totalSum += x < 100000 ? x : 0;
                ++k;
            }
            Console.WriteLine(totalSum);
            long rootSize = CalculateDirectorySize(rootDir);
            long freeSpace = 70000000 - rootSize;
            long toFree = 30000000 - freeSpace;
            Array.Sort(sizes);
            foreach(var x in sizes)
            {
                Console.Write("Size: {0} | finalSize: {1} | alreadyfree: {2}\n", x, rootSize-x, freeSpace);
                if(rootSize - x <= 40000000) {
                    Console.WriteLine("\n\nSize: " + x);
                    return;
                }
            }


        }

        static int CalculateDirectorySize(Directory directory)
        {
            if (!directory.SubDirectories.Any()) return directory.Size;

            return directory.Size + directory.SubDirectories.Sum(CalculateDirectorySize);
        }
    }




    public class Directory {


        public Directory(string name, Directory? parent) {
            Name = name;
            Parent = parent;
        }
        public string Name { get; set; }
        public Directory? Parent { get; set; }
        public ICollection<Directory> SubDirectories { get; set; } = new List<Directory>();
        public ICollection<SystemFile> Files { get; set; } = new List<SystemFile>();
        public int Size { get; set; }

}
    public class SystemFile {
        public string Name { get; set; }
        public long Size { get; set; }

        public SystemFile(string name, int size) {
            Name = name;
            Size = size;
        }

    }
}
