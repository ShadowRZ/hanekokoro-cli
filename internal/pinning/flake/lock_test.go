package flake_test

import (
	"testing"
	"testing/fstest"

	"github.com/ShadowRZ/hanekokoro-cli/internal/pinning/flake"
)

func TestFlakeLock(t *testing.T) {
	fsys := fstest.MapFS{
		"flake.lock": {
			Data: []byte(`
{
  "nodes": {
    "root": {
      "inputs": {
        "treefmt-nix": "treefmt-nix"
      }
    },
    "treefmt-nix": {
      "inputs": {
        "nixpkgs": []
      },
      "locked": {
        "lastModified": 1772660329,
        "narHash": "sha256-IjU1FxYqm+VDe5qIOxoW+pISBlGvVApRjiw/Y/ttJzY=",
        "owner": "numtide",
        "repo": "treefmt-nix",
        "rev": "3710e0e1218041bbad640352a0440114b1e10428",
        "type": "github"
      },
      "original": {
        "owner": "numtide",
        "repo": "treefmt-nix",
        "type": "github"
      }
    }
  },
  "root": "root",
  "version": 7
}
			`),
		},
	}

	lock, err := flake.ReadFlakeLock(fsys)
	if err != nil {
		t.Fatal(err)
	}
	got := lock.Version
	if got != 7 {
		t.Errorf("lock.version = %d, want 7", got)
	}
}
