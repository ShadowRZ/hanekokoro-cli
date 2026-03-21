package format

import "github.com/fatih/color"

var efmt = color.New(color.FgHiRed, color.Bold)

func Errorf(format string, a ...interface{}) (n int, err error) {
	return efmt.Printf(format, a...)
}
