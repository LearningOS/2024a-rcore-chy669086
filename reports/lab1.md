## 实现的功能
我在 `TaskManagerInner` 中加入了两个元素 `task_info` 和 `start_time`，分别代表对于任务的信息和启动时间。每次任务启动的时候，去判断 `start_time` 是否为 0，是的话代表这个这个任务第一次启动，我会将他设置为当前时间。

对于 `syssys_task_info` 的调用，我在外部设置了三个新函数，分别是 `get_running_task_start_time`、`get_running_task_info` 和 `get_running_taskid`，分别用来获取当前任务的 启动时间、任务信息和 ID。然后我会根据 start_time 更新信息的时间，然后复制给传入的 `TaskInfo`。

对于计数问题，我设立了一个辅助函数 `add_syscall_time`，他接收 `syscall_id` 作为参数，取得当前运行的 TaskInfo 然后在对应位置的计数加一。

## 问答题
### 1

sbi 版本：RustSBI-QEMU Version 0.2.0-alpha.2

`ch2b_bad_address.rs`
```text
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
```

访问 0x0 地址的时候被内核阻止。

`ch2b_bad_instructions.rs` && `ch2b_bad_register.rs`
```text
[kernel] IllegalInstruction in application, kernel killed it.
```

### 2

#### 1
a0 是内核栈压入 Trap 上下文之后的栈顶

上一个应用结束时，这个应用被启动的时候

发起系统调用后回到这个应用时

#### 2
处理了 `sstatus`，`sepc`，`sscratch` 寄存器

`sstatus`：给出 Trap 发生前在哪个特权级，用于恢复到用户态。

`sepc`：当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址，用于跳回正确的执行位置

`sscratch`：指向用户栈栈顶。

#### 3

x2 寄存器在前文已经被恢复 L45

x4 寄存器，除非我们手动出于一些特殊用途使用它，否则一般也不会被用到。

#### 4

sp 指向用户栈栈顶，sscratch 指向 系统栈栈顶

#### 5

L61 sret 的时候

sret 会根据 sstatus 的 SPP 字段设置特权级为 U/S，在这个函数中会恢复到 U 特权级

#### 6

sp 指向内核栈， sscratch 指向用户栈

#### 7

L38 call trap_handler

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

>无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

>无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。