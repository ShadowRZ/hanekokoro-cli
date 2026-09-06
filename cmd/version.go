package cmd

import (
	"fmt"

	"github.com/ShadowRZ/hanekokoro-cli/internal/build"
	"github.com/spf13/cobra"
)

// versionCmd represents the version command
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Show version for Hanekokoro CLI",
	Long:  "Show version for Hanekokoro CLI",
	Run: func(cmd *cobra.Command, args []string) {
		boldCyan.Printf("Hanekokoro CLI %s\n", build.Version)
		fmt.Printf("https://github.com/ShadowRZ/hanekokoro-cli/releases/tag/v%s\n", build.Version)
	},
}

func init() {
	rootCmd.AddCommand(versionCmd)
}
