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

// Run a command, without capturing any outputs.
func RunCmd(name string, arg ...string) error {
	cmd := exec.Command(name, arg...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin

	err := cmd.Start()
	if err != nil {
		return err
	}

	return cmd.Wait()
}

// Run a command, capturing stdout.
func RunCmdCaptured(name string, arg ...string) (stdout []byte, err error) {
	cmd := exec.Command(name, arg...)

	return cmd.Output()
}
