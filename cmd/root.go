package cmd

import (
	"fmt"
	"os"

	"github.com/ShadowRZ/hanekokoro-cli/internal/utils"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var bold = color.New(color.Bold)
var boldCyan = color.New(color.FgCyan, color.Bold)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "hanekokoro",
	Short: "Hanekokoro CLI",
	Long:  "@ShadowRZ's Nix/NixOS/Nixpkgs helpers",
	// Uncomment the following line if your bare application
	// has an action associated with it:
	// Run: func(cmd *cobra.Command, args []string) { },
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

const (
	helpCommandName = "help"
)

func init() {
	// Make Help a global flag instead of local (overrides Cobra builtin behaviour)
	rootCmd.PersistentFlags().BoolP("help", "h", false, "Show help for command")

	// Customize UsageFunc
	rootCmd.SetUsageFunc(func(c *cobra.Command) error {
		w := c.OutOrStderr()

		bold.Fprint(w, "USAGE")
		boldCyan.Fprintf(w, "\n  %s", c.CommandPath())
		if c.Runnable() {
			fmt.Fprint(w, " [flags]")
		}
		if c.HasAvailableSubCommands() {
			fmt.Fprint(w, " [command]")
		}
		if len(c.Aliases) > 0 {
			bold.Fprintf(w, "\n\nALIASES\n")
			fmt.Fprintf(w, "  %s", c.NameAndAliases())
		}
		if c.HasExample() {
			bold.Fprintf(w, "\n\nEXAMPLES\n")
			fmt.Fprintf(w, "%s", c.Example)
		}
		if c.HasAvailableSubCommands() {
			cmds := c.Commands()
			if len(c.Groups()) == 0 {
				bold.Fprintf(w, "\n\nAVALIABLE COMMANDS")
				for _, subcmd := range cmds {
					if subcmd.IsAvailableCommand() || subcmd.Name() == helpCommandName {
						boldCyan.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
						fmt.Fprint(w, subcmd.Short)
					}
				}
			} else {
				for _, group := range c.Groups() {
					bold.Fprintf(w, "\n\n%s", group.Title)
					for _, subcmd := range cmds {
						if subcmd.GroupID == group.ID && (subcmd.IsAvailableCommand() || subcmd.Name() == helpCommandName) {
							boldCyan.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
							fmt.Fprint(w, subcmd.Short)
						}
					}
				}
				if !c.AllChildCommandsHaveGroup() {
					fmt.Fprintf(w, "\n\nADDITIONAL COMMANDS")
					for _, subcmd := range cmds {
						if subcmd.GroupID == "" && (subcmd.IsAvailableCommand() || subcmd.Name() == helpCommandName) {
							boldCyan.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
							fmt.Fprint(w, subcmd.Short)
						}
					}
				}
			}
		}
		if c.HasAvailableLocalFlags() {
			bold.Fprintf(w, "\n\nFLAGS\n")
			fmt.Fprint(w, utils.TrimRightSpace(c.LocalFlags().FlagUsages()))
		}
		if c.HasAvailableInheritedFlags() {
			bold.Fprintf(w, "\n\nGLOBAL FLAGS\n")
			fmt.Fprint(w, utils.TrimRightSpace(c.InheritedFlags().FlagUsages()))
		}
		if c.HasHelpSubCommands() {
			fmt.Fprintf(w, "\n\nADDITIONAL HELP TOPICS")
			for _, subcmd := range c.Commands() {
				if subcmd.IsAdditionalHelpTopicCommand() {
					boldCyan.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
					fmt.Fprint(w, subcmd.Short)
				}
			}
		}
		bold.Fprintf(w, "\n\nLEARN MORE\n")
		fmt.Fprintf(w, "  Use `%s <command> <subcommand> --help` for more information about a command.", c.Root().CommandPath())
		fmt.Fprintln(w)
		return nil
	})
}
