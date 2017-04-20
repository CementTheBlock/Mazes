INDENT = sed 's/^/\t/g'
SGR_BOLD = printf '\e[1m'
SGR_RESET = printf '\e[0m'

.PHONY: all check build clean help

all: build check

check: mazes
	./mazes

build: mazes

mazes: src/mazes.rs
	rustfmt src/mazes.rs
	rustc src/mazes.rs

clean:
	-@echo "Actions that will be taken while cleaning:"
	-@$(SGR_BOLD)
	-@git clean -n | $(INDENT)
	-@git clean -Xn | $(INDENT)
	-@$(SGR_RESET)
	-@printf "Press enter to continue or <ctrl-c> to quit..."
	@read
	-@$(SGR_BOLD); { git clean -f; git clean -Xf; } | $(INDENT); $(SGR_RESET)

help:
	-@printf "Syntax: make ACTION\n"
	-@printf "where ACTION is one of the following (default: \e[1mall\e[0m):\n"
	-@printf "  \e[1mhelp\e[0m  - show this help message\n"
	-@printf "  \e[1mall\e[0m   - same as \e[1mmake build check\e[0m\n"
	-@printf "  \e[1mcheck\e[0m - run the tests\n"
	-@printf "  \e[1mbuild\e[0m - build everything\n"
	-@printf "  \e[1mmazes\e[0m - build the \e[1mmazes\e[0m executable\n"
	-@printf "  \e[1mclean\e[0m - clean up any untracked files\n"
