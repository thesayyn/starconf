# get the current compiler
cc = autoconf.get_compiler()

cdata = configuration_data()
cdata.set("HAVE_SOME_HEADER", cc.has_header("stddef.h"))

# comment out the next file to make the compilation fail
cdata.set("HAVE_ANOTHER_HEADER", cc.has_header("string.h"))


configure_file(
    input = config_in(),
    output = config_out(),
    configuration = cdata
)
