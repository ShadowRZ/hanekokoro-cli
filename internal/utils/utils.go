package utils

import (
	"fmt"
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
