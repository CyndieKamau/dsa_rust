//In a singly linked list, each node has some data, and reference to the next node.
//It has no info about the other nodes, or the node before it
//It only points to the next node, till the last node has no node to point to
//With the analogy of a Russian doll, the head doll (node) points to the inner doll, which 
//points to the inner doll, till they get to the innermost doll.
//In a double linked list, the doll can point to both the next doll, and the previous one.
// STEPS TO CREATE THE LINKED LIST
//First define how a single doll (node)looks like; we'll use a struct -> holds doll's generic data, link to next doll.
//We'll have an enum with 2 variants -> either its a doll, or end of the list, meaning it'll point to an
//empty node.
//Another struct for `LinkedList` -> for holding the head doll

//enum Link, either list is empty, or its a node
enum Link<T> {
    Empty,
    NodeList(Box<Node<T>>),
}

struct Node<T> {
    data: T,
    next_node: Link<T>,
}

struct LinkedList<T> {
    head_node: Link<T>
}


fn main() {
    println!("Hello, world!");
}
