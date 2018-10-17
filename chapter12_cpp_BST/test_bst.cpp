#include "bst.hpp"

#include <iostream>
#include <random>
#include <ctime>
using namespace std;

int main()
{
    BinarySearchTree<int> t; // 创建一个二叉搜索树

    uniform_int_distribution<unsigned int> u(0, 200); // 设置随机数分布
    default_random_engine e(time(0));                 // 设置随机数引擎（通过时间作为种子）

    cout << "==== 测试插入：" << endl;
    for (size_t i = 0; i < 8; ++i)
    {
        t.insert(u(e));
    }
    cout << "==== 测试打印：" << endl;
    t.printTree();
    cout << "==== 测设删除（删除小于100的数）：" << endl;
    for (size_t i = 0; i < 100; ++i)
    {
        t.remove(i);
    }
    t.printTree();
    cout << "==== 测试拷贝构造函数：" << endl;
    BinarySearchTree<int> t2(t);
    t2.printTree();
    cout << "==== 测试赋值操作：" << endl;
    BinarySearchTree<int> t3;
    t3 = t;
    t.printTree();
    cout << "==== 测试最大最小值：" << endl;
    cout << "最大值：" << t.findMax() << endl;
    cout << "最小值：" << t.findMin() << endl;

    return 0;
}