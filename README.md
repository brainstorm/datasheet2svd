# Description

WIP! WIP! WIP!

This is the missing piece between IC manufacturers that just publish datasheets without SVDs and [svd2rust][svd2rust]. It uses [tabula](https://tabula.technology/), an specialised OCR that recognises tables in PDFs.

This can also be used as the ideal companion for reverse engineering tools such as [Radare2](https://github.com/radareorg/radare2-extras/tree/master/r2svd) or [Ghidra SVD loaders](https://github.com/leveldown-security/SVD-Loader-Ghidra).

Future goal(s) are generating a PAC for the Renesas V850, but LLVM does not have such a backend and GCC-Rust is still WIP-ing at this point in time.

But at least we have a SVD for it now, that's a start :)

## TODO

* [x]: Make sure it loads well on radare2...
* [ ]: ...and Ghidra's SVD-Loader.
* [ ]: Collapse all the peripherals repetition into a single one and several registers inside it.
* [ ]: Cleanup/generalize for other ICs (RL78, V810, etc...)
* [ ]: I should have used https://crates.io/crates/xml-schema-derive instead of manual structs, maybe?


```shell
$ cargo run -q > v850.svd
```

## Rust XML state of the crates nation

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

There's also Rust embedded's [SVD parser](https://github.com/rust-embedded/svd) attempting to work with [Minicom XML](https://github.com/rust-embedded/svd/pull/138)... my very much non-expert opinion is that they [should take look at YaSerDe instead][yaserde_rfc_talk].

[xml_prettyprint]: https://www.samltool.com/prettyprint.php
[quickxml_serde_shortcomings]: https://github.com/tafia/quick-xml/issues/245
[svd2rust]: https://github.com/rust-embedded/svd2rust
[yaserde_docs]: https://github.com/media-io/yaserde/pull/106
[yaserde_rfc_talk]: https://users.rust-lang.org/t/rfc-serde-xml-support/737
