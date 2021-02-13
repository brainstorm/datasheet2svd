#!/usr/bin/env python
from cmsis_svd.parser import SVDParser

parser = SVDParser.for_xml_file("v850-datasheet2svd.svd")
for peripheral in parser.get_device().peripherals:
    print("%s @ 0x%08x" % (peripheral.name, peripheral.base_address))
