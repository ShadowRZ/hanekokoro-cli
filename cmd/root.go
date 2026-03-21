package cmd

import (
	"fmt"
	"os"

	"github.com/ShadowRZ/hanekokoro-cli/internal/utils"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var bold = color.New(color.Bold)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "hanekokoro-cli",
	Short: "Hanekokoro CLI",
	Long:  "Hanekokoro CLI\nKinda kitchen sink for something I might wrote?",
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
	rootCmd.SetUsageFunc(func(c *cobra.Command) error {
		w := c.OutOrStderr()

		bold.Fprint(w, "USAGE")
		bold.Fprintf(w, "\n  %s", c.CommandPath())
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
						bold.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
						fmt.Fprint(w, subcmd.Short)
					}
				}
			} else {
				for _, group := range c.Groups() {
					bold.Fprintf(w, "\n\n%s", group.Title)
					for _, subcmd := range cmds {
						if subcmd.GroupID == group.ID && (subcmd.IsAvailableCommand() || subcmd.Name() == helpCommandName) {
							bold.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
							fmt.Fprint(w, subcmd.Short)
						}
					}
				}
				if !c.AllChildCommandsHaveGroup() {
					fmt.Fprintf(w, "\n\nADDITIONAL COMMANDS")
					for _, subcmd := range cmds {
						if subcmd.GroupID == "" && (subcmd.IsAvailableCommand() || subcmd.Name() == helpCommandName) {
							bold.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
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
					bold.Fprintf(w, "\n  %s ", utils.Rpad(subcmd.Name(), subcmd.NamePadding()))
					fmt.Fprint(w, subcmd.Short)
				}
			}
		}
		if c.HasAvailableSubCommands() {
			fmt.Fprintf(w, "\n\nUse \"%s [command] --help\" for more information about a command.", c.CommandPath())
		}
		fmt.Fprintln(w)
		return nil
	})
}
