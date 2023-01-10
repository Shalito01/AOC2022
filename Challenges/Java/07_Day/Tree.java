import java.util.ArrayList;
import java.util.List;

public class DirectoryTree {
    private class Node {
        private String name;
        private int size;
        private boolean isDir;
        private ArrayList<Node> Children;

        public Node(String str, int sz, boolean type) {
            name = str;
            size = sz;
            isDir = type;
            if(type) {
                Children = new ArrayList<Node>();
            }
        }
    }

    private Node root;

    public DirectoryTree(String name, int size, boolean type) {
        root = new Node(name, 0, true);
    }


}
