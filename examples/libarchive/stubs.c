// Stub implementations for disabled features

// RAR5 support is disabled (requires libb2 for blake2)
int archive_read_support_format_rar5(void *a) {
    // Return ARCHIVE_WARN to indicate unsupported format
    return -20;  // ARCHIVE_WARN
}
