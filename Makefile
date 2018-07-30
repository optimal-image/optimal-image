
init:
	mkdir .githooks/pre-commit
	git config core.hooksPath .githooks
	chmod a+x .githooks/pre-commit
	rustup component add rustfmt-preview