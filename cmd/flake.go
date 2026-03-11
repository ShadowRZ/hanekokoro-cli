package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// flakeCmd represents the flake command
var flakeCmd = &cobra.Command{
	Use:   "flake",
	Short: "Manage Nix Flakes (or other types of input pinning)",
	Long: `Manage Nix Flakes (or other types of input pinning)

These commands support the following mechanisms for input pinning:
 * Nix Flakes (flake.nix, flake.lock)

Other types comming soon!`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("flake called")
	},
}

// flakeCmd represents the flake show command
var flakeShowCmd = &cobra.Command{
	Use:   "show",
	Short: "Inspect Nix Flakes (or other types of input pinning)",
	Long: `Inspect Nix Flakes (or other types of input pinning)

These commands support the following mechanisms for input pinning:
 * Nix Flakes (flake.nix, flake.lock)

Other types comming soon!`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("flake called")
	},
}

func init() {
	rootCmd.AddCommand(flakeCmd)
	flakeCmd.AddCommand(flakeShowCmd)
}
