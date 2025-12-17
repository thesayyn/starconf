// Simple test program that uses libarchive
#include <stdio.h>
#include <archive.h>
#include <archive_entry.h>

int main(int argc, char **argv) {
    printf("libarchive version: %s\n", archive_version_string());
    printf("libarchive details: %s\n", archive_version_details());

    // Create a simple archive object to verify everything links
    struct archive *a = archive_read_new();
    if (a == NULL) {
        fprintf(stderr, "Failed to create archive object\n");
        return 1;
    }

    // Enable common formats and filters
    archive_read_support_format_all(a);
    archive_read_support_filter_all(a);

    printf("Successfully created archive object with all formats and filters\n");

    archive_read_free(a);

    return 0;
}
