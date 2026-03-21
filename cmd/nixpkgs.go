package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// nixpkgsCmd represents the nixpkgs command
var nixpkgsCmd = &cobra.Command{
	Use:   "nixpkgs",
	Short: "Various commands that deal with Nixpkgs",
	Long:  `Various commands that deal with Nixpkgs`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("nixpkgs called")
	},
}

func init() {
	rootCmd.AddCommand(nixpkgsCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// nixpkgsCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// nixpkgsCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
