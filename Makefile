.PHONY: all
all: peripheralio.json peripheralio.csv

clean:
	rm -r build/

tabula.jar:
	wget https://github.com/tabulapdf/tabula-java/releases/download/v1.0.4/tabula-1.0.4-jar-with-dependencies.jar -O build/tabula.jar

peripheralio.json:
	java -jar build/tabula.jar -p 76-82 -l -f JSON build/nec-μPD703128.pdf -o build/peripheralio.json

peripheralio.csv:
	java -jar build/tabula.jar -p 76-82 -l -f CSV build/nec-μPD703128.pdf -o build/peripheralio.csv

opcodes.json:
	java -jar build/tabula.jar -p 424-430 -l -f JSON datasheets/r01us0001ej0100_v850e2m.pdf -o build/opcodes.json

opcodes.csv:
	java -jar build/tabula.jar -p 424-430 -l -f CSV datasheets/r01us0001ej0100_v850e2m.pdf -o build/opcodes.csv

opcodes-basic-insns.csv:
	java -jar build/tabula.jar -p 418-421 -l -f CSV datasheets/r01us0001ej0100_v850e2m.pdf -o build/opcodes-basic-insns.csv

peripherals.csv:
	java -jar build/tabula.jar -p 76-82 -l -f CSV datasheets/nec-μPD703128.pdf -o build/peripherals.csv
