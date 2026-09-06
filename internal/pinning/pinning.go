package pinning

import (
	"os"
	"path"
)

type Pinning int

const (
	// Pinned locks using Nix Flakes
	Flake Pinning = iota
	// Pinned locks using npins
	Npins
	// Unknown types of pinning
	Unknown
)

func CheckPinning(root string) Pinning {
	flakeLock := path.Join(root, "flake.lock")

	if _, err := os.Stat(flakeLock); err == nil {
		return Flake
	}

	return Unknown
}
