# Chapter 4 lab

- https://learningos.cn/rCore-Tutorial-Guide-2023A/chapter4/7exercise.html

- os/mm/memory_set.rs
    - mmap
        - 将vpn与ppn关联，从start_vpn遍历至end_vpn，若已存在对应pte，则返回失败，若不存在则分配ppn
    - munmap
        - 将vpn从page table移除


- os/syscall/process.rs
    - sys_get_time()
        - 用 translated_byte_buffer 将用户地址转换为对应的物理地址，然后读取TimeVal大小的内存
    - sys_task_info()
        - 同理先转化为对应物理地址，然后读取TaskInfo大小的内存

