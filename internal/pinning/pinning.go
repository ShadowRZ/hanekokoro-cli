package pinning

type Pinning int

const (
	// Pinned locks using Nix Flakes
	Flake Pinning = iota
	// Pinned locks using npins
	Npins
	// Unknown types of pinning
	Unknown
)
