package flake

import (
	"fmt"
	"maps"
	"os"
	"slices"

	"github.com/ShadowRZ/hanekokoro-cli/internal/format"
	"github.com/ShadowRZ/hanekokoro-cli/internal/pinning/flake"
	"github.com/ShadowRZ/hanekokoro-cli/internal/utils"
	"github.com/fatih/color"
)

var bold = color.New(color.Bold)

func FlakeShow() {
	pwd, err := os.Getwd()
	if err != nil {
		format.Errorf("Failed to determine current directory: %s", err.Error())
		return
	}
	fsys := os.DirFS(pwd)
	description, err := flake.ReadFlakeDescription(fsys)
	if err != nil {
		format.Errorf("Failed to read flake description: %s", err.Error())
		return
	}
	lock, err := flake.ReadFlakeLock(fsys)
	if err != nil {
		format.Errorf("Failed to read flake lock: %s", err.Error())
		return
	}
	fmt.Printf("%s%s\n", bold.Sprint(utils.Rpad("Description:", 15)), description)
	delete(lock.Nodes, lock.Root)
	bold.Println("Inputs")
	for _, node := range slices.Sorted(maps.Keys(lock.Nodes)) {
		item := lock.Nodes[node]
		bold.Printf("* %s", node)
		fmt.Println()
		fmt.Printf(" %s %s", bold.Sprint("->"), item.Locked.Sprint())
		fmt.Println()
	}
}
