"""Contains ANSI escape codes"""
# Colors
CL_YELLOW = '\u001b[33;1m'
CL_RED = '\u001b[31;1m'
CL_GREEN = '\u001b[32;1m'
CL_MAGENTA = '\u001b[35;1m'
CL_CYAN = '\u001b[36;1m'

CL_END = '\33[0m'

# Debug Funcs
def error(text: str):
    print("\r", CL_RED, "[ERROR] ", text, CL_END, sep="")
    exit(1)

def warn(text: str):
    print("\r", CL_YELLOW, "[WARNING] ", text, CL_END, sep="")

def info(text: str):
    print("\r", CL_CYAN, "[INFO] ", text, CL_END, sep="")

def time_info(text: str):
    print("\r", CL_MAGENTA, "[INFO] ", text, CL_END, sep="")

def success(text: str):
    print("\r", CL_GREEN, "[SUCCESS] ", text, CL_END, sep="")
    