package format

import "github.com/fatih/color"

var efmt = color.New(color.FgHiRed, color.Bold)
var wfmt = color.New(color.FgHiYellow, color.Bold)

func Errorf(format string, a ...any) (n int, err error) {
	return efmt.Printf(format, a...)
}

func Warningf(format string, a ...any) (n int, err error) {
	return wfmt.Printf(format, a...)
}
