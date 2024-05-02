# rust leetcode 本地刷题模板
## commen_def
`commen_def` 模块提供的功能提供了以下功能, 方便在本地环境快速进行测试.
1. 快速创建多维数组
```rust
let vec = vecnd![[0, 0, 0], [1, 1, 0], [1, 1, 0]];
```
2. 快速创建链表
```rust
let head = ListNode::from_vec(vec![5, 5, 0]);
```
3. 快速创建二叉树
```rust
let root = TreeNode::from_str("[1,7,0,7,-8,null,null]");
```

## 如何开始
可参考[这个提交](https://github.com/should-smile/rust_leetcode/commit/b41813a209ace62d7b91c00589d257ffa109a314)的修改, 在本项目中开始编写代码, 这个提交对应的题目是[974. 和可被 K 整除的子数组](https://leetcode.cn/problems/subarray-sums-divisible-by-k/description/).