## 实现

`sys_spawn` 使用了 `TaskControlBlock::new` 创建新进程，并和当前线程设置父子关系。

`sys_set_priority` 在 `TaskControlBlock` 中新增了 `priority` `stride` 两个元素，全局变量 `BIG_STRIDE`。每次进行任务的时候给当前任务的 `stride` 加上 `BIG_STRIDE / priority`。取出任务的时候直接遍历找到最小的那个任务。

## 问答作业

- 实际情况是轮到 p1 执行吗？为什么？

不是，因为 $250_{u8} + 10_{u8} = 4_{u8} < 255_{u8}$，会溢出导致 p2 继续执行。

- 为什么？尝试简单说明（不要求严格证明）。

当 优先级 == 2 的时候，两个一样 stride 的任务 t1, t2 

- 已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 > other.0 {
            if self.0 - other.0 < 255 / 2 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else {
            if other.0 - self.0 < 255 / 2 {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

>无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

>无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。git