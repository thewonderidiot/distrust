all: install

install: distrust
	rustpkg install distrust

distrust:
	rustpkg build distrust
