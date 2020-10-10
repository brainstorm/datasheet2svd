.PHONY: all
all: peripheralio.json peripheralio.csv

#clean:
#	rm -r build/

#v850-datasheet.pdf:
#    mkdir build 2>/dev/null
#    wget https://www.espressif.com/sites/default/files/documentation/esp8266-technical_reference_en.pdf -O build/esp8266-technical_reference_en.pdf

#appendix.pdf: esp8266-technical_reference_en.pdf
#    qpdf --empty --pages build/esp8266-technical_reference_en.pdf 113-116 -- build/appendix.pdf

tabula.jar:
	wget https://github.com/tabulapdf/tabula-java/releases/download/v1.0.4/tabula-1.0.4-jar-with-dependencies.jar -O build/tabula.jar

peripheralio.json:
	java -jar build/tabula.jar -p 76-82 -l -f JSON build/nec-μPD703128.pdf -o build/peripheralio.json

peripheralio.csv:
	java -jar build/tabula.jar -p 76-82 -l -f CSV build/nec-μPD703128.pdf -o build/peripheralio.csv
