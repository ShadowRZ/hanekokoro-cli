package flake

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"os"
	"os/exec"
	"strings"

	"github.com/savioxavier/termlink"
)

// Represents flake.lock content.
type FlakeLock struct {
	Root    string               `json:"root"`
	Nodes   map[string]FlakeNode `json:"nodes"`
	Version int                  `json:"version"`
}

type FlakeNode struct {
	Inputs map[string]Path `json:"inputs"`
	Locked Locked          `json:"locked"`
}

type Path []string

func (path *Path) UnmarshalJSON(b []byte) error {
	if len(b) == 0 {
		return fmt.Errorf("no bytes to unmarshal")
	}

	if b[0] == '[' {
		var res []string
		err := json.Unmarshal(b, &res)
		if err != nil {
			return err
		}
		(*path) = Path(res)
	} else {
		var res string
		err := json.Unmarshal(b, &res)
		if err != nil {
			return err
		}
		(*path) = Path([]string{res})
	}

	return nil
}

type Locked struct {
	LastModified int    `json:"lastModified"`
	NarHash      string `json:"narHash"`
	Owner        string `json:"owner"`
	Repo         string `json:"repo"`
	Rev          string `json:"rev"`
	Type         string `json:"type"`
	Url          string `json:"url"`
	Dir          string `json:"dir"`
}

func (locked *Locked) Print() {
	locked.Fprint(os.Stdout)
}

func (locked *Locked) Fprint(w io.Writer) {
	linkable := termlink.SupportsHyperlinks()
	switch locked.Type {
	case "tarball":
		fmt.Fprint(w, locked.Url)
	case "github":
		var outfmt strings.Builder
		fmt.Fprintf(&outfmt, "%s:%s/%s/%s", locked.Type, locked.Owner, locked.Repo, locked.Rev)
		if locked.Dir != "" {
			fmt.Fprintf(&outfmt, "?dir=%s", locked.Dir)
		}
		if linkable {
			fmt.Fprint(w, termlink.Link(outfmt.String(), fmt.Sprintf("https://github.com/%s/%s/commit/%s", locked.Owner, locked.Repo, locked.Rev)))
		}
	case "gitlab":
		var outfmt strings.Builder
		fmt.Fprintf(&outfmt, "%s:%s/%s/%s", locked.Type, locked.Owner, locked.Repo, locked.Rev)
		if locked.Dir != "" {
			fmt.Fprintf(&outfmt, "?dir=%s", locked.Dir)
		}
		if linkable {
			fmt.Fprint(w, termlink.Link(outfmt.String(), fmt.Sprintf("https://gitlab.com/%s/%s/-/commit/%s", locked.Owner, locked.Repo, locked.Rev)))
		}
	default:
		fmt.Fprintf(w, "%s:%s/%s/%s", locked.Type, locked.Owner, locked.Repo, locked.Rev)
	}
}

func (locked *Locked) Sprint() string {
	var ret strings.Builder
	locked.Fprint(&ret)

	return ret.String()
}

type Original struct {
	Owner string `json:"owner"`
	Repo  string `json:"repo"`
	Ref   string `json:"ref"`
	Type  string `json:"type"`
	Url   string `json:"url"`
	Dir   string `json:"dir"`
}

func parseFlakeLock(text []byte) (FlakeLock, error) {
	var res FlakeLock
	err := json.Unmarshal(text, &res)

	return res, err
}

func ReadFlakeLock(fsys fs.FS) (FlakeLock, error) {
	bytes, err := fs.ReadFile(fsys, "flake.lock")
	if err != nil {
		return FlakeLock{}, err
	}
	parsed, err := parseFlakeLock(bytes)
	if err != nil {
		return FlakeLock{}, err
	}

	return parsed, nil
}

var ErrNoNixInstantiate = errors.New("failed to find nix-instantiate binary")

func ReadFlakeDescription(fsys fs.FS) (string, error) {
	bytes, err := fs.ReadFile(fsys, "flake.nix")
	if err != nil {
		return "", err
	}

	cmd := exec.Command("nix-instantiate", "--json", "--eval", "--expr", fmt.Sprintf("let flake = %s; in (flake.description or null)", bytes))
	stdout, err := cmd.Output()
	if err != nil {
		switch {
		case errors.Is(err, exec.ErrNotFound):
			{
				return "", ErrNoNixInstantiate
			}
		case errors.Is(err, exec.ErrDot):
			{
				return "", ErrNoNixInstantiate
			}
		}
		return "", err
	}

	var result string
	err = json.Unmarshal(stdout, &result)

	if err != nil {
		return "", err
	}

	return string(result), nil
}
