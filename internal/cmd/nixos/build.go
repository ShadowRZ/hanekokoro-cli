package nixos

import (
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"

	"github.com/ShadowRZ/hanekokoro-cli/internal/format"
	"github.com/ShadowRZ/hanekokoro-cli/internal/pinning"
	"github.com/ShadowRZ/hanekokoro-cli/internal/utils"
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

func NixOSBuild() {
	root, err := utils.RootDir()
	if err != nil {
		format.Errorf("Failed to determine root directory: %w", err)
	}

	ty := pinning.CheckPinning(root)

	outPath, err := BuildConfiguration(root, ty)
	if err != nil && errors.Is(err, buildError) {
		return
	} else if err != nil {
		format.Errorf("%s", err)
	}

	RunDix(outPath)
}

// Run dix, comparing /run/current-system and new outPath, only if they're not the same path.
func RunDix(outPath string) error {
	oldPath, err := os.Readlink("/run/current-system")
	if err != nil {
		return err
	}

	if oldPath == outPath {
		return nil
	}

	dixExec, err := exec.LookPath("dix")
	if err != nil {
		return err
	}

	cmd := exec.Command(dixExec, oldPath, outPath)
	cmd.Stdout = os.Stdout
	cmd.Stdin = os.Stdin

	cmd.Start()
	cmd.Wait()

	return nil
}

var buildError = errors.New("Build failure")

func BuildConfiguration(root string, ty pinning.Pinning) (string, error) {
	switch ty {
	case pinning.Flake:
		hostname, err := os.Hostname()
		if err != nil {
			return "", fmt.Errorf("Failed to determine hostname: %w", err)
		}
		outPath, err := BuildFlake(
			root,
			fmt.Sprintf("nixosConfigurations.\"%s\".config.system.build.toplevel", hostname),
			true,
		)
		if err != nil {
			return "", fmt.Errorf("Failed to determine built configuration path: %w", err)
		}

		if outPath == "" {
			return "", buildError
		}

		return outPath, nil
	default:
		// TODO
		return "", fmt.Errorf("Not implemented!")
	}
}
