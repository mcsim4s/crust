.PHONY: release/minor release/major

release/minor:
	cargo release minor --no-publish --execute

release/major:
	cargo release major --no-publish --execute