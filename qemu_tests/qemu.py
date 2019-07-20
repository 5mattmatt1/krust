# Only Python3 is supported for these guys
from unittest import TestCase, main
from subprocess import Popen, PIPE

class QEMUTestCase(TestCase):
    def setUp(self, kernel):
        QEMU_START_COMMAND = ['qemu-system-aarch64', '-M', 'raspi3', '-kernel',
                            kernel, '-semihosting', '-serial', 'null', '-serial', 'stdio']
        self.proc = Popen(QEMU_START_COMMAND, stdin=PIPE, stdout=PIPE)

    def assertLine(self, line):
        read_line = self.proc.stdout.readline().strip().decode('utf-8')
        self.assertEqual(read_line, line)

    def tearDown(self):
        self.proc.stdin.close()
        self.proc.wait()
        self.proc.terminate()