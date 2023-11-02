# Chapter 3 lab

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        《你交流的对象说明》

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

- https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter3/5exercise.html#:~:text=%E5%AE%9E%E9%AA%8C%E7%BB%83%E4%B9%A0%E5%8C%85%E6%8B%AC%E5%AE%9E%E8%B7%B5%E4%BD%9C%E4%B8%9A%E5%92%8C%E9%97%AE%E7%AD%94%E4%BD%9C%E4%B8%9A%E4%B8%A4%E9%83%A8%E5%88%86%E3%80%82-,%E5%AE%9E%E8%B7%B5%E4%BD%9C%E4%B8%9A,-%E8%8E%B7%E5%8F%96%E4%BB%BB%E5%8A%A1%E4%BF%A1%E6%81%AF

- 为TaskControlBlock增加field
    - start_time: usize
    - syscall_count: [u32: MAX_SYSCALL_NUM]

- os/src/task/mod.rs
1. 为TaskManager实现syscall_count_increment()
    主要逻辑：
    - 使用match来匹配syscall id，从而决定增加conut的对象。
    - 并在trap_handler中调用。
2. 在run_first_task()中使用get_time_ms()获取当前时间

- os/src/syscall/mod.rs
1. 为TaskInfo实现reveal()
    主要逻辑：
    - 计算运行时间
    - 从Task Control Block中获取task info
    在sys_task_info()调用
