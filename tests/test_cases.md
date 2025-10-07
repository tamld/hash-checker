# Test Cases

*   **Test Case 1: Hash a text file**
    *   **Description:** Verify that the tool correctly hashes a simple text file.
    *   **Steps:**
        1.  Create a text file with known content.
        2.  Run the tool to hash the file.
        3.  Compare the generated hash with the expected hash.

*   **Test Case 2: Hash a binary file**
    *   **Description:** Verify that the tool correctly hashes a binary file (e.g., an image).
    *   **Steps:**
        1.  Create a binary file.
        2.  Run the tool to hash the file.
        3.  Compare the generated hash with the expected hash.

*   **Test Case 3: Compare a matching hash**
    *   **Description:** Verify that the tool correctly identifies a matching hash.
    *   **Steps:**
        1.  Hash a file.
        2.  Run the tool to compare the file with its correct hash.
        3.  Verify that the tool reports a match.

*   **Test Case 4: Compare a non-matching hash**
    *   **Description:** Verify that the tool correctly identifies a non-matching hash.
    *   **Steps:**
        1.  Hash a file.
        2.  Run the tool to compare the file with an incorrect hash.
        3.  Verify that the tool reports a mismatch.

*   **Test Case 5: Invalid file path**
    *   **Description:** Verify that the tool handles an invalid file path gracefully.
    *   **Steps:**
        1.  Run the tool with a path to a non-existent file.
        2.  Verify that the tool displays an appropriate error message.
