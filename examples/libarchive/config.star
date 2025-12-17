# libarchive starconf configuration
# Ported from meson.build: https://github.com/mesonbuild/wrapdb/blob/master/subprojects/packagefiles/libarchive/meson.build

VERSION = "3.7.9"

# Helper function to convert strings like "long long" to "LONG_LONG"
def underscorify(s):
    return s.replace(" ", "_").replace(".", "_").replace("-", "_").replace("/", "_")

cc = autoconf.get_compiler()
cdata = configuration_data()

# Version strings
cdata.set("VERSION", VERSION)
cdata.set("BSDCAT_VERSION_STRING", VERSION)
cdata.set("BSDCPIO_VERSION_STRING", VERSION)
cdata.set("BSDTAR_VERSION_STRING", VERSION)
cdata.set("LIBARCHIVE_VERSION_STRING", VERSION)
cdata.set("SAFE_TO_DEFINE_EXTENSIONS", 1)

# Platform-specific settings
if host_machine.system() == "windows":
    cdata.set("_WIN32_WINNT", "0x0A00", description="Windows 10 APIs")
    cdata.set("NTDDI_VERSION", "0x0A000000", description="Windows 10 APIs")
    cdata.set("WINVER", "0x0A00", description="Windows 10 APIs")

# macOS-specific settings
if host_machine.system() == "darwin":
    # macOS has these headers but starconf may not detect them in sandboxed builds
    darwin_headers = [
        "copyfile.h",
        "membership.h",
        "sys/acl.h",
        "sys/mount.h",
        "sys/param.h",
        "sys/statvfs.h",
        "sys/xattr.h",
    ]
    for h in darwin_headers:
        cdata.set("HAVE_" + underscorify(h).upper(), 1)

    # macOS xattr support (ACL support disabled due to API differences)
    cdata.set("ARCHIVE_XATTR_DARWIN", 1)
    cdata.set("HAVE_DECL_XATTR_NOFOLLOW", 1)

    # macOS struct members
    cdata.set("HAVE_STRUCT_STAT_ST_BIRTHTIME", 1)
    cdata.set("HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC", 1)
    cdata.set("HAVE_STRUCT_STAT_ST_MTIMESPEC_TV_NSEC", 1)
    cdata.set("HAVE_STRUCT_STAT_ST_FLAGS", 1)
    # Note: macOS statvfs doesn't have f_iosize, only statfs does

    # macOS doesn't have futimesat (Linux-specific)
    # The HAVE_FUTIMESAT will not be set, so libarchive uses alternatives

# File offset bits for 32-bit systems
if cc.sizeof("long") == 4:
    cdata.set("_FILE_OFFSET_BITS", 64)

# =============================================================================
# Type detection - stdint types
# =============================================================================
stdint_types = [
    "__int64",
    "int16_t",
    "int32_t",
    "int64_t",
    "intmax_t",
    "uint8_t",
    "uint16_t",
    "uint32_t",
    "uint64_t",
    "uintmax_t",
    "unsigned __int64",
]

for t in stdint_types:
    if cc.has_type(t, prefix="#include <stdint.h>"):
        cdata.set("HAVE_" + underscorify(t).upper(), 1)

# =============================================================================
# Size detection - basic types
# =============================================================================
size_types = [
    "short",
    "int",
    "long",
    "long long",
    "unsigned",
    "unsigned long",
    "unsigned long long",
    "unsigned short",
]

for s in size_types:
    upper = underscorify(s).upper()
    size = cc.sizeof(s)
    # Use @VAR@ as key since starconf matches entire lines
    cdata.set(
        "@SIZEOF_{}_CODE@".format(upper),
        "#define SIZEOF_{} {}".format(upper, size),
    )

# wchar_t detection
cdata.set10("HAVE_WCHAR_T", cc.has_type("wchar_t", prefix="#include <wchar.h>"))
cdata.set("SIZEOF_WCHAR_T", cc.sizeof("wchar_t", prefix="#include <wchar.h>"))

# =============================================================================
# Min/max constant detection
# =============================================================================
minmax = {
    "INT32_MAX": "stdint.h",
    "INT32_MIN": "stdint.h",
    "INT64_MAX": "stdint.h",
    "INT64_MIN": "stdint.h",
    "INTMAX_MAX": "stdint.h",
    "INTMAX_MIN": "stdint.h",
    "SIZE_MAX": "stdint.h",
    "SSIZE_MAX": "limits.h",
    "UINT32_MAX": "stdint.h",
    "UINT64_MAX": "stdint.h",
    "UINTMAX_MAX": "stdint.h",
}

for m, h in minmax.items():
    if cc.has_header_symbol(h, m):
        cdata.set("HAVE_DECL_" + m, 1)

# =============================================================================
# Specific header symbol checks
# =============================================================================
if cc.has_header_symbol("langinfo.h", "D_MD_ORDER"):
    cdata.set("HAVE_D_MD_ORDER", 1)

if cc.has_header_symbol("linux/fs.h", "FS_IOC_GETFLAGS"):
    cdata.set("HAVE_WORKING_FS_IOC_GETFLAGS", 1)

if cc.has_header_symbol("errno.h", "EFTYPE"):
    cdata.set("HAVE_EFTYPE", 1)

if cc.has_header_symbol("errno.h", "EILSEQ"):
    cdata.set("HAVE_EILSEQ", 1)

if cc.has_header_symbol("ext2fs/ext2_fs.h", "EXT2_IOC_GETFLAGS"):
    cdata.set("HAVE_WORKING_EXT2_IOC_GETFLAGS", 1)

if cc.has_header_symbol("unistd.h", "sysconf"):
    cdata.set("HAVE_SYSCONF", 1)

# Time-related symbols
time_symbols = ["_mkgmtime", "ctime_s", "gmtime_s", "localtime_s"]
for w in time_symbols:
    if cc.has_header_symbol("time.h", w):
        cdata.set("HAVE_" + w.upper(), 1)

# =============================================================================
# Struct member detection
# =============================================================================
stat_members = [
    "st_birthtime",
    "st_birthtimespec.tv_nsec",
    "st_mtimespec.tv_nsec",
    "st_mtim.tv_nsec",
    "st_mtime_n",
    "st_umtime",
    "st_mtime_usec",
    "st_blksize",
    "st_flags",
]

for m in stat_members:
    if cc.has_member("struct stat", m, prefix="#include <sys/stat.h>"):
        cdata.set("HAVE_STRUCT_STAT_" + underscorify(m).upper(), 1)

if cc.has_member("struct tm", "tm_gmtoff", prefix="#include <time.h>"):
    cdata.set("HAVE_STRUCT_TM_TM_GMTOFF", 1)

# =============================================================================
# ACL type detection
# =============================================================================
acl_types = ["acl_t", "acl_entry_t", "acl_permset_t", "acl_tag_t"]
for a in acl_types:
    if cc.has_type(a, prefix="#include <sys/acl.h>"):
        cdata.set("HAVE_" + a.upper(), 1)

# =============================================================================
# DIR header detection
# =============================================================================
dir_headers = ["dirent.h", "sys/ndir.h", "sys/dir.h", "ndir.h"]
for h in dir_headers:
    if cc.has_header_symbol(h, "DIR"):
        cdata.set("HAVE_" + underscorify(h).upper(), 1)
        break

# =============================================================================
# Time header detection
# =============================================================================
time_headers = ["sys/types.h", "sys/time.h", "time.h"]
for h in time_headers:
    if cc.has_member("struct tm", "tm_sec", prefix="#include <{}>".format(h)):
        cdata.set("TIME_WITH_SYS_TIME", 1)
        break

# =============================================================================
# Misc symbol checks
# =============================================================================
if cc.has_header_symbol("sys/sysmacros.h", "major"):
    cdata.set("MAJOR_IN_SYSMACROS", 1)

if cc.has_header_symbol("crtdbg.h", "_CrtSetReportMode"):
    cdata.set("HAVE__CrtSetReportMode", 1)

# =============================================================================
# Type fallback definitions
# Note: These are only needed on systems that lack these POSIX types.
# Most modern systems (Linux, macOS, BSD) have them, so we skip this section.
# If building for exotic platforms, uncomment and adjust as needed.
# =============================================================================
# type_fallbacks = {
#     "dev_t": "unsigned int",
#     "id_t": "short",
#     "gid_t": "short",
#     "mode_t": "unsigned short",
#     "off_t": "int64_t",
#     "pid_t": "int",
#     "size_t": "unsigned long",
#     "ssize_t": "ptrdiff_t",
#     "uid_t": "short",
# }
# for t, v in type_fallbacks.items():
#     if not cc.has_type(t, prefix="#include <sys/types.h>"):
#         cdata.set(t, v)

# =============================================================================
# Platform-specific xattr
# =============================================================================
if host_machine.system() in ["cygwin", "linux"]:
    cdata.set("ARCHIVE_XATTR_LINUX", 1)

# =============================================================================
# Header detection
# =============================================================================
headers = [
    "acl/libacl.h",
    "attr/xattr.h",
    # "blake2.h",  # Disabled - would need libb2
    # "bzlib.h",  # Disabled - would need bzip2
    "copyfile.h",
    "ctype.h",
    "direct.h",
    "dlfcn.h",
    "errno.h",
    "ext2fs/ext2_fs.h",
    "expat.h",
    "fcntl.h",
    "grp.h",
    "iconv.h",
    "inttypes.h",
    "io.h",
    "langinfo.h",
    "libxml/xmlreader.h",
    "libxml/xmlwriter.h",
    "limits.h",
    "linux/ext2_fs.h",
    "linux/fiemap.h",
    "linux/fs.h",
    "linux/magic.h",
    "linux/types.h",
    "localcharset.h",
    "locale.h",
    "lz4.h",
    "lz4hc.h",
    "lzma.h",
    "membership.h",
    "memory.h",
    "openssl/evp.h",
    "paths.h",
    "pcre2posix.h",
    "pcreposix.h",
    "poll.h",
    "process.h",
    "pthread.h",
    "pwd.h",
    "readpassphrase.h",
    "regex.h",
    "signal.h",
    "spawn.h",
    "stdarg.h",
    "stdint.h",
    "stdlib.h",
    "string.h",
    "strings.h",
    "sys/acl.h",
    "sys/cdefs.h",
    "sys/ea.h",
    "sys/extattr.h",
    "sys/ioctl.h",
    "sys/mkdev.h",
    "sys/mount.h",
    "sys/param.h",
    "sys/poll.h",
    "sys/richacl.h",
    "sys/select.h",
    "sys/stat.h",
    "sys/statfs.h",
    "sys/statvfs.h",
    "sys/sysmacros.h",
    "sys/time.h",
    "sys/types.h",
    "sys/utime.h",
    "sys/utsname.h",
    "sys/vfs.h",
    "sys/wait.h",
    "sys/xattr.h",
    "time.h",
    "unistd.h",
    "utime.h",
    "wchar.h",
    "wctype.h",
    "wincrypt.h",
    "windows.h",
    "winioctl.h",
    "zlib.h",
    "zstd.h",
]

for h in headers:
    if cc.has_header(h):
        cdata.set("HAVE_" + underscorify(h).upper(), 1)

# =============================================================================
# Function detection
# =============================================================================
functions = [
    "arc4random_buf",
    "chflags",
    "chown",
    "chroot",
    "ctime_r",
    "cygwin_conv_path",
    "dirfd",
    "fchdir",
    "fchflags",
    "fchmod",
    "fchown",
    "fcntl",
    "fdopendir",
    "fgetxattr",
    "flistxattr",
    "fork",
    "fseeko",
    "fsetxattr",
    "fstat",
    "fstatat",
    "fstatfs",
    "fstatvfs",
    "ftruncate",
    "futimens",
    "futimes",
    # "futimesat",  # Linux-specific, not on macOS
    "geteuid",
    "getgrgid_r",
    "getgrnam_r",
    "getpwnam_r",
    "getpwuid_r",
    "getpid",
    "getvfsbyname",
    "getxattr",
    "gmtime_r",
    "iconv",
    "lchflags",
    "lchmod",
    "lchown",
    "lgetxattr",
    "link",
    "linkat",
    "listxattr",
    "llistxattr",
    "localtime_r",
    "lsetxattr",
    "lstat",
    "lutimes",
    "mbrtowc",
    "memmove",
    "mkdir",
    "mkfifo",
    "mknod",
    "mkstemp",
    "nl_langinfo",
    "openat",
    "pipe",
    "poll",
    "posix_spawnp",
    "readdir_r",
    "readlink",
    "readlinkat",
    "readpassphrase",
    "regcomp",
    "select",
    "setenv",
    "setlocale",
    "setxattr",
    "sigaction",
    "statfs",
    "statvfs",
    "strchr",
    "strdup",
    "strerror",
    "strerror_r",
    "strftime",
    "strncpy_s",
    "strnlen",
    "strrchr",
    "symlink",
    "tcgetattr",
    "tcsetattr",
    "timegm",
    "tzset",
    "unlinkat",
    "unsetenv",
    "utime",
    "utimes",
    "utimensat",
    "vprintf",
    "vfork",
    "wcrtomb",
    "wcscmp",
    "wcscpy",
    "wcslen",
    "wctomb",
    "wmemcmp",
    "wmemcpy",
    "wmemmove",
    "_fseeki64",
    "_get_timezone",
]

for f in functions:
    if cc.has_function(f):
        cdata.set("HAVE_" + underscorify(f).upper(), 1)

# strerror_r declaration
if cc.has_function("strerror_r"):
    cdata.set("HAVE_DECL_STRERROR_R", 1)

# =============================================================================
# Regex detection
# =============================================================================
if cc.has_function("regcomp", prefix="#include <regex.h>"):
    cdata.set("HAVE_REGEX_H", 1)
elif cc.has_header("pcre2posix.h"):
    cdata.set("HAVE_PCRE2POSIX_H", 1)
elif cc.has_header("pcreposix.h"):
    cdata.set("HAVE_PCREPOSIX_H", 1)

# =============================================================================
# iconv const detection
# =============================================================================
if cc.has_header("iconv.h"):
    cdata.set("HAVE_ICONV", 1)
    # Check if iconv takes const char**
    iconv_const_code = """
#include <iconv.h>
size_t iconv(iconv_t, const char**, size_t*, char**, size_t*);
int main(){}
"""
    if cc.compiles(iconv_const_code):
        # Set entire line for substitution
        cdata.set("#define ICONV_CONST @ICONV_CONST@", "#define ICONV_CONST const")
    else:
        cdata.set("#define ICONV_CONST @ICONV_CONST@", "#define ICONV_CONST")
else:
    cdata.set("#define ICONV_CONST @ICONV_CONST@", "#define ICONV_CONST")

# =============================================================================
# Compression library detection (via deps - assumes Bazel provides these)
# Note: In starconf, dependencies are provided via Bazel's deps attribute.
# These checks verify the headers are available from the deps.
# =============================================================================

# zlib
if cc.has_header("zlib.h"):
    cdata.set("HAVE_LIBZ", 1)

# bzip2 - disabled for this example (would need bzip2 library)
# if cc.has_header("bzlib.h"):
#     cdata.set("HAVE_LIBBZ2", 1)

# libb2 (BLAKE2) - disabled for this example (would need libb2 library)
# if cc.has_header("blake2.h"):
#     cdata.set("HAVE_LIBB2", 1)

# lz4
if cc.has_header("lz4.h"):
    cdata.set("HAVE_LIBLZ4", 1)

# zstd
if cc.has_header("zstd.h"):
    cdata.set("HAVE_LIBZSTD", 1)
    if cc.has_header_symbol("zstd.h", "ZSTD_compressStream"):
        cdata.set("HAVE_ZSTD_compressStream", 1)

# lzma
if cc.has_header("lzma.h"):
    cdata.set("HAVE_LIBLZMA", 1)
    if cc.has_header_symbol("lzma.h", "lzma_stream_encoder_mt"):
        cdata.set("HAVE_LZMA_STREAM_ENCODER_MT", 1)

# lzo2
if cc.has_header("lzo/lzo1x.h"):
    cdata.set("HAVE_LIBLZO2", 1)
    cdata.set("HAVE_LZO_LZO1X_H", 1)
elif cc.has_header("lzo1x.h"):
    cdata.set("HAVE_LIBLZO2", 1)
    cdata.set("HAVE_LZO1X_H", 1)

# =============================================================================
# OpenSSL/crypto detection
# =============================================================================
if cc.has_header("openssl/evp.h"):
    cdata.set("HAVE_LIBCRYPTO", 1)

    # Check which algorithms are NOT disabled
    crypto_algos = ["md5", "rmd160", "sha1", "sha256", "sha384", "sha512"]
    for algo in crypto_algos:
        if not cc.has_header_symbol("openssl/ssl.h", "OPENSSL_NO_" + algo.upper()):
            cdata.set("ARCHIVE_CRYPTO_{}_OPENSSL".format(algo.upper()), 1)

    if cc.has_header_symbol("openssl/evp.h", "PKCS5_PBKDF2_HMAC_SHA1"):
        cdata.set("HAVE_PKCS5_PBKDF2_HMAC_SHA1", 1)

# =============================================================================
# XML library detection
# =============================================================================
if cc.has_header("libxml/xmlreader.h"):
    cdata.set("HAVE_LIBXML2", 1)
elif cc.has_header("expat.h"):
    cdata.set("HAVE_LIBEXPAT", 1)

# =============================================================================
# ACL library detection
# =============================================================================
if cc.has_header("acl/libacl.h"):
    cdata.set("HAVE_LIBACL", 1)
    cdata.set("ARCHIVE_ACL_LIBACL", 1)

# xattr library
if cc.has_header("attr/xattr.h"):
    cdata.set("HAVE_LIBATTR", 1)

# =============================================================================
# Windows bcrypt
# =============================================================================
if host_machine.system() == "windows":
    if cc.has_header("bcrypt.h"):
        cdata.set("HAVE_BCRYPT_H", 1)
        # Windows bcrypt crypto support
        for algo in ["md5", "sha1", "sha256", "sha384", "sha512"]:
            cdata.set("ARCHIVE_CRYPTO_{}_WIN".format(algo.upper()), 1)

# =============================================================================
# Generate config.h
# =============================================================================
configure_file(
    input=config_in(),
    output=config_out(),
    configuration=cdata,
)
