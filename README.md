# Description

This is the missing piece between IC manufacturers that just publish datasheets without SVDs and [svd2rust][svd2rust].

It can also be seen as the ideal companion for reverse engineering tools such as Radare2 or Ghidra SVD loaders. 

# Quickstart

WIP! But when finished this tool will generate a correct SVD definition ready to be loaded into either radare2, Ghidra and/or generate a HAL or PAC via svd2rust. 

```shell
$ cargo run -q
```

## Rust XML state of the nation

This blogpost from simplabs explains a few of the [pain points of XML with Rust in 2021](https://simplabs.com/blog/2020/12/31/xml-and-rust/). There's also a matrix of what's [supported and what is not on different crates](https://github.com/RazrFalcon/roxmltree#alternatives), which gives a more detailed idea on Rust's maturity w.r.t XML (de)serializing.

Default serde structs do not seem to be aware of the desired serialized structure (i.e child vs attrs) so we end up with **an incorrect CMSIS-SVD definition**:

```xml
<?xml version="1.0"?>
<xml version="1.0" encoding="utf-8">
  <device xmlns="http://www.w3.org/2001/XMLSchema-instance" schemaversion="foo" xsnonamespaceschemalocation="CMSIS-SVD.xsd">
    <devattributes vendor="Renesas" vendorid="Renesas" name="V850" series="E1/E2/CA2" version="1.2" description="NEC/Renesas V850 automotive grade ICs" licensetext="GPLv3" addressunitbits="8" width="32" size="32" access="read-write" resetvalue="0x00000000" resetmask="0xFFFFFFFF">
      <cpu name="V850" revision="r1" endian="LE" mpupresent="false" fpupresent="false" vendorsystickconfig="false"/>
      <peripherals name="Specific Registers" version="1.0" description="Specific Registers" groupname="MCU" baseaddress="0xFFFFF1FC" size="16" access="read-write">
        <registers name="PRCMD" description="This command register (PRCMD) is to protect the registers that may have a significant influence on the application system (PSC, PSM) from an inadvertent write access, so that the system does not stop in case of a program hang-up." addressoffset="0xFFFFF1FC" size="8" access="read-write" resetvalue="0x0000" resetmask="0xFFFF"/>
      </peripherals>
    </devattributes>
  </device>
</xml>
```

The XML above is inconsistent with CMSIS-SVD since most of the information is encoded as **attributes** when they should be **child** elements instead, that's why `strong-xml`, which is heavily typed, is a better fit for this particular XML schema.

Unfortunately it lacks good examples.

YaSerDe on the other hand, it's intuitive and just works. Every member of the struct gets labeled as either attribute or child [and it has good docs][yaserde_docs].

[xml_prettyprint]: https://www.samltool.com/prettyprint.php
[quickxml_serde_shortcomings]: https://github.com/tafia/quick-xml/issues/245
[svd2rust]: https://github.com/rust-embedded/svd2rust
[yaserde_docs]: https://github.com/media-io/yaserde/pull/106
