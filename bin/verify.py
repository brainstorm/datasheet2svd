#!/usr/bin/env python
from cmsis_svd.parser import SVDParser

v850_6bit_msb_gone = lambda addr: hex(addr & 0x3fffffff)

parser = SVDParser.for_xml_file("v850-datasheet2svd.svd")
for peripheral in parser.get_device().peripherals:
#    print("%s @ 0x%08x" % (peripheral.name, peripheral.base_address))
    print("%s\t@\t%s" % (peripheral.name, v850_6bit_msb_gone(peripheral.base_address)))
