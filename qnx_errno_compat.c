/*
 * QNX errno compatibility shim
 * 
 * Provides __errno() function as a wrapper around QNX's __get_errno_ptr()
 * This is a dirty hack to make crates that expect __errno() work on QNX.
 */

// Forward declaration of QNX's actual errno function
extern int* __get_errno_ptr(void);

// Provide the missing __errno function that other crates expect
int* __errno(void) {
    return __get_errno_ptr();
}
