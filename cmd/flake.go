package cmd

import (
	"github.com/ShadowRZ/hanekokoro-cli/internal/cmd/flake"
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
}

// flakeShowCmd represents the flake show command
var flakeShowCmd = &cobra.Command{
	Use:   "show",
	Short: "Show outputs",
	Long:  `Show all output attributes provided by the Nix project.`,
	Run: func(cmd *cobra.Command, args []string) {
		flake.FlakeShow()
	},
}

func init() {
	rootCmd.AddCommand(flakeCmd)
	flakeCmd.AddCommand(flakeShowCmd)
}
