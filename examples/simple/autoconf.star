# get the current compiler
cc = autoconf.get_compiler()

cdata = configuration_data()
cdata.set("HAVE_SOME_HEADER", cc.has_header("some_header.h"))
cdata.set("HAVE_ANOTHER_HEADER", cc.has_header("another_header.h"))
