package runnix

import (
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/ShadowRZ/hanekokoro-cli/internal/format"
)

func Build(
	path_ string,
	attr string,
	nom bool,
	arg ...string,
) (path string, err error) {
	var nixExec = "nix-build"
	if nom {
		nomExec, err := exec.LookPath("nom-build")
		if err != nil {
			format.Warningf("Failed to find nom-build, using nix-build instead")
		} else {
			nixExec = nomExec
		}
	}

	cmd := exec.Command(
		nixExec,
		append(
			[]string{
				path_,
				"--attr",
				attr,
			},
			arg...)...,
	)
	outPath, err := cmd.Output()

	return strings.TrimSpace(string(outPath)), err
}

func BuildFlake(
	path_ string,
	attr string,
	nom bool,
	arg ...string,
) (path string, err error) {
	var nixExec = "nix"
	if nom {
		nomExec, err := exec.LookPath("nom")
		if err != nil {
			format.Warningf("Failed to find nom, using nix instead")
		} else {
			nixExec = nomExec
		}
	}

	var finalArgs = []string{}

	// This indirection is required because nix-output-monitor may not accept --extra-experimental-features
	if !nom {
		finalArgs = append(
			finalArgs,
			"--extra-experimental-features",
			"nix-command flakes",
		)
	}

	finalArgs = append(
		finalArgs,
		"build",
		"--print-out-paths",
		fmt.Sprintf("%s#%s", path_, attr),
	)

	cmd := exec.Command(
		nixExec,
		append(
			finalArgs,
			arg...,
		)...,
	)
	cmd.Stderr = os.Stderr
	outPath, err := cmd.Output()

	return strings.TrimSpace(string(outPath)), err
}
