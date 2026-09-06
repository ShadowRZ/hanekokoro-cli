package cmd

import (
	"github.com/ShadowRZ/hanekokoro-cli/internal/cmd/nixos"
	"github.com/spf13/cobra"
)

// nixosCmd represents the nixos command
var nixosCmd = &cobra.Command{
	Use:   "nixos",
	Short: "NixOS related functions",
	Long: `NixOS related functions

Implements some functions that mostly built around, or with the idea of nixos-rebuild in mind.`,
}

// nixosBuildCmd represents the nixos build command
var nixosBuildCmd = &cobra.Command{
	Use:   "build",
	Short: "Build a NixOS configuration",
	Long: `Build a NixOS configuration.

How to build a NixOS configuration is automatically determined:

 * In a Git repository, if the root of the Git worktree contains flake.lock, flake configuration will be used.
 * Outside of a Git repository, if the working directory contains flake.lock, flake configuration will be used.

Other methods comming soon!`,
	Run: func(cmd *cobra.Command, args []string) {
		nixos.NixOSBuild()
	},
}

func init() {
	rootCmd.AddCommand(nixosCmd)
	nixosCmd.AddCommand(nixosBuildCmd)
	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// nixosCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// nixosCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
