"""One resource supervisor for the entire measurement process and all child groups."""
from pathlib import Path
import json
import os
import shutil
import signal
import subprocess
import threading
import time

_ACTIVE = None


def available_memory_kib():
    return next(int(line.split()[1]) for line in Path('/proc/meminfo').read_text().splitlines()
                if line.startswith('MemAvailable:'))


def descendants_rss(pid):
    pending, seen, total = [pid], set(), 0
    while pending:
        child = pending.pop()
        if child in seen:
            continue
        seen.add(child)
        try:
            status = Path(f'/proc/{child}/status').read_text().splitlines()
            total += next((int(x.split()[1]) for x in status if x.startswith('VmRSS:')), 0)
            for task in Path(f'/proc/{child}/task').iterdir():
                try:
                    pending.extend(map(int, (task / 'children').read_text().split()))
                except FileNotFoundError:
                    pass
        except (FileNotFoundError, ProcessLookupError):
            pass
    return total


class Watch:
    def __init__(self, work, stage="calibration", *, max_children=4):
        self.stage = stage
        self.max_children = max_children
        self.work = Path(work)
        self.work.mkdir(parents=True, exist_ok=True)
        self.lock = threading.RLock()
        self.children, self.failure = {}, None
        self.peak_kib, self.stops = 0, 0
        self.metrics = (self.work / 'resources.jsonl').open('a', buffering=1)
        self.commands = (self.work / 'commands.jsonl').open('a', buffering=1)
        self.check()
        threading.Thread(target=self.monitor, daemon=True).start()

    def check(self):
        with self.lock:
            available = available_memory_kib()
            disk = shutil.disk_usage(self.work).free
            aggregate = 0
            for pid, entry in self.children.items():
                rss = descendants_rss(pid)
                entry['peak_rss_kib'] = max(entry['peak_rss_kib'], rss)
                aggregate += rss
            self.peak_kib = max(self.peak_kib, aggregate)
            row = dict(time=time.time(), stage=self.stage, available_kib=available, free_disk_bytes=disk,
                       active=len(self.children), aggregate_rss_kib=aggregate, peak_rss_kib=self.peak_kib)
            if available < 6 * 1024**2 or aggregate > 16 * 1024**2 or disk < 20 * 1024**3:
                if self.failure is None:
                    self.failure = row
                    self.stops += 1
                    self.metrics.write(json.dumps(row | {'RESOURCE_STOP': True}) + '\n')
                    (self.work / 'resource-stop-1b.json').write_text(json.dumps(row, indent=2))
                for pid in list(self.children):
                    self.terminate(pid)
            return row

    def monitor(self):
        last_log = 0
        while True:
            row = self.check()
            if time.monotonic() - last_log >= 1:
                with self.lock:
                    self.metrics.write(json.dumps(row) + '\n')
                last_log = time.monotonic()
            time.sleep(0.1)

    @staticmethod
    def terminate(pid, sig=signal.SIGTERM):
        try:
            os.killpg(pid, sig)
        except ProcessLookupError:
            pass

    def launch(self, command, **kwargs):
        with self.lock:
            self.check()
            if self.failure:
                raise MemoryError('measurement resource supervisor stopped this stage')
            if len(self.children) >= self.max_children:
                raise RuntimeError(f'at most {self.max_children} active measured commands')
            child = subprocess.Popen(command, start_new_session=True, **kwargs)
            self.children[child.pid] = dict(command=list(map(str, command)), stage=self.stage, pid=child.pid,
                                            start=time.time(), peak_rss_kib=0)
            return child

    def wait(self, child, timeout):
        start = time.monotonic()
        timed_out = False
        while child.poll() is None:
            if self.failure or time.monotonic() - start > timeout:
                timed_out = self.failure is None
                self.terminate(child.pid)
                try:
                    child.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    self.terminate(child.pid, signal.SIGKILL)
                    child.wait()
            else:
                time.sleep(0.1)
        with self.lock:
            row = self.children.pop(child.pid)
            row.update(exit=child.returncode, timeout=timed_out, resource_stop=bool(self.failure),
                       seconds=time.monotonic() - start)
            self.commands.write(json.dumps(row) + '\n')
            (self.work / 'resource-summary.json').write_text(json.dumps(
                dict(stops=self.stops, peak_aggregate_rss_kib=self.peak_kib), indent=2))
        return child.returncode, timed_out, bool(self.failure), row['peak_rss_kib']


def activate(work, stage="calibration", *, max_children=4):
    global _ACTIVE
    if _ACTIVE is not None:
        raise RuntimeError('resource supervisor already active')
    _ACTIVE = Watch(work, stage, max_children=max_children)
    return _ACTIVE


def active():
    if _ACTIVE is None:
        raise RuntimeError('start resource supervisor before any measured command')
    return _ACTIVE
