package utils

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
	"unicode"
)

// rpad adds padding to the right of a string.
func Rpad(s string, padding int) string {
	formattedString := fmt.Sprintf("%%-%ds", padding)
	return fmt.Sprintf(formattedString, s)
}

func TrimRightSpace(s string) string {
	return strings.TrimRightFunc(s, unicode.IsSpace)
}

func RootDir() (string, error) {
	cmd := exec.Command("git", "rev-parse", "--show-toplevel")
	pwd, err := cmd.Output()
	if err != nil {
		// Not a Git worktree.
		return os.Getwd()
	}

	return strings.TrimSpace(string(pwd)), nil
}
