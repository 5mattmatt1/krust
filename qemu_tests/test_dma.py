"""
DMA Testing (Direct Memory Access)

A suite of tests ensuring that data can be properly copied between two arrays using
DMA in QEMU.
"""
# python -m unittest discover qemu_tests

from unittest import TestCase, main
from qemu import QEMUTestCase

class DMATestCase(QEMUTestCase):
    def setUp(self):
        super(DMATestCase, self).setUp('test_dma.img')
        
    def tearDown(self):
        super(DMATestCase, self).tearDown()
    
    def test_dma(self):
        self.assertLine("test_src_inc_dst_inc")
        self.assertLine("[AC, DC, FF, FF, DE, AD, BE, EF]")
        self.assertLine("test_src_dst_inc")
        self.assertLine("[AC, AC, AC, AC, AC, AC, AC, AC]")

if __name__ == "__main__":
    main()