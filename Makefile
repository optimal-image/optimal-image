
init:
	git config core.hooksPath .githooks
	chmod a+x .githooks/pre-commit
	rustup component add rustfmt-preview
