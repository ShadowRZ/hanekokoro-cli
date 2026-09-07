package format

import (
	"fmt"

	"github.com/fatih/color"
)

var efmt = color.New(color.FgHiRed, color.Bold)
var wfmt = color.New(color.FgHiYellow, color.Bold)
var mfmt = color.New(color.FgHiMagenta, color.Bold)

func Logf(format string, a ...any) {
	mfmt.Print("> ")
	fmt.Printf(format, a...)
}

func Errorf(format string, a ...any) {
	efmt.Printf(format, a...)
}

func Warningf(format string, a ...any) {
	wfmt.Printf(format, a...)
}
