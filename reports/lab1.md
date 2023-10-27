# Chapter 3 lab

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
