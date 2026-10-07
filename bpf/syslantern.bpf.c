#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

SEC("tracepoint/syscalls/sys_enter_execve")
int handle_execve(struct trace_event_raw_sys_enter *ctx)
{
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    char comm[16];

    bpf_get_current_comm(&comm, sizeof(comm));
    bpf_printk("execve pid=%d comm=%s", pid, comm);
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";