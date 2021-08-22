###############################################################################
# NAME:		    Makefile
#
# AUTHOR:	    Ethan D. Twardy <edtwardy@mtu.edu>
#
# DESCRIPTION:	    This Makefile is mostly used to prepare binary packages
#		    for distribution to various Linux distros.
#
# CREATED:	    08/21/2021
#
# LAST EDITED:	    08/21/2021
###

binName=renderbars
release=target/release/$(binName)

all: $(release)

$(release): Cargo.toml $(shell find src)
	cargo build --release
	strip $@

install: $(release)
	install -d $(DESTDIR)/usr/bin
	install -m755 $< $(DESTDIR)/usr/bin/$(binName)

clean:
	cargo clean

.PHONY: fake

fake:

version=$(shell awk -F'["=]' '/version/{print $$(NF-1)}' Cargo.toml)
debVersion=1
packageName=renderbars
zipArchive=../$(packageName)_$(version).orig.tar.xz

zip: $(zipArchive)

$(zipArchive): $(shell find .)
	tar cJvf $@ `ls | grep -v '^\.git$$'`

package: $(zipArchive)
	debuild -us -uc

###############################################################################
