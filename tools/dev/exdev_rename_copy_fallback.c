#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

static int copy_path_to_path(const char *src, const char *dst) {
    int in_fd = open(src, O_RDONLY);
    if (in_fd < 0) {
        return -1;
    }

    struct stat st;
    if (fstat(in_fd, &st) != 0) {
        close(in_fd);
        return -1;
    }

    int out_fd = open(dst, O_WRONLY | O_CREAT | O_TRUNC, st.st_mode & 0777);
    if (out_fd < 0) {
        close(in_fd);
        return -1;
    }

    char buf[1 << 16];
    for (;;) {
        ssize_t n = read(in_fd, buf, sizeof(buf));
        if (n == 0) {
            break;
        }
        if (n < 0) {
            close(in_fd);
            close(out_fd);
            return -1;
        }
        char *p = buf;
        ssize_t left = n;
        while (left > 0) {
            ssize_t w = write(out_fd, p, left);
            if (w < 0) {
                close(in_fd);
                close(out_fd);
                return -1;
            }
            p += w;
            left -= w;
        }
    }

    close(in_fd);
    if (close(out_fd) != 0) {
        return -1;
    }
    return 0;
}

int rename(const char *oldpath, const char *newpath) {
    static int (*real_rename)(const char *, const char *) = NULL;
    if (!real_rename) {
        real_rename = dlsym(RTLD_NEXT, "rename");
    }

    int rc = real_rename(oldpath, newpath);
    if (rc == 0) {
        return 0;
    }
    if (errno == EXDEV && oldpath && newpath) {
        if (copy_path_to_path(oldpath, newpath) == 0) {
            unlink(oldpath);
            return 0;
        }
    }
    return rc;
}

int renameat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath) {
    static int (*real_renameat)(int, const char *, int, const char *) = NULL;
    if (!real_renameat) {
        real_renameat = dlsym(RTLD_NEXT, "renameat");
    }

    int rc = real_renameat(olddirfd, oldpath, newdirfd, newpath);
    if (rc == 0) {
        return 0;
    }
    if (errno == EXDEV && olddirfd == AT_FDCWD && newdirfd == AT_FDCWD && oldpath && newpath) {
        if (copy_path_to_path(oldpath, newpath) == 0) {
            unlink(oldpath);
            return 0;
        }
    }
    return rc;
}

int renameat2(
    int olddirfd,
    const char *oldpath,
    int newdirfd,
    const char *newpath,
    unsigned int flags
) {
    static int (*real_renameat2)(int, const char *, int, const char *, unsigned int) = NULL;
    if (!real_renameat2) {
        real_renameat2 = dlsym(RTLD_NEXT, "renameat2");
    }

    int rc = real_renameat2(olddirfd, oldpath, newdirfd, newpath, flags);
    if (rc == 0) {
        return 0;
    }
    if (
        errno == EXDEV && flags == 0 && olddirfd == AT_FDCWD && newdirfd == AT_FDCWD && oldpath
        && newpath
    ) {
        if (copy_path_to_path(oldpath, newpath) == 0) {
            unlink(oldpath);
            return 0;
        }
    }
    return rc;
}
