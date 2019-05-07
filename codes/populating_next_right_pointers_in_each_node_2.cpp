class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() {}

    Node(int _val, Node* _left, Node* _right, Node* _next) {
        val = _val;
        left = _left;
        right = _right;
        next = _next;
    }
};
class Solution {
public:
    Node* connect(Node* root) {
      Node* pre = root;
      Node* dummy = new Node();
      while (pre) {
        Node* cur = dummy;
        dummy->next = nullptr;
        while (pre) {
          if (pre->left)
            cur = cur->next = pre->left;
          if (pre->right)
            cur = cur->next = pre->right;
          pre = pre->next;
        }
        pre = dummy->next;
      }
      delete dummy;
      return root;
    }
};
