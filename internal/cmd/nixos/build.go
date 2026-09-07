package nixos

import (
	"errors"
	"fmt"
	"os"

	"github.com/ShadowRZ/hanekokoro-cli/internal/format"
	"github.com/ShadowRZ/hanekokoro-cli/internal/pinning"
	"github.com/ShadowRZ/hanekokoro-cli/internal/runnix"
	"github.com/ShadowRZ/hanekokoro-cli/internal/utils"
)

func NixOSBuild() {
	root, err := utils.RootDir()
	if err != nil {
		format.Errorf("Failed to determine root directory: %s", err)
	}

	ty := pinning.CheckPinning(root)

	outPath, err := BuildConfiguration(root, ty)
	if err != nil && errors.Is(err, buildError) {
		return
	} else if err != nil {
		format.Errorf("%s", err)
	}

	runnix.RunDix(outPath)
}

var buildError = errors.New("Build failure")

func BuildConfiguration(root string, ty pinning.Pinning) (string, error) {
	switch ty {
	case pinning.Flake:
		hostname, err := os.Hostname()
		if err != nil {
			return "", fmt.Errorf("Failed to determine hostname: %w", err)
		}
		outPath, err := runnix.BuildFlake(
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
