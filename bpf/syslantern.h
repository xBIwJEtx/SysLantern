/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
#ifndef __SYSLANTERN_H
#define __SYSLANTERN_H
#define COMM_LEN 16
#define MAX_FILENAME_LEN 256

struct exec_event{
    __u32 pid;
    __u32 ppid;
    __u32 uid;
    char comm[COMM_LEN];
    char filename[MAX_FILENAME_LEN];
};

#endif /* __SYSLANTERN_H */